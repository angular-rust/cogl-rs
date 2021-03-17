extern crate cogl_sys;
extern crate shell_words;
extern crate tempfile;
use cogl_sys::*;
use std::env;
use std::error::Error;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["cogl-1.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Compiler, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Compiler { args })
    }

    pub fn define<'a, V: Into<Option<&'a str>>>(&mut self, var: &str, val: V) {
        let arg = match val.into() {
            None => format!("-D{}", var),
            Some(val) => format!("-D{}={}", var, val),
        };
        self.args.push(arg);
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let mut cmd = Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
    /// Number of tests that failed to compile.
    failed_to_compile: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn record_failed_to_compile(&mut self) {
        self.failed += 1;
        self.failed_to_compile += 1;
    }
    fn summary(&self) -> String {
        format!(
            "{} passed; {} failed (compilation errors: {})",
            self.passed, self.failed, self.failed_to_compile
        )
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let tmpdir = Builder::new()
        .prefix("abi")
        .tempdir()
        .expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!(
        "1",
        get_c_value(tmpdir.path(), &cc, "1").expect("C constant"),
        "failed to obtain correct constant value for 1"
    );

    let mut results: Results = Default::default();
    for (i, &(name, rust_value)) in RUST_CONSTANTS.iter().enumerate() {
        match get_c_value(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            }
            Ok(ref c_value) => {
                if rust_value == c_value {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!(
                        "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                        name, rust_value, c_value
                    );
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("constants ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let tmpdir = Builder::new()
        .prefix("abi")
        .tempdir()
        .expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!(
        Layout {
            size: 1,
            alignment: 1
        },
        get_c_layout(tmpdir.path(), &cc, "char").expect("C layout"),
        "failed to obtain correct layout for char type"
    );

    let mut results: Results = Default::default();
    for (i, &(name, rust_layout)) in RUST_LAYOUTS.iter().enumerate() {
        match get_c_layout(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            }
            Ok(c_layout) => {
                if rust_layout == c_layout {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!(
                        "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                        name, rust_layout, &c_layout
                    );
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("layout    ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

fn get_c_layout(dir: &Path, cc: &Compiler, name: &str) -> Result<Layout, Box<dyn Error>> {
    let exe = dir.join("layout");
    let mut cc = cc.clone();
    cc.define("ABI_TYPE_NAME", name);
    cc.compile(Path::new("tests/layout.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    let stdout = str::from_utf8(&output.stdout)?;
    let mut words = stdout.trim().split_whitespace();
    let size = words.next().unwrap().parse().unwrap();
    let alignment = words.next().unwrap().parse().unwrap();
    Ok(Layout { size, alignment })
}

fn get_c_value(dir: &Path, cc: &Compiler, name: &str) -> Result<String, Box<dyn Error>> {
    let exe = dir.join("constant");
    let mut cc = cc.clone();
    cc.define("ABI_CONSTANT_NAME", name);
    cc.compile(Path::new("tests/constant.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    let output = str::from_utf8(&output.stdout)?.trim();
    if !output.starts_with("###gir test###") || !output.ends_with("###gir test###") {
        return Err(format!(
            "command {:?} return invalid output, {:?}",
            &abi_cmd, &output
        )
        .into());
    }

    Ok(String::from(&output[14..(output.len() - 14)]))
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "CoglAngle",
        Layout {
            size: size_of::<CoglAngle>(),
            alignment: align_of::<CoglAngle>(),
        },
    ),
    (
        "CoglAttributeType",
        Layout {
            size: size_of::<CoglAttributeType>(),
            alignment: align_of::<CoglAttributeType>(),
        },
    ),
    (
        "CoglBitmapError",
        Layout {
            size: size_of::<CoglBitmapError>(),
            alignment: align_of::<CoglBitmapError>(),
        },
    ),
    (
        "CoglBlendStringError",
        Layout {
            size: size_of::<CoglBlendStringError>(),
            alignment: align_of::<CoglBlendStringError>(),
        },
    ),
    (
        "CoglBool",
        Layout {
            size: size_of::<CoglBool>(),
            alignment: align_of::<CoglBool>(),
        },
    ),
    (
        "CoglBufferBit",
        Layout {
            size: size_of::<CoglBufferBit>(),
            alignment: align_of::<CoglBufferBit>(),
        },
    ),
    (
        "CoglBufferTarget",
        Layout {
            size: size_of::<CoglBufferTarget>(),
            alignment: align_of::<CoglBufferTarget>(),
        },
    ),
    (
        "CoglColor",
        Layout {
            size: size_of::<CoglColor>(),
            alignment: align_of::<CoglColor>(),
        },
    ),
    (
        "CoglColorMask",
        Layout {
            size: size_of::<CoglColorMask>(),
            alignment: align_of::<CoglColorMask>(),
        },
    ),
    (
        "CoglDepthTestFunction",
        Layout {
            size: size_of::<CoglDepthTestFunction>(),
            alignment: align_of::<CoglDepthTestFunction>(),
        },
    ),
    (
        "CoglFeatureFlags",
        Layout {
            size: size_of::<CoglFeatureFlags>(),
            alignment: align_of::<CoglFeatureFlags>(),
        },
    ),
    (
        "CoglFilterReturn",
        Layout {
            size: size_of::<CoglFilterReturn>(),
            alignment: align_of::<CoglFilterReturn>(),
        },
    ),
    (
        "CoglFogMode",
        Layout {
            size: size_of::<CoglFogMode>(),
            alignment: align_of::<CoglFogMode>(),
        },
    ),
    (
        "CoglHandle",
        Layout {
            size: size_of::<CoglHandle>(),
            alignment: align_of::<CoglHandle>(),
        },
    ),
    (
        "CoglIndicesType",
        Layout {
            size: size_of::<CoglIndicesType>(),
            alignment: align_of::<CoglIndicesType>(),
        },
    ),
    (
        "CoglMaterialAlphaFunc",
        Layout {
            size: size_of::<CoglMaterialAlphaFunc>(),
            alignment: align_of::<CoglMaterialAlphaFunc>(),
        },
    ),
    (
        "CoglMaterialFilter",
        Layout {
            size: size_of::<CoglMaterialFilter>(),
            alignment: align_of::<CoglMaterialFilter>(),
        },
    ),
    (
        "CoglMaterialLayerType",
        Layout {
            size: size_of::<CoglMaterialLayerType>(),
            alignment: align_of::<CoglMaterialLayerType>(),
        },
    ),
    (
        "CoglMaterialWrapMode",
        Layout {
            size: size_of::<CoglMaterialWrapMode>(),
            alignment: align_of::<CoglMaterialWrapMode>(),
        },
    ),
    (
        "CoglMatrix",
        Layout {
            size: size_of::<CoglMatrix>(),
            alignment: align_of::<CoglMatrix>(),
        },
    ),
    (
        "CoglPixelFormat",
        Layout {
            size: size_of::<CoglPixelFormat>(),
            alignment: align_of::<CoglPixelFormat>(),
        },
    ),
    (
        "CoglReadPixelsFlags",
        Layout {
            size: size_of::<CoglReadPixelsFlags>(),
            alignment: align_of::<CoglReadPixelsFlags>(),
        },
    ),
    (
        "CoglRendererError",
        Layout {
            size: size_of::<CoglRendererError>(),
            alignment: align_of::<CoglRendererError>(),
        },
    ),
    (
        "CoglShaderType",
        Layout {
            size: size_of::<CoglShaderType>(),
            alignment: align_of::<CoglShaderType>(),
        },
    ),
    (
        "CoglStereoMode",
        Layout {
            size: size_of::<CoglStereoMode>(),
            alignment: align_of::<CoglStereoMode>(),
        },
    ),
    (
        "CoglSystemError",
        Layout {
            size: size_of::<CoglSystemError>(),
            alignment: align_of::<CoglSystemError>(),
        },
    ),
    (
        "CoglTextureComponents",
        Layout {
            size: size_of::<CoglTextureComponents>(),
            alignment: align_of::<CoglTextureComponents>(),
        },
    ),
    (
        "CoglTextureError",
        Layout {
            size: size_of::<CoglTextureError>(),
            alignment: align_of::<CoglTextureError>(),
        },
    ),
    (
        "CoglTextureFlags",
        Layout {
            size: size_of::<CoglTextureFlags>(),
            alignment: align_of::<CoglTextureFlags>(),
        },
    ),
    (
        "CoglTextureType",
        Layout {
            size: size_of::<CoglTextureType>(),
            alignment: align_of::<CoglTextureType>(),
        },
    ),
    (
        "CoglTextureVertex",
        Layout {
            size: size_of::<CoglTextureVertex>(),
            alignment: align_of::<CoglTextureVertex>(),
        },
    ),
    (
        "CoglVerticesMode",
        Layout {
            size: size_of::<CoglVerticesMode>(),
            alignment: align_of::<CoglVerticesMode>(),
        },
    ),
    (
        "CoglWinding",
        Layout {
            size: size_of::<CoglWinding>(),
            alignment: align_of::<CoglWinding>(),
        },
    ),
    (
        "CoglWinsysFeature",
        Layout {
            size: size_of::<CoglWinsysFeature>(),
            alignment: align_of::<CoglWinsysFeature>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("COGL_AFIRST_BIT", "64"),
    ("(gint) COGL_ATTRIBUTE_TYPE_BYTE", "5120"),
    ("(gint) COGL_ATTRIBUTE_TYPE_FLOAT", "5126"),
    ("(gint) COGL_ATTRIBUTE_TYPE_SHORT", "5122"),
    ("(gint) COGL_ATTRIBUTE_TYPE_UNSIGNED_BYTE", "5121"),
    ("(gint) COGL_ATTRIBUTE_TYPE_UNSIGNED_SHORT", "5123"),
    ("COGL_A_BIT", "16"),
    ("COGL_BGR_BIT", "32"),
    ("(gint) COGL_BITMAP_ERROR_CORRUPT_IMAGE", "2"),
    ("(gint) COGL_BITMAP_ERROR_FAILED", "0"),
    ("(gint) COGL_BITMAP_ERROR_UNKNOWN_TYPE", "1"),
    ("(gint) COGL_BLEND_STRING_ERROR_ARGUMENT_PARSE_ERROR", "1"),
    ("(gint) COGL_BLEND_STRING_ERROR_GPU_UNSUPPORTED_ERROR", "3"),
    ("(gint) COGL_BLEND_STRING_ERROR_INVALID_ERROR", "2"),
    ("(gint) COGL_BLEND_STRING_ERROR_PARSE_ERROR", "0"),
    ("(guint) COGL_BUFFER_BIT_COLOR", "1"),
    ("(guint) COGL_BUFFER_BIT_DEPTH", "2"),
    ("(guint) COGL_BUFFER_BIT_STENCIL", "4"),
    ("(guint) COGL_COLOR_MASK_ALL", "15"),
    ("(guint) COGL_COLOR_MASK_ALPHA", "8"),
    ("(guint) COGL_COLOR_MASK_BLUE", "4"),
    ("(guint) COGL_COLOR_MASK_GREEN", "2"),
    ("(guint) COGL_COLOR_MASK_NONE", "0"),
    ("(guint) COGL_COLOR_MASK_RED", "1"),
    ("COGL_DEPTH_BIT", "256"),
    ("(gint) COGL_DEPTH_TEST_FUNCTION_ALWAYS", "519"),
    ("(gint) COGL_DEPTH_TEST_FUNCTION_EQUAL", "514"),
    ("(gint) COGL_DEPTH_TEST_FUNCTION_GEQUAL", "518"),
    ("(gint) COGL_DEPTH_TEST_FUNCTION_GREATER", "516"),
    ("(gint) COGL_DEPTH_TEST_FUNCTION_LEQUAL", "515"),
    ("(gint) COGL_DEPTH_TEST_FUNCTION_LESS", "513"),
    ("(gint) COGL_DEPTH_TEST_FUNCTION_NEVER", "512"),
    ("(gint) COGL_DEPTH_TEST_FUNCTION_NOTEQUAL", "517"),
    ("(guint) COGL_FEATURE_DEPTH_RANGE", "16384"),
    ("(guint) COGL_FEATURE_DEPTH_TEXTURE", "16777216"),
    ("(guint) COGL_FEATURE_FOUR_CLIP_PLANES", "512"),
    ("(guint) COGL_FEATURE_MAP_BUFFER_FOR_READ", "2097152"),
    ("(guint) COGL_FEATURE_MAP_BUFFER_FOR_WRITE", "4194304"),
    ("(guint) COGL_FEATURE_OFFSCREEN", "64"),
    ("(guint) COGL_FEATURE_OFFSCREEN_BLIT", "256"),
    ("(guint) COGL_FEATURE_OFFSCREEN_MULTISAMPLE", "128"),
    ("(guint) COGL_FEATURE_ONSCREEN_MULTIPLE", "8388608"),
    ("(guint) COGL_FEATURE_PBOS", "4096"),
    ("(guint) COGL_FEATURE_POINT_SPRITE", "262144"),
    ("(guint) COGL_FEATURE_SHADERS_ARBFP", "1048576"),
    ("(guint) COGL_FEATURE_SHADERS_GLSL", "32"),
    ("(guint) COGL_FEATURE_STENCIL_BUFFER", "1024"),
    ("(guint) COGL_FEATURE_TEXTURE_3D", "524288"),
    ("(guint) COGL_FEATURE_TEXTURE_NPOT", "4"),
    ("(guint) COGL_FEATURE_TEXTURE_NPOT_BASIC", "32768"),
    ("(guint) COGL_FEATURE_TEXTURE_NPOT_MIPMAP", "65536"),
    ("(guint) COGL_FEATURE_TEXTURE_NPOT_REPEAT", "131072"),
    ("(guint) COGL_FEATURE_TEXTURE_READ_PIXELS", "16"),
    ("(guint) COGL_FEATURE_TEXTURE_RECTANGLE", "2"),
    ("(guint) COGL_FEATURE_TEXTURE_YUV", "8"),
    ("(guint) COGL_FEATURE_UNSIGNED_INT_INDICES", "8192"),
    ("(guint) COGL_FEATURE_VBOS", "2048"),
    ("(gint) COGL_FILTER_CONTINUE", "0"),
    ("(gint) COGL_FILTER_REMOVE", "1"),
    ("COGL_FIXED_0_5", "32768"),
    ("COGL_FIXED_1", "1"),
    ("COGL_FIXED_2_PI", "411775"),
    ("COGL_FIXED_BITS", "32"),
    ("COGL_FIXED_EPSILON", "1"),
    ("COGL_FIXED_MAX", "2147483647"),
    ("COGL_FIXED_PI", "205887"),
    ("COGL_FIXED_PI_2", "102944"),
    ("COGL_FIXED_PI_4", "51472"),
    ("COGL_FIXED_Q", "-16"),
    ("(gint) COGL_FOG_MODE_EXPONENTIAL", "1"),
    ("(gint) COGL_FOG_MODE_EXPONENTIAL_SQUARED", "2"),
    ("(gint) COGL_FOG_MODE_LINEAR", "0"),
    ("(gint) COGL_INDICES_TYPE_UNSIGNED_BYTE", "0"),
    ("(gint) COGL_INDICES_TYPE_UNSIGNED_INT", "2"),
    ("(gint) COGL_INDICES_TYPE_UNSIGNED_SHORT", "1"),
    ("(gint) COGL_MATERIAL_ALPHA_FUNC_ALWAYS", "519"),
    ("(gint) COGL_MATERIAL_ALPHA_FUNC_EQUAL", "514"),
    ("(gint) COGL_MATERIAL_ALPHA_FUNC_GEQUAL", "518"),
    ("(gint) COGL_MATERIAL_ALPHA_FUNC_GREATER", "516"),
    ("(gint) COGL_MATERIAL_ALPHA_FUNC_LEQUAL", "515"),
    ("(gint) COGL_MATERIAL_ALPHA_FUNC_LESS", "513"),
    ("(gint) COGL_MATERIAL_ALPHA_FUNC_NEVER", "512"),
    ("(gint) COGL_MATERIAL_ALPHA_FUNC_NOTEQUAL", "517"),
    ("(gint) COGL_MATERIAL_FILTER_LINEAR", "9729"),
    ("(gint) COGL_MATERIAL_FILTER_LINEAR_MIPMAP_LINEAR", "9987"),
    ("(gint) COGL_MATERIAL_FILTER_LINEAR_MIPMAP_NEAREST", "9985"),
    ("(gint) COGL_MATERIAL_FILTER_NEAREST", "9728"),
    ("(gint) COGL_MATERIAL_FILTER_NEAREST_MIPMAP_LINEAR", "9986"),
    ("(gint) COGL_MATERIAL_FILTER_NEAREST_MIPMAP_NEAREST", "9984"),
    ("(gint) COGL_MATERIAL_LAYER_TYPE_TEXTURE", "0"),
    ("(gint) COGL_MATERIAL_WRAP_MODE_AUTOMATIC", "519"),
    ("(gint) COGL_MATERIAL_WRAP_MODE_CLAMP_TO_EDGE", "33071"),
    ("(gint) COGL_MATERIAL_WRAP_MODE_REPEAT", "10497"),
    ("(guint) COGL_OFFSCREEN_BUFFER", "4"),
    ("(gint) COGL_PIXEL_FORMAT_ABGR_2101010", "125"),
    ("(gint) COGL_PIXEL_FORMAT_ABGR_2101010_PRE", "253"),
    ("(gint) COGL_PIXEL_FORMAT_ABGR_8888", "115"),
    ("(gint) COGL_PIXEL_FORMAT_ABGR_8888_PRE", "243"),
    ("(gint) COGL_PIXEL_FORMAT_ANY", "0"),
    ("(gint) COGL_PIXEL_FORMAT_ARGB_2101010", "93"),
    ("(gint) COGL_PIXEL_FORMAT_ARGB_2101010_PRE", "221"),
    ("(gint) COGL_PIXEL_FORMAT_ARGB_8888", "83"),
    ("(gint) COGL_PIXEL_FORMAT_ARGB_8888_PRE", "211"),
    ("(gint) COGL_PIXEL_FORMAT_A_8", "17"),
    ("(gint) COGL_PIXEL_FORMAT_BGRA_1010102", "61"),
    ("(gint) COGL_PIXEL_FORMAT_BGRA_1010102_PRE", "189"),
    ("(gint) COGL_PIXEL_FORMAT_BGRA_8888", "51"),
    ("(gint) COGL_PIXEL_FORMAT_BGRA_8888_PRE", "179"),
    ("(gint) COGL_PIXEL_FORMAT_BGR_888", "34"),
    ("(gint) COGL_PIXEL_FORMAT_DEPTH_16", "265"),
    ("(gint) COGL_PIXEL_FORMAT_DEPTH_24_STENCIL_8", "771"),
    ("(gint) COGL_PIXEL_FORMAT_DEPTH_32", "259"),
    ("(gint) COGL_PIXEL_FORMAT_G_8", "8"),
    ("(gint) COGL_PIXEL_FORMAT_RGBA_1010102", "29"),
    ("(gint) COGL_PIXEL_FORMAT_RGBA_1010102_PRE", "157"),
    ("(gint) COGL_PIXEL_FORMAT_RGBA_4444", "21"),
    ("(gint) COGL_PIXEL_FORMAT_RGBA_4444_PRE", "149"),
    ("(gint) COGL_PIXEL_FORMAT_RGBA_5551", "22"),
    ("(gint) COGL_PIXEL_FORMAT_RGBA_5551_PRE", "150"),
    ("(gint) COGL_PIXEL_FORMAT_RGBA_8888", "19"),
    ("(gint) COGL_PIXEL_FORMAT_RGBA_8888_PRE", "147"),
    ("(gint) COGL_PIXEL_FORMAT_RGB_565", "4"),
    ("(gint) COGL_PIXEL_FORMAT_RGB_888", "2"),
    ("(gint) COGL_PIXEL_FORMAT_RG_88", "9"),
    ("(gint) COGL_PIXEL_FORMAT_YUV", "7"),
    ("COGL_PREMULT_BIT", "128"),
    ("COGL_RADIANS_TO_DEGREES", "3754936"),
    ("(guint) COGL_READ_PIXELS_COLOR_BUFFER", "1"),
    ("(gint) COGL_RENDERER_ERROR_BAD_CONSTRAINT", "1"),
    ("(gint) COGL_RENDERER_ERROR_XLIB_DISPLAY_OPEN", "0"),
    ("(gint) COGL_SHADER_TYPE_FRAGMENT", "1"),
    ("(gint) COGL_SHADER_TYPE_VERTEX", "0"),
    ("COGL_SQRTI_ARG_10_PERCENT", "5590"),
    ("COGL_SQRTI_ARG_5_PERCENT", "210"),
    ("COGL_SQRTI_ARG_MAX", "4194303"),
    ("COGL_STENCIL_BIT", "512"),
    ("(gint) COGL_STEREO_BOTH", "0"),
    ("(gint) COGL_STEREO_LEFT", "1"),
    ("(gint) COGL_STEREO_RIGHT", "2"),
    ("(gint) COGL_SYSTEM_ERROR_NO_MEMORY", "1"),
    ("(gint) COGL_SYSTEM_ERROR_UNSUPPORTED", "0"),
    ("(gint) COGL_TEXTURE_COMPONENTS_A", "1"),
    ("(gint) COGL_TEXTURE_COMPONENTS_DEPTH", "5"),
    ("(gint) COGL_TEXTURE_COMPONENTS_RG", "2"),
    ("(gint) COGL_TEXTURE_COMPONENTS_RGB", "3"),
    ("(gint) COGL_TEXTURE_COMPONENTS_RGBA", "4"),
    ("(gint) COGL_TEXTURE_ERROR_BAD_PARAMETER", "2"),
    ("(gint) COGL_TEXTURE_ERROR_FORMAT", "1"),
    ("(gint) COGL_TEXTURE_ERROR_SIZE", "0"),
    ("(gint) COGL_TEXTURE_ERROR_TYPE", "3"),
    ("COGL_TEXTURE_MAX_WASTE", "127"),
    ("(guint) COGL_TEXTURE_NONE", "0"),
    ("(guint) COGL_TEXTURE_NO_ATLAS", "4"),
    ("(guint) COGL_TEXTURE_NO_AUTO_MIPMAP", "1"),
    ("(guint) COGL_TEXTURE_NO_SLICING", "2"),
    ("(gint) COGL_TEXTURE_TYPE_2D", "0"),
    ("(gint) COGL_TEXTURE_TYPE_3D", "1"),
    ("(gint) COGL_TEXTURE_TYPE_RECTANGLE", "2"),
    ("(gint) COGL_VERTICES_MODE_LINES", "1"),
    ("(gint) COGL_VERTICES_MODE_LINE_LOOP", "2"),
    ("(gint) COGL_VERTICES_MODE_LINE_STRIP", "3"),
    ("(gint) COGL_VERTICES_MODE_POINTS", "0"),
    ("(gint) COGL_VERTICES_MODE_TRIANGLES", "4"),
    ("(gint) COGL_VERTICES_MODE_TRIANGLE_FAN", "6"),
    ("(gint) COGL_VERTICES_MODE_TRIANGLE_STRIP", "5"),
    ("(gint) COGL_WINDING_CLOCKWISE", "0"),
    ("(gint) COGL_WINDING_COUNTER_CLOCKWISE", "1"),
    ("(guint) COGL_WINDOW_BUFFER", "2"),
    ("(gint) COGL_WINSYS_FEATURE_BUFFER_AGE", "9"),
    ("(gint) COGL_WINSYS_FEATURE_MULTIPLE_ONSCREEN", "0"),
    ("(gint) COGL_WINSYS_FEATURE_N_FEATURES", "11"),
    ("(gint) COGL_WINSYS_FEATURE_SWAP_BUFFERS_EVENT", "5"),
    ("(gint) COGL_WINSYS_FEATURE_SWAP_REGION", "6"),
    ("(gint) COGL_WINSYS_FEATURE_SWAP_REGION_SYNCHRONIZED", "8"),
    ("(gint) COGL_WINSYS_FEATURE_SWAP_REGION_THROTTLE", "7"),
    ("(gint) COGL_WINSYS_FEATURE_SWAP_THROTTLE", "1"),
    ("(gint) COGL_WINSYS_FEATURE_SYNC_AND_COMPLETE_EVENT", "10"),
    ("(gint) COGL_WINSYS_FEATURE_TEXTURE_FROM_PIXMAP", "4"),
    ("(gint) COGL_WINSYS_FEATURE_VBLANK_COUNTER", "2"),
    ("(gint) COGL_WINSYS_FEATURE_VBLANK_WAIT", "3"),
];

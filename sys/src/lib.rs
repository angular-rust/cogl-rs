#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal
)]

extern crate glib_sys as glib;
extern crate gobject_sys as gobject;
extern crate libc;

mod manual;

pub use manual::*;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, time_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Aliases
pub type CoglAngle = i32;
pub type CoglBool = c_int;
pub type CoglHandle = *mut c_void;

// Enums
pub type CoglAttributeType = c_int;
pub const COGL_ATTRIBUTE_TYPE_BYTE: CoglAttributeType = 5120;
pub const COGL_ATTRIBUTE_TYPE_UNSIGNED_BYTE: CoglAttributeType = 5121;
pub const COGL_ATTRIBUTE_TYPE_SHORT: CoglAttributeType = 5122;
pub const COGL_ATTRIBUTE_TYPE_UNSIGNED_SHORT: CoglAttributeType = 5123;
pub const COGL_ATTRIBUTE_TYPE_FLOAT: CoglAttributeType = 5126;

pub type CoglBitmapError = c_int;
pub const COGL_BITMAP_ERROR_FAILED: CoglBitmapError = 0;
pub const COGL_BITMAP_ERROR_UNKNOWN_TYPE: CoglBitmapError = 1;
pub const COGL_BITMAP_ERROR_CORRUPT_IMAGE: CoglBitmapError = 2;

pub type CoglBlendStringError = c_int;
pub const COGL_BLEND_STRING_ERROR_PARSE_ERROR: CoglBlendStringError = 0;
pub const COGL_BLEND_STRING_ERROR_ARGUMENT_PARSE_ERROR: CoglBlendStringError = 1;
pub const COGL_BLEND_STRING_ERROR_INVALID_ERROR: CoglBlendStringError = 2;
pub const COGL_BLEND_STRING_ERROR_GPU_UNSUPPORTED_ERROR: CoglBlendStringError = 3;

pub type CoglDepthTestFunction = c_int;
pub const COGL_DEPTH_TEST_FUNCTION_NEVER: CoglDepthTestFunction = 512;
pub const COGL_DEPTH_TEST_FUNCTION_LESS: CoglDepthTestFunction = 513;
pub const COGL_DEPTH_TEST_FUNCTION_EQUAL: CoglDepthTestFunction = 514;
pub const COGL_DEPTH_TEST_FUNCTION_LEQUAL: CoglDepthTestFunction = 515;
pub const COGL_DEPTH_TEST_FUNCTION_GREATER: CoglDepthTestFunction = 516;
pub const COGL_DEPTH_TEST_FUNCTION_NOTEQUAL: CoglDepthTestFunction = 517;
pub const COGL_DEPTH_TEST_FUNCTION_GEQUAL: CoglDepthTestFunction = 518;
pub const COGL_DEPTH_TEST_FUNCTION_ALWAYS: CoglDepthTestFunction = 519;

pub type CoglFilterReturn = c_int;
pub const COGL_FILTER_CONTINUE: CoglFilterReturn = 0;
pub const COGL_FILTER_REMOVE: CoglFilterReturn = 1;

pub type CoglFogMode = c_int;
pub const COGL_FOG_MODE_LINEAR: CoglFogMode = 0;
pub const COGL_FOG_MODE_EXPONENTIAL: CoglFogMode = 1;
pub const COGL_FOG_MODE_EXPONENTIAL_SQUARED: CoglFogMode = 2;

pub type CoglIndicesType = c_int;
pub const COGL_INDICES_TYPE_UNSIGNED_BYTE: CoglIndicesType = 0;
pub const COGL_INDICES_TYPE_UNSIGNED_SHORT: CoglIndicesType = 1;
pub const COGL_INDICES_TYPE_UNSIGNED_INT: CoglIndicesType = 2;

pub type CoglMaterialAlphaFunc = c_int;
pub const COGL_MATERIAL_ALPHA_FUNC_NEVER: CoglMaterialAlphaFunc = 512;
pub const COGL_MATERIAL_ALPHA_FUNC_LESS: CoglMaterialAlphaFunc = 513;
pub const COGL_MATERIAL_ALPHA_FUNC_EQUAL: CoglMaterialAlphaFunc = 514;
pub const COGL_MATERIAL_ALPHA_FUNC_LEQUAL: CoglMaterialAlphaFunc = 515;
pub const COGL_MATERIAL_ALPHA_FUNC_GREATER: CoglMaterialAlphaFunc = 516;
pub const COGL_MATERIAL_ALPHA_FUNC_NOTEQUAL: CoglMaterialAlphaFunc = 517;
pub const COGL_MATERIAL_ALPHA_FUNC_GEQUAL: CoglMaterialAlphaFunc = 518;
pub const COGL_MATERIAL_ALPHA_FUNC_ALWAYS: CoglMaterialAlphaFunc = 519;

pub type CoglMaterialFilter = c_int;
pub const COGL_MATERIAL_FILTER_NEAREST: CoglMaterialFilter = 9728;
pub const COGL_MATERIAL_FILTER_LINEAR: CoglMaterialFilter = 9729;
pub const COGL_MATERIAL_FILTER_NEAREST_MIPMAP_NEAREST: CoglMaterialFilter = 9984;
pub const COGL_MATERIAL_FILTER_LINEAR_MIPMAP_NEAREST: CoglMaterialFilter = 9985;
pub const COGL_MATERIAL_FILTER_NEAREST_MIPMAP_LINEAR: CoglMaterialFilter = 9986;
pub const COGL_MATERIAL_FILTER_LINEAR_MIPMAP_LINEAR: CoglMaterialFilter = 9987;

pub type CoglMaterialLayerType = c_int;
pub const COGL_MATERIAL_LAYER_TYPE_TEXTURE: CoglMaterialLayerType = 0;

pub type CoglMaterialWrapMode = c_int;
pub const COGL_MATERIAL_WRAP_MODE_REPEAT: CoglMaterialWrapMode = 10497;
pub const COGL_MATERIAL_WRAP_MODE_CLAMP_TO_EDGE: CoglMaterialWrapMode = 33071;
pub const COGL_MATERIAL_WRAP_MODE_AUTOMATIC: CoglMaterialWrapMode = 519;

pub type CoglPixelFormat = c_int;
pub const COGL_PIXEL_FORMAT_ANY: CoglPixelFormat = 0;
pub const COGL_PIXEL_FORMAT_A_8: CoglPixelFormat = 17;
pub const COGL_PIXEL_FORMAT_RGB_565: CoglPixelFormat = 4;
pub const COGL_PIXEL_FORMAT_RGBA_4444: CoglPixelFormat = 21;
pub const COGL_PIXEL_FORMAT_RGBA_5551: CoglPixelFormat = 22;
pub const COGL_PIXEL_FORMAT_YUV: CoglPixelFormat = 7;
pub const COGL_PIXEL_FORMAT_G_8: CoglPixelFormat = 8;
pub const COGL_PIXEL_FORMAT_RG_88: CoglPixelFormat = 9;
pub const COGL_PIXEL_FORMAT_RGB_888: CoglPixelFormat = 2;
pub const COGL_PIXEL_FORMAT_BGR_888: CoglPixelFormat = 34;
pub const COGL_PIXEL_FORMAT_RGBA_8888: CoglPixelFormat = 19;
pub const COGL_PIXEL_FORMAT_BGRA_8888: CoglPixelFormat = 51;
pub const COGL_PIXEL_FORMAT_ARGB_8888: CoglPixelFormat = 83;
pub const COGL_PIXEL_FORMAT_ABGR_8888: CoglPixelFormat = 115;
pub const COGL_PIXEL_FORMAT_RGBA_1010102: CoglPixelFormat = 29;
pub const COGL_PIXEL_FORMAT_BGRA_1010102: CoglPixelFormat = 61;
pub const COGL_PIXEL_FORMAT_ARGB_2101010: CoglPixelFormat = 93;
pub const COGL_PIXEL_FORMAT_ABGR_2101010: CoglPixelFormat = 125;
pub const COGL_PIXEL_FORMAT_RGBA_8888_PRE: CoglPixelFormat = 147;
pub const COGL_PIXEL_FORMAT_BGRA_8888_PRE: CoglPixelFormat = 179;
pub const COGL_PIXEL_FORMAT_ARGB_8888_PRE: CoglPixelFormat = 211;
pub const COGL_PIXEL_FORMAT_ABGR_8888_PRE: CoglPixelFormat = 243;
pub const COGL_PIXEL_FORMAT_RGBA_4444_PRE: CoglPixelFormat = 149;
pub const COGL_PIXEL_FORMAT_RGBA_5551_PRE: CoglPixelFormat = 150;
pub const COGL_PIXEL_FORMAT_RGBA_1010102_PRE: CoglPixelFormat = 157;
pub const COGL_PIXEL_FORMAT_BGRA_1010102_PRE: CoglPixelFormat = 189;
pub const COGL_PIXEL_FORMAT_ARGB_2101010_PRE: CoglPixelFormat = 221;
pub const COGL_PIXEL_FORMAT_ABGR_2101010_PRE: CoglPixelFormat = 253;
pub const COGL_PIXEL_FORMAT_DEPTH_16: CoglPixelFormat = 265;
pub const COGL_PIXEL_FORMAT_DEPTH_32: CoglPixelFormat = 259;
pub const COGL_PIXEL_FORMAT_DEPTH_24_STENCIL_8: CoglPixelFormat = 771;

pub type CoglRendererError = c_int;
pub const COGL_RENDERER_ERROR_XLIB_DISPLAY_OPEN: CoglRendererError = 0;
pub const COGL_RENDERER_ERROR_BAD_CONSTRAINT: CoglRendererError = 1;

pub type CoglShaderType = c_int;
pub const COGL_SHADER_TYPE_VERTEX: CoglShaderType = 0;
pub const COGL_SHADER_TYPE_FRAGMENT: CoglShaderType = 1;

pub type CoglStereoMode = c_int;
pub const COGL_STEREO_BOTH: CoglStereoMode = 0;
pub const COGL_STEREO_LEFT: CoglStereoMode = 1;
pub const COGL_STEREO_RIGHT: CoglStereoMode = 2;

pub type CoglSystemError = c_int;
pub const COGL_SYSTEM_ERROR_UNSUPPORTED: CoglSystemError = 0;
pub const COGL_SYSTEM_ERROR_NO_MEMORY: CoglSystemError = 1;

pub type CoglTextureComponents = c_int;
pub const COGL_TEXTURE_COMPONENTS_A: CoglTextureComponents = 1;
pub const COGL_TEXTURE_COMPONENTS_RG: CoglTextureComponents = 2;
pub const COGL_TEXTURE_COMPONENTS_RGB: CoglTextureComponents = 3;
pub const COGL_TEXTURE_COMPONENTS_RGBA: CoglTextureComponents = 4;
pub const COGL_TEXTURE_COMPONENTS_DEPTH: CoglTextureComponents = 5;

pub type CoglTextureError = c_int;
pub const COGL_TEXTURE_ERROR_SIZE: CoglTextureError = 0;
pub const COGL_TEXTURE_ERROR_FORMAT: CoglTextureError = 1;
pub const COGL_TEXTURE_ERROR_BAD_PARAMETER: CoglTextureError = 2;
pub const COGL_TEXTURE_ERROR_TYPE: CoglTextureError = 3;

pub type CoglTextureType = c_int;
pub const COGL_TEXTURE_TYPE_2D: CoglTextureType = 0;
pub const COGL_TEXTURE_TYPE_3D: CoglTextureType = 1;
pub const COGL_TEXTURE_TYPE_RECTANGLE: CoglTextureType = 2;

pub type CoglVerticesMode = c_int;
pub const COGL_VERTICES_MODE_POINTS: CoglVerticesMode = 0;
pub const COGL_VERTICES_MODE_LINES: CoglVerticesMode = 1;
pub const COGL_VERTICES_MODE_LINE_LOOP: CoglVerticesMode = 2;
pub const COGL_VERTICES_MODE_LINE_STRIP: CoglVerticesMode = 3;
pub const COGL_VERTICES_MODE_TRIANGLES: CoglVerticesMode = 4;
pub const COGL_VERTICES_MODE_TRIANGLE_STRIP: CoglVerticesMode = 5;
pub const COGL_VERTICES_MODE_TRIANGLE_FAN: CoglVerticesMode = 6;

pub type CoglWinding = c_int;
pub const COGL_WINDING_CLOCKWISE: CoglWinding = 0;
pub const COGL_WINDING_COUNTER_CLOCKWISE: CoglWinding = 1;

pub type CoglWinsysFeature = c_int;
pub const COGL_WINSYS_FEATURE_MULTIPLE_ONSCREEN: CoglWinsysFeature = 0;
pub const COGL_WINSYS_FEATURE_SWAP_THROTTLE: CoglWinsysFeature = 1;
pub const COGL_WINSYS_FEATURE_VBLANK_COUNTER: CoglWinsysFeature = 2;
pub const COGL_WINSYS_FEATURE_VBLANK_WAIT: CoglWinsysFeature = 3;
pub const COGL_WINSYS_FEATURE_TEXTURE_FROM_PIXMAP: CoglWinsysFeature = 4;
pub const COGL_WINSYS_FEATURE_SWAP_BUFFERS_EVENT: CoglWinsysFeature = 5;
pub const COGL_WINSYS_FEATURE_SWAP_REGION: CoglWinsysFeature = 6;
pub const COGL_WINSYS_FEATURE_SWAP_REGION_THROTTLE: CoglWinsysFeature = 7;
pub const COGL_WINSYS_FEATURE_SWAP_REGION_SYNCHRONIZED: CoglWinsysFeature = 8;
pub const COGL_WINSYS_FEATURE_BUFFER_AGE: CoglWinsysFeature = 9;
pub const COGL_WINSYS_FEATURE_SYNC_AND_COMPLETE_EVENT: CoglWinsysFeature = 10;
pub const COGL_WINSYS_FEATURE_N_FEATURES: CoglWinsysFeature = 11;

// Constants
pub const COGL_AFIRST_BIT: c_int = 64;
pub const COGL_A_BIT: c_int = 16;
pub const COGL_BGR_BIT: c_int = 32;
pub const COGL_DEPTH_BIT: c_int = 256;
pub const COGL_FIXED_0_5: c_int = 32768;
pub const COGL_FIXED_1: c_int = 1;
pub const COGL_FIXED_2_PI: c_int = 411775;
pub const COGL_FIXED_BITS: c_int = 32;
pub const COGL_FIXED_EPSILON: c_int = 1;
pub const COGL_FIXED_MAX: c_int = 2147483647;
pub const COGL_FIXED_PI: c_int = 205887;
pub const COGL_FIXED_PI_2: c_int = 102944;
pub const COGL_FIXED_PI_4: c_int = 51472;
pub const COGL_FIXED_Q: c_int = -16;
pub const COGL_PREMULT_BIT: c_int = 128;
pub const COGL_RADIANS_TO_DEGREES: c_int = 3754936;
pub const COGL_SQRTI_ARG_10_PERCENT: c_int = 5590;
pub const COGL_SQRTI_ARG_5_PERCENT: c_int = 210;
pub const COGL_SQRTI_ARG_MAX: c_int = 4194303;
pub const COGL_STENCIL_BIT: c_int = 512;
pub const COGL_TEXTURE_MAX_WASTE: c_int = 127;

// Flags
pub type CoglBufferBit = c_uint;
pub const COGL_BUFFER_BIT_COLOR: CoglBufferBit = 1;
pub const COGL_BUFFER_BIT_DEPTH: CoglBufferBit = 2;
pub const COGL_BUFFER_BIT_STENCIL: CoglBufferBit = 4;

pub type CoglBufferTarget = c_uint;
pub const COGL_WINDOW_BUFFER: CoglBufferTarget = 2;
pub const COGL_OFFSCREEN_BUFFER: CoglBufferTarget = 4;

pub type CoglColorMask = c_uint;
pub const COGL_COLOR_MASK_NONE: CoglColorMask = 0;
pub const COGL_COLOR_MASK_RED: CoglColorMask = 1;
pub const COGL_COLOR_MASK_GREEN: CoglColorMask = 2;
pub const COGL_COLOR_MASK_BLUE: CoglColorMask = 4;
pub const COGL_COLOR_MASK_ALPHA: CoglColorMask = 8;
pub const COGL_COLOR_MASK_ALL: CoglColorMask = 15;

pub type CoglFeatureFlags = c_uint;
pub const COGL_FEATURE_TEXTURE_RECTANGLE: CoglFeatureFlags = 2;
pub const COGL_FEATURE_TEXTURE_NPOT: CoglFeatureFlags = 4;
pub const COGL_FEATURE_TEXTURE_YUV: CoglFeatureFlags = 8;
pub const COGL_FEATURE_TEXTURE_READ_PIXELS: CoglFeatureFlags = 16;
pub const COGL_FEATURE_SHADERS_GLSL: CoglFeatureFlags = 32;
pub const COGL_FEATURE_OFFSCREEN: CoglFeatureFlags = 64;
pub const COGL_FEATURE_OFFSCREEN_MULTISAMPLE: CoglFeatureFlags = 128;
pub const COGL_FEATURE_OFFSCREEN_BLIT: CoglFeatureFlags = 256;
pub const COGL_FEATURE_FOUR_CLIP_PLANES: CoglFeatureFlags = 512;
pub const COGL_FEATURE_STENCIL_BUFFER: CoglFeatureFlags = 1024;
pub const COGL_FEATURE_VBOS: CoglFeatureFlags = 2048;
pub const COGL_FEATURE_PBOS: CoglFeatureFlags = 4096;
pub const COGL_FEATURE_UNSIGNED_INT_INDICES: CoglFeatureFlags = 8192;
pub const COGL_FEATURE_DEPTH_RANGE: CoglFeatureFlags = 16384;
pub const COGL_FEATURE_TEXTURE_NPOT_BASIC: CoglFeatureFlags = 32768;
pub const COGL_FEATURE_TEXTURE_NPOT_MIPMAP: CoglFeatureFlags = 65536;
pub const COGL_FEATURE_TEXTURE_NPOT_REPEAT: CoglFeatureFlags = 131072;
pub const COGL_FEATURE_POINT_SPRITE: CoglFeatureFlags = 262144;
pub const COGL_FEATURE_TEXTURE_3D: CoglFeatureFlags = 524288;
pub const COGL_FEATURE_SHADERS_ARBFP: CoglFeatureFlags = 1048576;
pub const COGL_FEATURE_MAP_BUFFER_FOR_READ: CoglFeatureFlags = 2097152;
pub const COGL_FEATURE_MAP_BUFFER_FOR_WRITE: CoglFeatureFlags = 4194304;
pub const COGL_FEATURE_ONSCREEN_MULTIPLE: CoglFeatureFlags = 8388608;
pub const COGL_FEATURE_DEPTH_TEXTURE: CoglFeatureFlags = 16777216;

pub type CoglReadPixelsFlags = c_uint;
pub const COGL_READ_PIXELS_COLOR_BUFFER: CoglReadPixelsFlags = 1;

pub type CoglTextureFlags = c_uint;
pub const COGL_TEXTURE_NONE: CoglTextureFlags = 0;
pub const COGL_TEXTURE_NO_AUTO_MIPMAP: CoglTextureFlags = 1;
pub const COGL_TEXTURE_NO_SLICING: CoglTextureFlags = 2;
pub const COGL_TEXTURE_NO_ATLAS: CoglTextureFlags = 4;

// Callbacks
pub type CoglFuncPtr = Option<unsafe extern "C" fn()>;
pub type CoglXlibFilterFunc =
    Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> CoglFilterReturn>;

// Records
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoglColor {
    pub private_member_red: u8,
    pub private_member_green: u8,
    pub private_member_blue: u8,
    pub private_member_alpha: u8,
    pub private_member_padding0: u32,
    pub private_member_padding1: u32,
    pub private_member_padding2: u32,
}

impl ::std::fmt::Debug for CoglColor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglColor @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct _CoglEuler(c_void);

pub type CoglEuler = *mut _CoglEuler;

#[repr(C)]
pub struct _CoglMaterial(c_void);

pub type CoglMaterial = *mut _CoglMaterial;

#[repr(C)]
pub struct _CoglMaterialLayer(c_void);

pub type CoglMaterialLayer = *mut _CoglMaterialLayer;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoglMatrix {
    pub xx: c_float,
    pub yx: c_float,
    pub zx: c_float,
    pub wx: c_float,
    pub xy: c_float,
    pub yy: c_float,
    pub zy: c_float,
    pub wy: c_float,
    pub xz: c_float,
    pub yz: c_float,
    pub zz: c_float,
    pub wz: c_float,
    pub xw: c_float,
    pub yw: c_float,
    pub zw: c_float,
    pub ww: c_float,
    pub private_member_inv: [c_float; 16],
    pub private_member_type: c_ulong,
    pub private_member_flags: c_ulong,
    pub private_member__padding3: c_ulong,
}

impl ::std::fmt::Debug for CoglMatrix {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglMatrix @ {:?}", self as *const _))
            .field("xx", &self.xx)
            .field("yx", &self.yx)
            .field("zx", &self.zx)
            .field("wx", &self.wx)
            .field("xy", &self.xy)
            .field("yy", &self.yy)
            .field("zy", &self.zy)
            .field("wy", &self.wy)
            .field("xz", &self.xz)
            .field("yz", &self.yz)
            .field("zz", &self.zz)
            .field("wz", &self.wz)
            .field("xw", &self.xw)
            .field("yw", &self.yw)
            .field("zw", &self.zw)
            .field("ww", &self.ww)
            .finish()
    }
}

#[repr(C)]
pub struct _CoglQuaternion(c_void);

pub type CoglQuaternion = *mut _CoglQuaternion;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoglTextureVertex {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub tx: c_float,
    pub ty: c_float,
    pub color: CoglColor,
}

impl ::std::fmt::Debug for CoglTextureVertex {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglTextureVertex @ {:?}", self as *const _))
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("tx", &self.tx)
            .field("ty", &self.ty)
            .field("color", &self.color)
            .finish()
    }
}

#[repr(C)]
pub struct _CoglColorSizeCheck {
    _truncated_record_marker: c_void,
    // /*Ignored*/field compile_time_assert_CoglColor_size has empty c:type
}

impl ::std::fmt::Debug for _CoglColorSizeCheck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("_CoglColorSizeCheck @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct _CoglMatrixSizeCheck {
    _truncated_record_marker: c_void,
    // /*Ignored*/field compile_time_assert_CoglMatrix_size has empty c:type
}

impl ::std::fmt::Debug for _CoglMatrixSizeCheck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("_CoglMatrixSizeCheck @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct _CoglTextureVertexSizeCheck {
    _truncated_record_marker: c_void,
    // /*Ignored*/field compile_time_assert_CoglTextureVertex_size has empty c:type
}

impl ::std::fmt::Debug for _CoglTextureVertexSizeCheck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "_CoglTextureVertexSizeCheck @ {:?}",
            self as *const _
        ))
        .finish()
    }
}

// Classes
#[repr(C)]
pub struct CoglBitmap(c_void);

impl ::std::fmt::Debug for CoglBitmap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglBitmap @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglFixed(c_void);

impl ::std::fmt::Debug for CoglFixed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglFixed @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglOffscreen(c_void);

impl ::std::fmt::Debug for CoglOffscreen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglOffscreen @ {:?}", self as *const _))
            .finish()
    }
}

// Interfaces
#[repr(C)]
pub struct CoglTexture(c_void);

impl ::std::fmt::Debug for CoglTexture {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "CoglTexture @ {:?}", self as *const _)
    }
}

#[link(name = "cogl")]
extern "C" {

    //=========================================================================
    // CoglAttributeType
    //=========================================================================
    pub fn cogl_attribute_type_get_type() -> GType;

    //=========================================================================
    // CoglBitmapError
    //=========================================================================
    pub fn cogl_bitmap_error_get_type() -> GType;
    pub fn cogl_bitmap_error_quark() -> u32;

    //=========================================================================
    // CoglBlendStringError
    //=========================================================================
    pub fn cogl_blend_string_error_get_type() -> GType;
    pub fn cogl_blend_string_error_quark() -> u32;

    //=========================================================================
    // CoglDepthTestFunction
    //=========================================================================
    pub fn cogl_depth_test_function_get_type() -> GType;

    //=========================================================================
    // CoglFilterReturn
    //=========================================================================
    pub fn cogl_filter_return_get_type() -> GType;

    //=========================================================================
    // CoglFogMode
    //=========================================================================
    pub fn cogl_fog_mode_get_type() -> GType;

    //=========================================================================
    // CoglIndicesType
    //=========================================================================
    pub fn cogl_indices_type_get_type() -> GType;

    //=========================================================================
    // CoglMaterialAlphaFunc
    //=========================================================================
    pub fn cogl_material_alpha_func_get_type() -> GType;

    //=========================================================================
    // CoglMaterialFilter
    //=========================================================================
    pub fn cogl_material_filter_get_type() -> GType;

    //=========================================================================
    // CoglMaterialLayerType
    //=========================================================================
    pub fn cogl_material_layer_type_get_type() -> GType;

    //=========================================================================
    // CoglMaterialWrapMode
    //=========================================================================
    pub fn cogl_material_wrap_mode_get_type() -> GType;

    //=========================================================================
    // CoglPixelFormat
    //=========================================================================
    pub fn cogl_pixel_format_get_type() -> GType;

    //=========================================================================
    // CoglRendererError
    //=========================================================================
    pub fn cogl_renderer_error_get_type() -> GType;

    //=========================================================================
    // CoglShaderType
    //=========================================================================
    pub fn cogl_shader_type_get_type() -> GType;

    //=========================================================================
    // CoglStereoMode
    //=========================================================================
    pub fn cogl_stereo_mode_get_type() -> GType;

    //=========================================================================
    // CoglSystemError
    //=========================================================================
    pub fn cogl_system_error_get_type() -> GType;

    //=========================================================================
    // CoglTextureComponents
    //=========================================================================
    pub fn cogl_texture_components_get_type() -> GType;

    //=========================================================================
    // CoglTextureError
    //=========================================================================
    pub fn cogl_texture_error_get_type() -> GType;
    pub fn cogl_texture_error_quark() -> u32;

    //=========================================================================
    // CoglTextureType
    //=========================================================================
    pub fn cogl_texture_type_get_type() -> GType;

    //=========================================================================
    // CoglVerticesMode
    //=========================================================================
    pub fn cogl_vertices_mode_get_type() -> GType;

    //=========================================================================
    // CoglWinding
    //=========================================================================
    pub fn cogl_winding_get_type() -> GType;

    //=========================================================================
    // CoglWinsysFeature
    //=========================================================================
    pub fn cogl_winsys_feature_get_type() -> GType;

    //=========================================================================
    // CoglBufferBit
    //=========================================================================
    pub fn cogl_buffer_bit_get_type() -> GType;

    //=========================================================================
    // CoglBufferTarget
    //=========================================================================
    pub fn cogl_buffer_target_get_type() -> GType;

    //=========================================================================
    // CoglColorMask
    //=========================================================================
    pub fn cogl_color_mask_get_type() -> GType;

    //=========================================================================
    // CoglFeatureFlags
    //=========================================================================
    pub fn cogl_feature_flags_get_type() -> GType;

    //=========================================================================
    // CoglReadPixelsFlags
    //=========================================================================
    pub fn cogl_read_pixels_flags_get_type() -> GType;

    //=========================================================================
    // CoglTextureFlags
    //=========================================================================
    pub fn cogl_texture_flags_get_type() -> GType;

    //=========================================================================
    // CoglColor
    //=========================================================================
    pub fn cogl_color_get_gtype() -> GType;
    pub fn cogl_color_new() -> *mut CoglColor;
    pub fn cogl_color_copy(color: *const CoglColor) -> *mut CoglColor;
    pub fn cogl_color_free(color: *mut CoglColor);
    pub fn cogl_color_get_alpha(color: *const CoglColor) -> c_float;
    pub fn cogl_color_get_alpha_byte(color: *const CoglColor) -> u8;
    pub fn cogl_color_get_alpha_float(color: *const CoglColor) -> c_float;
    pub fn cogl_color_get_blue(color: *const CoglColor) -> c_float;
    pub fn cogl_color_get_blue_byte(color: *const CoglColor) -> u8;
    pub fn cogl_color_get_blue_float(color: *const CoglColor) -> c_float;
    pub fn cogl_color_get_green(color: *const CoglColor) -> c_float;
    pub fn cogl_color_get_green_byte(color: *const CoglColor) -> u8;
    pub fn cogl_color_get_green_float(color: *const CoglColor) -> c_float;
    pub fn cogl_color_get_red(color: *const CoglColor) -> c_float;
    pub fn cogl_color_get_red_byte(color: *const CoglColor) -> u8;
    pub fn cogl_color_get_red_float(color: *const CoglColor) -> c_float;
    pub fn cogl_color_init_from_4f(
        color: *mut CoglColor,
        red: c_float,
        green: c_float,
        blue: c_float,
        alpha: c_float,
    );
    pub fn cogl_color_init_from_4fv(color: *mut CoglColor, color_array: *const c_float);
    pub fn cogl_color_init_from_4ub(color: *mut CoglColor, red: u8, green: u8, blue: u8, alpha: u8);
    pub fn cogl_color_premultiply(color: *mut CoglColor);
    pub fn cogl_color_set_alpha(color: *mut CoglColor, alpha: c_float);
    pub fn cogl_color_set_alpha_byte(color: *mut CoglColor, alpha: u8);
    pub fn cogl_color_set_alpha_float(color: *mut CoglColor, alpha: c_float);
    pub fn cogl_color_set_blue(color: *mut CoglColor, blue: c_float);
    pub fn cogl_color_set_blue_byte(color: *mut CoglColor, blue: u8);
    pub fn cogl_color_set_blue_float(color: *mut CoglColor, blue: c_float);
    pub fn cogl_color_set_from_4f(
        color: *mut CoglColor,
        red: c_float,
        green: c_float,
        blue: c_float,
        alpha: c_float,
    );
    pub fn cogl_color_set_from_4ub(color: *mut CoglColor, red: u8, green: u8, blue: u8, alpha: u8);
    pub fn cogl_color_set_green(color: *mut CoglColor, green: c_float);
    pub fn cogl_color_set_green_byte(color: *mut CoglColor, green: u8);
    pub fn cogl_color_set_green_float(color: *mut CoglColor, green: c_float);
    pub fn cogl_color_set_red(color: *mut CoglColor, red: c_float);
    pub fn cogl_color_set_red_byte(color: *mut CoglColor, red: u8);
    pub fn cogl_color_set_red_float(color: *mut CoglColor, red: c_float);
    pub fn cogl_color_to_hsl(
        color: *const CoglColor,
        hue: *mut c_float,
        saturation: *mut c_float,
        luminance: *mut c_float,
    );
    pub fn cogl_color_unpremultiply(color: *mut CoglColor);
    pub fn cogl_color_equal(v1: *mut c_void, v2: *mut c_void) -> CoglBool;
    pub fn cogl_color_init_from_hsl(
        color: *mut CoglColor,
        hue: c_float,
        saturation: c_float,
        luminance: c_float,
    );

    //=========================================================================
    // CoglMaterial
    //=========================================================================
    pub fn cogl_material_copy(source: *mut CoglMaterial) -> *mut CoglMaterial;
    pub fn cogl_material_get_ambient(material: *mut CoglMaterial, ambient: *mut CoglColor);
    pub fn cogl_material_get_color(material: *mut CoglMaterial, color: *mut CoglColor);
    pub fn cogl_material_get_diffuse(material: *mut CoglMaterial, diffuse: *mut CoglColor);
    pub fn cogl_material_get_emission(material: *mut CoglMaterial, emission: *mut CoglColor);
    pub fn cogl_material_get_layer_point_sprite_coords_enabled(
        material: *mut CoglMaterial,
        layer_index: c_int,
    ) -> CoglBool;
    pub fn cogl_material_get_layer_wrap_mode_p(
        material: *mut CoglMaterial,
        layer_index: c_int,
    ) -> CoglMaterialWrapMode;
    pub fn cogl_material_get_layer_wrap_mode_s(
        material: *mut CoglMaterial,
        layer_index: c_int,
    ) -> CoglMaterialWrapMode;
    pub fn cogl_material_get_layer_wrap_mode_t(
        material: *mut CoglMaterial,
        layer_index: c_int,
    ) -> CoglMaterialWrapMode;
    pub fn cogl_material_get_layers(material: *mut CoglMaterial) -> *const glib::GList;
    pub fn cogl_material_get_n_layers(material: *mut CoglMaterial) -> c_int;
    pub fn cogl_material_get_point_size(material: *mut CoglMaterial) -> c_float;
    pub fn cogl_material_get_shininess(material: *mut CoglMaterial) -> c_float;
    pub fn cogl_material_get_specular(material: *mut CoglMaterial, specular: *mut CoglColor);
    pub fn cogl_material_get_user_program(material: *mut CoglMaterial) -> CoglHandle;
    pub fn cogl_material_remove_layer(material: *mut CoglMaterial, layer_index: c_int);
    pub fn cogl_material_set_alpha_test_function(
        material: *mut CoglMaterial,
        alpha_func: CoglMaterialAlphaFunc,
        alpha_reference: c_float,
    );
    pub fn cogl_material_set_ambient(material: *mut CoglMaterial, ambient: *const CoglColor);
    pub fn cogl_material_set_ambient_and_diffuse(
        material: *mut CoglMaterial,
        color: *const CoglColor,
    );
    pub fn cogl_material_set_blend(
        material: *mut CoglMaterial,
        blend_string: *const c_char,
        error: *mut *mut glib::GError,
    ) -> CoglBool;
    pub fn cogl_material_set_blend_constant(
        material: *mut CoglMaterial,
        constant_color: *const CoglColor,
    );
    pub fn cogl_material_set_color(material: *mut CoglMaterial, color: *const CoglColor);
    pub fn cogl_material_set_color4f(
        material: *mut CoglMaterial,
        red: c_float,
        green: c_float,
        blue: c_float,
        alpha: c_float,
    );
    pub fn cogl_material_set_color4ub(
        material: *mut CoglMaterial,
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
    );
    pub fn cogl_material_set_diffuse(material: *mut CoglMaterial, diffuse: *const CoglColor);
    pub fn cogl_material_set_emission(material: *mut CoglMaterial, emission: *const CoglColor);
    pub fn cogl_material_set_layer(
        material: *mut CoglMaterial,
        layer_index: c_int,
        texture: CoglHandle,
    );
    pub fn cogl_material_set_layer_combine(
        material: *mut CoglMaterial,
        layer_index: c_int,
        blend_string: *const c_char,
        error: *mut *mut glib::GError,
    ) -> CoglBool;
    pub fn cogl_material_set_layer_combine_constant(
        material: *mut CoglMaterial,
        layer_index: c_int,
        constant: *const CoglColor,
    );
    pub fn cogl_material_set_layer_filters(
        material: *mut CoglMaterial,
        layer_index: c_int,
        min_filter: CoglMaterialFilter,
        mag_filter: CoglMaterialFilter,
    );
    pub fn cogl_material_set_layer_matrix(
        material: *mut CoglMaterial,
        layer_index: c_int,
        matrix: *const CoglMatrix,
    );
    pub fn cogl_material_set_layer_point_sprite_coords_enabled(
        material: *mut CoglMaterial,
        layer_index: c_int,
        enable: CoglBool,
        error: *mut *mut glib::GError,
    ) -> CoglBool;
    pub fn cogl_material_set_layer_wrap_mode(
        material: *mut CoglMaterial,
        layer_index: c_int,
        mode: CoglMaterialWrapMode,
    );
    pub fn cogl_material_set_layer_wrap_mode_p(
        material: *mut CoglMaterial,
        layer_index: c_int,
        mode: CoglMaterialWrapMode,
    );
    pub fn cogl_material_set_layer_wrap_mode_s(
        material: *mut CoglMaterial,
        layer_index: c_int,
        mode: CoglMaterialWrapMode,
    );
    pub fn cogl_material_set_layer_wrap_mode_t(
        material: *mut CoglMaterial,
        layer_index: c_int,
        mode: CoglMaterialWrapMode,
    );
    pub fn cogl_material_set_point_size(material: *mut CoglMaterial, point_size: c_float);
    pub fn cogl_material_set_shininess(material: *mut CoglMaterial, shininess: c_float);
    pub fn cogl_material_set_specular(material: *mut CoglMaterial, specular: *const CoglColor);
    pub fn cogl_material_set_user_program(material: *mut CoglMaterial, program: CoglHandle);
    pub fn cogl_material_new() -> *mut CoglMaterial;
    pub fn cogl_material_ref(material: CoglHandle) -> CoglHandle;
    pub fn cogl_material_unref(material: CoglHandle);

    //=========================================================================
    // CoglMaterialLayer
    //=========================================================================
    pub fn cogl_material_layer_get_mag_filter(layer: *mut CoglMaterialLayer) -> CoglMaterialFilter;
    pub fn cogl_material_layer_get_min_filter(layer: *mut CoglMaterialLayer) -> CoglMaterialFilter;
    pub fn cogl_material_layer_get_texture(layer: *mut CoglMaterialLayer) -> CoglHandle;
    pub fn cogl_material_layer_get_type(layer: *mut CoglMaterialLayer) -> CoglMaterialLayerType;
    pub fn cogl_material_layer_get_wrap_mode_p(
        layer: *mut CoglMaterialLayer,
    ) -> CoglMaterialWrapMode;
    pub fn cogl_material_layer_get_wrap_mode_s(
        layer: *mut CoglMaterialLayer,
    ) -> CoglMaterialWrapMode;
    pub fn cogl_material_layer_get_wrap_mode_t(
        layer: *mut CoglMaterialLayer,
    ) -> CoglMaterialWrapMode;

    //=========================================================================
    // CoglMatrix
    //=========================================================================
    pub fn cogl_matrix_get_gtype() -> GType;
    pub fn cogl_matrix_copy(matrix: *const CoglMatrix) -> *mut CoglMatrix;
    pub fn cogl_matrix_free(matrix: *mut CoglMatrix);
    pub fn cogl_matrix_frustum(
        matrix: *mut CoglMatrix,
        left: c_float,
        right: c_float,
        bottom: c_float,
        top: c_float,
        z_near: c_float,
        z_far: c_float,
    );
    pub fn cogl_matrix_get_array(matrix: *const CoglMatrix) -> *const c_float;
    pub fn cogl_matrix_get_inverse(matrix: *const CoglMatrix, inverse: *mut CoglMatrix)
        -> CoglBool;
    pub fn cogl_matrix_init_from_array(matrix: *mut CoglMatrix, array: *const c_float);
    pub fn cogl_matrix_init_identity(matrix: *mut CoglMatrix);
    #[cfg(any(feature = "v2_0", feature = "dox"))]
    pub fn cogl_matrix_init_translation(
        matrix: *mut CoglMatrix,
        tx: c_float,
        ty: c_float,
        tz: c_float,
    );
    pub fn cogl_matrix_is_identity(matrix: *const CoglMatrix) -> CoglBool;
    pub fn cogl_matrix_look_at(
        matrix: *mut CoglMatrix,
        eye_position_x: c_float,
        eye_position_y: c_float,
        eye_position_z: c_float,
        object_x: c_float,
        object_y: c_float,
        object_z: c_float,
        world_up_x: c_float,
        world_up_y: c_float,
        world_up_z: c_float,
    );
    pub fn cogl_matrix_multiply(
        result: *mut CoglMatrix,
        a: *const CoglMatrix,
        b: *const CoglMatrix,
    );
    pub fn cogl_matrix_ortho(
        matrix: *mut CoglMatrix,
        left: c_float,
        right: c_float,
        bottom: c_float,
        top: c_float,
        near: c_float,
        far: c_float,
    );
    pub fn cogl_matrix_perspective(
        matrix: *mut CoglMatrix,
        fov_y: c_float,
        aspect: c_float,
        z_near: c_float,
        z_far: c_float,
    );
    pub fn cogl_matrix_rotate(
        matrix: *mut CoglMatrix,
        angle: c_float,
        x: c_float,
        y: c_float,
        z: c_float,
    );
    pub fn cogl_matrix_scale(matrix: *mut CoglMatrix, sx: c_float, sy: c_float, sz: c_float);
    pub fn cogl_matrix_transform_point(
        matrix: *const CoglMatrix,
        x: *mut c_float,
        y: *mut c_float,
        z: *mut c_float,
        w: *mut c_float,
    );
    pub fn cogl_matrix_translate(matrix: *mut CoglMatrix, x: c_float, y: c_float, z: c_float);
    pub fn cogl_matrix_transpose(matrix: *mut CoglMatrix);
    pub fn cogl_matrix_equal(v1: *mut c_void, v2: *mut c_void) -> CoglBool;

    //=========================================================================
    // CoglBitmap
    //=========================================================================
    pub fn cogl_bitmap_get_gtype() -> GType;
    pub fn cogl_bitmap_new_from_file(
        filename: *const c_char,
        error: *mut *mut glib::GError,
    ) -> *mut CoglBitmap;
    pub fn cogl_bitmap_get_size_from_file(
        filename: *const c_char,
        width: *mut c_int,
        height: *mut c_int,
    ) -> CoglBool;

    //=========================================================================
    // CoglFixed
    //=========================================================================
    pub fn cogl_fixed_get_type() -> GType;
    pub fn cogl_fixed_log2(x: c_uint) -> CoglFixed;
    pub fn cogl_fixed_pow(x: c_uint, y: CoglFixed) -> c_uint;
    pub fn cogl_fixed_atan(a: CoglFixed) -> CoglFixed;
    pub fn cogl_fixed_atan2(a: CoglFixed, b: CoglFixed) -> CoglFixed;
    pub fn cogl_fixed_cos(angle: CoglFixed) -> CoglFixed;
    pub fn cogl_fixed_pow2(x: CoglFixed) -> c_uint;
    pub fn cogl_fixed_sin(angle: CoglFixed) -> CoglFixed;
    pub fn cogl_fixed_sqrt(x: CoglFixed) -> CoglFixed;
    pub fn cogl_fixed_tan(angle: CoglFixed) -> CoglFixed;

    //=========================================================================
    // CoglOffscreen
    //=========================================================================
    pub fn cogl_offscreen_get_gtype() -> GType;
    pub fn cogl_offscreen_new_to_texture(texture: *mut CoglTexture) -> *mut CoglOffscreen;
    pub fn cogl_offscreen_new_with_texture(texture: *mut CoglTexture) -> *mut CoglOffscreen;
    pub fn cogl_offscreen_ref(offscreen: *mut c_void) -> *mut c_void;
    pub fn cogl_offscreen_unref(offscreen: *mut c_void);

    //=========================================================================
    // CoglTexture
    //=========================================================================
    pub fn cogl_texture_get_gtype() -> GType;
    pub fn cogl_texture_ref(texture: *mut c_void) -> *mut c_void;
    pub fn cogl_texture_unref(texture: *mut c_void);
    pub fn cogl_texture_allocate(
        texture: *mut CoglTexture,
        error: *mut *mut glib::GError,
    ) -> CoglBool;
    pub fn cogl_texture_get_components(texture: *mut CoglTexture) -> CoglTextureComponents;
    pub fn cogl_texture_get_data(
        texture: *mut CoglTexture,
        format: CoglPixelFormat,
        rowstride: c_uint,
        data: *mut u8,
    ) -> c_int;
    pub fn cogl_texture_get_gl_texture(
        texture: *mut CoglTexture,
        out_gl_handle: *mut c_uint,
        out_gl_target: *mut c_uint,
    ) -> CoglBool;
    pub fn cogl_texture_get_height(texture: *mut CoglTexture) -> c_uint;
    pub fn cogl_texture_get_max_waste(texture: *mut CoglTexture) -> c_int;
    pub fn cogl_texture_get_premultiplied(texture: *mut CoglTexture) -> CoglBool;
    pub fn cogl_texture_get_width(texture: *mut CoglTexture) -> c_uint;
    pub fn cogl_texture_is_sliced(texture: *mut CoglTexture) -> CoglBool;
    pub fn cogl_texture_set_components(
        texture: *mut CoglTexture,
        components: CoglTextureComponents,
    );
    pub fn cogl_texture_set_premultiplied(texture: *mut CoglTexture, premultiplied: CoglBool);
    pub fn cogl_texture_set_region(
        texture: *mut CoglTexture,
        src_x: c_int,
        src_y: c_int,
        dst_x: c_int,
        dst_y: c_int,
        dst_width: c_uint,
        dst_height: c_uint,
        width: c_int,
        height: c_int,
        format: CoglPixelFormat,
        rowstride: c_uint,
        data: *const u8,
    ) -> CoglBool;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn cogl_angle_cos(angle: CoglAngle) -> CoglFixed;
    pub fn cogl_angle_sin(angle: CoglAngle) -> CoglFixed;
    pub fn cogl_angle_tan(angle: CoglAngle) -> CoglFixed;
    pub fn cogl_begin_gl();
    pub fn cogl_check_extension(name: *const c_char, ext: *const c_char) -> CoglBool;
    pub fn cogl_clear(color: *const CoglColor, buffers: c_ulong);
    pub fn cogl_clip_ensure();
    pub fn cogl_clip_pop();
    pub fn cogl_clip_push(x_offset: c_float, y_offset: c_float, width: c_float, height: c_float);
    pub fn cogl_clip_push_primitive(
        primitive: *mut c_void,
        bounds_x1: c_float,
        bounds_y1: c_float,
        bounds_x2: c_float,
        bounds_y2: c_float,
    );
    pub fn cogl_clip_push_rectangle(x0: c_float, y0: c_float, x1: c_float, y1: c_float);
    pub fn cogl_clip_push_window_rect(
        x_offset: c_float,
        y_offset: c_float,
        width: c_float,
        height: c_float,
    );
    pub fn cogl_clip_push_window_rectangle(
        x_offset: c_int,
        y_offset: c_int,
        width: c_int,
        height: c_int,
    );
    pub fn cogl_clip_stack_restore();
    pub fn cogl_clip_stack_save();
    pub fn cogl_clutter_check_extension_CLUTTER(
        name: *const c_char,
        ext: *const c_char,
    ) -> CoglBool;
    pub fn cogl_clutter_winsys_has_feature_CLUTTER(feature: CoglWinsysFeature) -> CoglBool;
    pub fn cogl_clutter_winsys_xlib_get_visual_info_CLUTTER();
    pub fn cogl_create_program() -> CoglHandle;
    pub fn cogl_create_shader(shader_type: CoglShaderType) -> CoglHandle;
    #[cfg(any(feature = "v2_0", feature = "dox"))]
    pub fn cogl_debug_matrix_print(matrix: *const CoglMatrix);
    pub fn cogl_disable_fog();
    pub fn cogl_double_to_fixed(value: c_double) -> CoglFixed;
    pub fn cogl_double_to_int(value: c_double) -> c_int;
    pub fn cogl_double_to_uint(value: c_double) -> c_uint;
    pub fn cogl_end_gl();
    pub fn cogl_features_available(features: CoglFeatureFlags) -> CoglBool;
    pub fn cogl_flush();
    pub fn cogl_framebuffer_get_color_format(framebuffer: *mut c_void) -> CoglPixelFormat;
    pub fn cogl_frustum(
        left: c_float,
        right: c_float,
        bottom: c_float,
        top: c_float,
        z_near: c_float,
        z_far: c_float,
    );
    pub fn cogl_get_backface_culling_enabled() -> CoglBool;
    pub fn cogl_get_bitmasks(
        red: *mut c_int,
        green: *mut c_int,
        blue: *mut c_int,
        alpha: *mut c_int,
    );
    pub fn cogl_get_depth_test_enabled() -> CoglBool;
    pub fn cogl_get_features() -> CoglFeatureFlags;
    pub fn cogl_get_modelview_matrix(matrix: *mut CoglMatrix);
    pub fn cogl_get_option_group() -> *mut glib::GOptionGroup;
    pub fn cogl_get_proc_address(name: *const c_char) -> CoglFuncPtr;
    pub fn cogl_get_projection_matrix(matrix: *mut CoglMatrix);
    pub fn cogl_get_source() -> *mut c_void;
    pub fn cogl_get_viewport(v: *mut [c_float; 4]);
    pub fn cogl_gtype_matrix_get_type() -> GType;
    pub fn cogl_handle_get_type() -> GType;
    pub fn cogl_handle_ref(handle: CoglHandle) -> CoglHandle;
    pub fn cogl_handle_unref(handle: CoglHandle);
    pub fn cogl_is_bitmap(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_material(handle: CoglHandle) -> CoglBool;
    pub fn cogl_is_offscreen(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_program(handle: CoglHandle) -> CoglBool;
    pub fn cogl_is_shader(handle: CoglHandle) -> CoglBool;
    pub fn cogl_is_texture(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_vertex_buffer(handle: CoglHandle) -> CoglBool;
    pub fn cogl_is_vertex_buffer_indices(handle: CoglHandle) -> CoglBool;
    pub fn cogl_onscreen_clutter_backend_set_size_CLUTTER(width: c_int, height: c_int);
    pub fn cogl_ortho(
        left: c_float,
        right: c_float,
        bottom: c_float,
        top: c_float,
        near: c_float,
        far: c_float,
    );
    pub fn cogl_perspective(fovy: c_float, aspect: c_float, z_near: c_float, z_far: c_float);
    pub fn cogl_polygon(
        vertices: *const CoglTextureVertex,
        n_vertices: c_uint,
        use_color: CoglBool,
    );
    pub fn cogl_pop_draw_buffer();
    pub fn cogl_pop_framebuffer();
    pub fn cogl_pop_matrix();
    pub fn cogl_pop_source();
    pub fn cogl_program_attach_shader(program_handle: CoglHandle, shader_handle: CoglHandle);
    pub fn cogl_program_get_uniform_location(
        handle: CoglHandle,
        uniform_name: *const c_char,
    ) -> c_int;
    pub fn cogl_program_link(handle: CoglHandle);
    pub fn cogl_program_ref(handle: CoglHandle) -> CoglHandle;
    pub fn cogl_program_set_uniform_1f(
        program: CoglHandle,
        uniform_location: c_int,
        value: c_float,
    );
    pub fn cogl_program_set_uniform_1i(program: CoglHandle, uniform_location: c_int, value: c_int);
    pub fn cogl_program_set_uniform_float(
        program: CoglHandle,
        uniform_location: c_int,
        n_components: c_int,
        count: c_int,
        value: *const c_float,
    );
    pub fn cogl_program_set_uniform_int(
        program: CoglHandle,
        uniform_location: c_int,
        n_components: c_int,
        count: c_int,
        value: *const c_int,
    );
    pub fn cogl_program_set_uniform_matrix(
        program: CoglHandle,
        uniform_location: c_int,
        dimensions: c_int,
        count: c_int,
        transpose: CoglBool,
        value: *const c_float,
    );
    pub fn cogl_program_uniform_1f(uniform_no: c_int, value: c_float);
    pub fn cogl_program_uniform_1i(uniform_no: c_int, value: c_int);
    pub fn cogl_program_uniform_float(
        uniform_no: c_int,
        size: c_int,
        count: c_int,
        value: *const c_float,
    );
    pub fn cogl_program_uniform_int(
        uniform_no: c_int,
        size: c_int,
        count: c_int,
        value: *const c_int,
    );
    pub fn cogl_program_uniform_matrix(
        uniform_no: c_int,
        size: c_int,
        count: c_int,
        transpose: CoglBool,
        value: *const c_float,
    );
    pub fn cogl_program_unref(handle: CoglHandle);
    pub fn cogl_program_use(handle: CoglHandle);
    pub fn cogl_push_draw_buffer();
    pub fn cogl_push_framebuffer(buffer: *mut c_void);
    pub fn cogl_push_matrix();
    pub fn cogl_push_source(material: *mut c_void);
    pub fn cogl_read_pixels(
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        source: CoglReadPixelsFlags,
        format: CoglPixelFormat,
        pixels: *mut u8,
    );
    pub fn cogl_rectangle(x_1: c_float, y_1: c_float, x_2: c_float, y_2: c_float);
    pub fn cogl_rectangle_with_multitexture_coords(
        x1: c_float,
        y1: c_float,
        x2: c_float,
        y2: c_float,
        tex_coords: *const c_float,
        tex_coords_len: c_int,
    );
    pub fn cogl_rectangle_with_texture_coords(
        x1: c_float,
        y1: c_float,
        x2: c_float,
        y2: c_float,
        tx1: c_float,
        ty1: c_float,
        tx2: c_float,
        ty2: c_float,
    );
    pub fn cogl_rectangles(verts: *const c_float, n_rects: c_uint);
    pub fn cogl_rectangles_with_texture_coords(verts: *const c_float, n_rects: c_uint);
    pub fn cogl_rotate(angle: c_float, x: c_float, y: c_float, z: c_float);
    pub fn cogl_scale(x: c_float, y: c_float, z: c_float);
    pub fn cogl_set_backface_culling_enabled(setting: CoglBool);
    pub fn cogl_set_depth_test_enabled(setting: CoglBool);
    pub fn cogl_set_draw_buffer(target: CoglBufferTarget, offscreen: CoglHandle);
    pub fn cogl_set_fog(
        fog_color: *const CoglColor,
        mode: CoglFogMode,
        density: c_float,
        z_near: c_float,
        z_far: c_float,
    );
    pub fn cogl_set_framebuffer(buffer: *mut c_void);
    pub fn cogl_set_modelview_matrix(matrix: *mut CoglMatrix);
    pub fn cogl_set_projection_matrix(matrix: *mut CoglMatrix);
    pub fn cogl_set_source(material: *mut c_void);
    pub fn cogl_set_source_color(color: *const CoglColor);
    pub fn cogl_set_source_color4f(red: c_float, green: c_float, blue: c_float, alpha: c_float);
    pub fn cogl_set_source_color4ub(red: u8, green: u8, blue: u8, alpha: u8);
    pub fn cogl_set_source_texture(texture: *mut CoglTexture);
    pub fn cogl_set_viewport(x: c_int, y: c_int, width: c_int, height: c_int);
    pub fn cogl_shader_compile(handle: CoglHandle);
    pub fn cogl_shader_get_info_log(handle: CoglHandle) -> *mut c_char;
    pub fn cogl_shader_get_type(handle: CoglHandle) -> CoglShaderType;
    pub fn cogl_shader_is_compiled(handle: CoglHandle) -> CoglBool;
    pub fn cogl_shader_ref(handle: CoglHandle) -> CoglHandle;
    pub fn cogl_shader_source(shader: CoglHandle, source: *const c_char);
    pub fn cogl_shader_unref(handle: CoglHandle);
    pub fn cogl_sqrti(x: c_int) -> c_int;
    pub fn cogl_transform(matrix: *const CoglMatrix);
    pub fn cogl_translate(x: c_float, y: c_float, z: c_float);
    pub fn cogl_vertex_buffer_add(
        handle: CoglHandle,
        attribute_name: *const c_char,
        n_components: u8,
        type_: CoglAttributeType,
        normalized: CoglBool,
        stride: u16,
        pointer: *mut c_void,
    );
    pub fn cogl_vertex_buffer_delete(handle: CoglHandle, attribute_name: *const c_char);
    pub fn cogl_vertex_buffer_disable(handle: CoglHandle, attribute_name: *const c_char);
    pub fn cogl_vertex_buffer_draw(
        handle: CoglHandle,
        mode: CoglVerticesMode,
        first: c_int,
        count: c_int,
    );
    pub fn cogl_vertex_buffer_draw_elements(
        handle: CoglHandle,
        mode: CoglVerticesMode,
        indices: CoglHandle,
        min_index: c_int,
        max_index: c_int,
        indices_offset: c_int,
        count: c_int,
    );
    pub fn cogl_vertex_buffer_enable(handle: CoglHandle, attribute_name: *const c_char);
    pub fn cogl_vertex_buffer_get_n_vertices(handle: CoglHandle) -> c_uint;
    pub fn cogl_vertex_buffer_indices_get_for_quads(n_indices: c_uint) -> CoglHandle;
    pub fn cogl_vertex_buffer_indices_get_type(indices: CoglHandle) -> CoglIndicesType;
    pub fn cogl_vertex_buffer_indices_new(
        indices_type: CoglIndicesType,
        indices_array: *mut c_void,
        indices_len: c_int,
    ) -> CoglHandle;
    pub fn cogl_vertex_buffer_new(n_vertices: c_uint) -> CoglHandle;
    pub fn cogl_vertex_buffer_ref(handle: CoglHandle) -> CoglHandle;
    pub fn cogl_vertex_buffer_submit(handle: CoglHandle);
    pub fn cogl_vertex_buffer_unref(handle: CoglHandle);
    pub fn cogl_viewport(width: c_uint, height: c_uint);
    pub fn cogl_xlib_renderer_add_filter(
        renderer: *mut c_void,
        func: CoglXlibFilterFunc,
        data: *mut c_void,
    );
    pub fn cogl_xlib_renderer_get_display(renderer: *mut c_void);
    pub fn cogl_xlib_renderer_get_foreign_display(renderer: *mut c_void);
    pub fn cogl_xlib_renderer_get_visual_info(renderer: *mut c_void);
    pub fn cogl_xlib_renderer_handle_event(
        renderer: *mut c_void,
        event: *mut c_void,
    ) -> CoglFilterReturn;
    pub fn cogl_xlib_renderer_remove_filter(
        renderer: *mut c_void,
        func: CoglXlibFilterFunc,
        data: *mut c_void,
    );
    pub fn cogl_xlib_renderer_set_event_retrieval_enabled(renderer: *mut c_void, enable: CoglBool);
    pub fn cogl_xlib_renderer_set_foreign_display(renderer: *mut c_void, display: *mut c_void);

}

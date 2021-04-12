#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::upper_case_acronyms,
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal
)]

extern crate glib_sys as glib;
extern crate gobject_sys as gobject;
extern crate libc;

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
pub type CoglBuffer = c_void;
pub type CoglHandle = *mut c_void;
pub type CoglMetaTexture = c_void;
pub type CoglPrimitiveTexture = c_void;
pub type CoglUserDataDestroyCallback = glib::GDestroyNotify;

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

pub type CoglBufferError = c_int;
pub const COGL_BUFFER_ERROR_MAP: CoglBufferError = 0;

pub type CoglBufferUpdateHint = c_int;
pub const COGL_BUFFER_UPDATE_HINT_STATIC: CoglBufferUpdateHint = 0;
pub const COGL_BUFFER_UPDATE_HINT_DYNAMIC: CoglBufferUpdateHint = 1;
pub const COGL_BUFFER_UPDATE_HINT_STREAM: CoglBufferUpdateHint = 2;

pub type CoglDepthTestFunction = c_int;
pub const COGL_DEPTH_TEST_FUNCTION_NEVER: CoglDepthTestFunction = 512;
pub const COGL_DEPTH_TEST_FUNCTION_LESS: CoglDepthTestFunction = 513;
pub const COGL_DEPTH_TEST_FUNCTION_EQUAL: CoglDepthTestFunction = 514;
pub const COGL_DEPTH_TEST_FUNCTION_LEQUAL: CoglDepthTestFunction = 515;
pub const COGL_DEPTH_TEST_FUNCTION_GREATER: CoglDepthTestFunction = 516;
pub const COGL_DEPTH_TEST_FUNCTION_NOTEQUAL: CoglDepthTestFunction = 517;
pub const COGL_DEPTH_TEST_FUNCTION_GEQUAL: CoglDepthTestFunction = 518;
pub const COGL_DEPTH_TEST_FUNCTION_ALWAYS: CoglDepthTestFunction = 519;

pub type CoglDriver = c_int;
pub const COGL_DRIVER_ANY: CoglDriver = 0;
pub const COGL_DRIVER_NOP: CoglDriver = 1;
pub const COGL_DRIVER_GL: CoglDriver = 2;
pub const COGL_DRIVER_GL3: CoglDriver = 3;
pub const COGL_DRIVER_GLES1: CoglDriver = 4;
pub const COGL_DRIVER_GLES2: CoglDriver = 5;
pub const COGL_DRIVER_WEBGL: CoglDriver = 6;

pub type CoglFeatureID = c_int;
pub const COGL_FEATURE_ID_TEXTURE_NPOT_BASIC: CoglFeatureID = 1;
pub const COGL_FEATURE_ID_TEXTURE_NPOT_MIPMAP: CoglFeatureID = 2;
pub const COGL_FEATURE_ID_TEXTURE_NPOT_REPEAT: CoglFeatureID = 3;
pub const COGL_FEATURE_ID_TEXTURE_NPOT: CoglFeatureID = 4;
pub const COGL_FEATURE_ID_TEXTURE_RECTANGLE: CoglFeatureID = 5;
pub const COGL_FEATURE_ID_TEXTURE_3D: CoglFeatureID = 6;
pub const COGL_FEATURE_ID_GLSL: CoglFeatureID = 7;
pub const COGL_FEATURE_ID_ARBFP: CoglFeatureID = 8;
pub const COGL_FEATURE_ID_OFFSCREEN: CoglFeatureID = 9;
pub const COGL_FEATURE_ID_OFFSCREEN_MULTISAMPLE: CoglFeatureID = 10;
pub const COGL_FEATURE_ID_ONSCREEN_MULTIPLE: CoglFeatureID = 11;
pub const COGL_FEATURE_ID_UNSIGNED_INT_INDICES: CoglFeatureID = 12;
pub const COGL_FEATURE_ID_DEPTH_RANGE: CoglFeatureID = 13;
pub const COGL_FEATURE_ID_POINT_SPRITE: CoglFeatureID = 14;
pub const COGL_FEATURE_ID_MAP_BUFFER_FOR_READ: CoglFeatureID = 15;
pub const COGL_FEATURE_ID_MAP_BUFFER_FOR_WRITE: CoglFeatureID = 16;
pub const COGL_FEATURE_ID_MIRRORED_REPEAT: CoglFeatureID = 17;
pub const COGL_FEATURE_ID_SWAP_BUFFERS_EVENT: CoglFeatureID = 18;
pub const COGL_FEATURE_ID_GLES2_CONTEXT: CoglFeatureID = 19;
pub const COGL_FEATURE_ID_DEPTH_TEXTURE: CoglFeatureID = 20;
pub const COGL_FEATURE_ID_PRESENTATION_TIME: CoglFeatureID = 21;
pub const COGL_FEATURE_ID_FENCE: CoglFeatureID = 22;
pub const COGL_FEATURE_ID_PER_VERTEX_POINT_SIZE: CoglFeatureID = 23;
pub const COGL_FEATURE_ID_TEXTURE_RG: CoglFeatureID = 24;
pub const COGL_FEATURE_ID_BUFFER_AGE: CoglFeatureID = 25;

pub type CoglFilterReturn = c_int;
pub const COGL_FILTER_CONTINUE: CoglFilterReturn = 0;
pub const COGL_FILTER_REMOVE: CoglFilterReturn = 1;

pub type CoglFogMode = c_int;
pub const COGL_FOG_MODE_LINEAR: CoglFogMode = 0;
pub const COGL_FOG_MODE_EXPONENTIAL: CoglFogMode = 1;
pub const COGL_FOG_MODE_EXPONENTIAL_SQUARED: CoglFogMode = 2;

pub type CoglFrameEvent = c_int;
pub const COGL_FRAME_EVENT_SYNC: CoglFrameEvent = 1;
pub const COGL_FRAME_EVENT_COMPLETE: CoglFrameEvent = 2;

pub type CoglFramebufferError = c_int;
pub const COGL_FRAMEBUFFER_ERROR_ALLOCATE: CoglFramebufferError = 0;

pub type CoglGLES2ContextError = c_int;
pub const COGL_GLES2_CONTEXT_ERROR_UNSUPPORTED: CoglGLES2ContextError = 0;
pub const COGL_GLES2_CONTEXT_ERROR_DRIVER: CoglGLES2ContextError = 1;

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

pub type CoglPipelineAlphaFunc = c_int;
pub const COGL_PIPELINE_ALPHA_FUNC_NEVER: CoglPipelineAlphaFunc = 512;
pub const COGL_PIPELINE_ALPHA_FUNC_LESS: CoglPipelineAlphaFunc = 513;
pub const COGL_PIPELINE_ALPHA_FUNC_EQUAL: CoglPipelineAlphaFunc = 514;
pub const COGL_PIPELINE_ALPHA_FUNC_LEQUAL: CoglPipelineAlphaFunc = 515;
pub const COGL_PIPELINE_ALPHA_FUNC_GREATER: CoglPipelineAlphaFunc = 516;
pub const COGL_PIPELINE_ALPHA_FUNC_NOTEQUAL: CoglPipelineAlphaFunc = 517;
pub const COGL_PIPELINE_ALPHA_FUNC_GEQUAL: CoglPipelineAlphaFunc = 518;
pub const COGL_PIPELINE_ALPHA_FUNC_ALWAYS: CoglPipelineAlphaFunc = 519;

pub type CoglPipelineCullFaceMode = c_int;
pub const COGL_PIPELINE_CULL_FACE_MODE_NONE: CoglPipelineCullFaceMode = 0;
pub const COGL_PIPELINE_CULL_FACE_MODE_FRONT: CoglPipelineCullFaceMode = 1;
pub const COGL_PIPELINE_CULL_FACE_MODE_BACK: CoglPipelineCullFaceMode = 2;
pub const COGL_PIPELINE_CULL_FACE_MODE_BOTH: CoglPipelineCullFaceMode = 3;

pub type CoglPipelineFilter = c_int;
pub const COGL_PIPELINE_FILTER_NEAREST: CoglPipelineFilter = 9728;
pub const COGL_PIPELINE_FILTER_LINEAR: CoglPipelineFilter = 9729;
pub const COGL_PIPELINE_FILTER_NEAREST_MIPMAP_NEAREST: CoglPipelineFilter = 9984;
pub const COGL_PIPELINE_FILTER_LINEAR_MIPMAP_NEAREST: CoglPipelineFilter = 9985;
pub const COGL_PIPELINE_FILTER_NEAREST_MIPMAP_LINEAR: CoglPipelineFilter = 9986;
pub const COGL_PIPELINE_FILTER_LINEAR_MIPMAP_LINEAR: CoglPipelineFilter = 9987;

pub type CoglPipelineWrapMode = c_int;
pub const COGL_PIPELINE_WRAP_MODE_REPEAT: CoglPipelineWrapMode = 10497;
pub const COGL_PIPELINE_WRAP_MODE_MIRRORED_REPEAT: CoglPipelineWrapMode = 33648;
pub const COGL_PIPELINE_WRAP_MODE_CLAMP_TO_EDGE: CoglPipelineWrapMode = 33071;
pub const COGL_PIPELINE_WRAP_MODE_AUTOMATIC: CoglPipelineWrapMode = 519;

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

pub type CoglPollFDEvent = c_int;
pub const COGL_POLL_FD_EVENT_IN: CoglPollFDEvent = 1;
pub const COGL_POLL_FD_EVENT_PRI: CoglPollFDEvent = 2;
pub const COGL_POLL_FD_EVENT_OUT: CoglPollFDEvent = 4;
pub const COGL_POLL_FD_EVENT_ERR: CoglPollFDEvent = 8;
pub const COGL_POLL_FD_EVENT_HUP: CoglPollFDEvent = 16;
pub const COGL_POLL_FD_EVENT_NVAL: CoglPollFDEvent = 32;

pub type CoglRendererError = c_int;
pub const COGL_RENDERER_ERROR_XLIB_DISPLAY_OPEN: CoglRendererError = 0;
pub const COGL_RENDERER_ERROR_BAD_CONSTRAINT: CoglRendererError = 1;

pub type CoglShaderType = c_int;
pub const COGL_SHADER_TYPE_VERTEX: CoglShaderType = 0;
pub const COGL_SHADER_TYPE_FRAGMENT: CoglShaderType = 1;

pub type CoglSnippetHook = c_int;
pub const COGL_SNIPPET_HOOK_VERTEX: CoglSnippetHook = 0;
pub const COGL_SNIPPET_HOOK_VERTEX_TRANSFORM: CoglSnippetHook = 1;
pub const COGL_SNIPPET_HOOK_VERTEX_GLOBALS: CoglSnippetHook = 2;
pub const COGL_SNIPPET_HOOK_POINT_SIZE: CoglSnippetHook = 3;
pub const COGL_SNIPPET_HOOK_FRAGMENT: CoglSnippetHook = 2048;
pub const COGL_SNIPPET_HOOK_FRAGMENT_GLOBALS: CoglSnippetHook = 2049;
pub const COGL_SNIPPET_HOOK_TEXTURE_COORD_TRANSFORM: CoglSnippetHook = 4096;
pub const COGL_SNIPPET_HOOK_LAYER_FRAGMENT: CoglSnippetHook = 6144;
pub const COGL_SNIPPET_HOOK_TEXTURE_LOOKUP: CoglSnippetHook = 6145;

pub type CoglStereoMode = c_int;
pub const COGL_STEREO_BOTH: CoglStereoMode = 0;
pub const COGL_STEREO_LEFT: CoglStereoMode = 1;
pub const COGL_STEREO_RIGHT: CoglStereoMode = 2;

pub type CoglSubpixelOrder = c_int;
pub const COGL_SUBPIXEL_ORDER_UNKNOWN: CoglSubpixelOrder = 0;
pub const COGL_SUBPIXEL_ORDER_NONE: CoglSubpixelOrder = 1;
pub const COGL_SUBPIXEL_ORDER_HORIZONTAL_RGB: CoglSubpixelOrder = 2;
pub const COGL_SUBPIXEL_ORDER_HORIZONTAL_BGR: CoglSubpixelOrder = 3;
pub const COGL_SUBPIXEL_ORDER_VERTICAL_RGB: CoglSubpixelOrder = 4;
pub const COGL_SUBPIXEL_ORDER_VERTICAL_BGR: CoglSubpixelOrder = 5;

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

pub type CoglTexturePixmapX11Error = c_int;
pub const COGL_TEXTURE_PIXMAP_X11_ERROR_X11: CoglTexturePixmapX11Error = 0;

pub type CoglTexturePixmapX11ReportLevel = c_int;
pub const COGL_TEXTURE_PIXMAP_X11_DAMAGE_RAW_RECTANGLES: CoglTexturePixmapX11ReportLevel = 0;
pub const COGL_TEXTURE_PIXMAP_X11_DAMAGE_DELTA_RECTANGLES: CoglTexturePixmapX11ReportLevel = 1;
pub const COGL_TEXTURE_PIXMAP_X11_DAMAGE_BOUNDING_BOX: CoglTexturePixmapX11ReportLevel = 2;
pub const COGL_TEXTURE_PIXMAP_X11_DAMAGE_NON_EMPTY: CoglTexturePixmapX11ReportLevel = 3;

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

pub type CoglWinsysID = c_int;
pub const COGL_WINSYS_ID_ANY: CoglWinsysID = 0;
pub const COGL_WINSYS_ID_STUB: CoglWinsysID = 1;
pub const COGL_WINSYS_ID_GLX: CoglWinsysID = 2;
pub const COGL_WINSYS_ID_EGL_XLIB: CoglWinsysID = 3;
pub const COGL_WINSYS_ID_EGL_NULL: CoglWinsysID = 4;
pub const COGL_WINSYS_ID_EGL_GDL: CoglWinsysID = 5;
pub const COGL_WINSYS_ID_EGL_WAYLAND: CoglWinsysID = 6;
pub const COGL_WINSYS_ID_EGL_KMS: CoglWinsysID = 7;
pub const COGL_WINSYS_ID_EGL_ANDROID: CoglWinsysID = 8;
pub const COGL_WINSYS_ID_EGL_MIR: CoglWinsysID = 9;
pub const COGL_WINSYS_ID_WGL: CoglWinsysID = 10;
pub const COGL_WINSYS_ID_SDL: CoglWinsysID = 11;

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
pub const COGL_FIXED_MIN: c_int = -2147483648;
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
pub const COGL_VERSION_COMPONENT_BITS: c_int = 10;
pub const COGL_VERSION_MAX_COMPONENT_VALUE: c_int = 0;

// Flags
pub type CoglBufferAccess = c_uint;
pub const COGL_BUFFER_ACCESS_READ: CoglBufferAccess = 1;
pub const COGL_BUFFER_ACCESS_WRITE: CoglBufferAccess = 2;
pub const COGL_BUFFER_ACCESS_READ_WRITE: CoglBufferAccess = 3;

pub type CoglBufferBit = c_uint;
pub const COGL_BUFFER_BIT_COLOR: CoglBufferBit = 1;
pub const COGL_BUFFER_BIT_DEPTH: CoglBufferBit = 2;
pub const COGL_BUFFER_BIT_STENCIL: CoglBufferBit = 4;

pub type CoglBufferMapHint = c_uint;
pub const COGL_BUFFER_MAP_HINT_DISCARD: CoglBufferMapHint = 1;
pub const COGL_BUFFER_MAP_HINT_DISCARD_RANGE: CoglBufferMapHint = 2;

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

pub type CoglRendererConstraint = c_uint;
pub const COGL_RENDERER_CONSTRAINT_USES_X11: CoglRendererConstraint = 1;
pub const COGL_RENDERER_CONSTRAINT_USES_XLIB: CoglRendererConstraint = 2;
pub const COGL_RENDERER_CONSTRAINT_USES_EGL: CoglRendererConstraint = 4;
pub const COGL_RENDERER_CONSTRAINT_SUPPORTS_COGL_GLES2: CoglRendererConstraint = 8;

pub type CoglTextureFlags = c_uint;
pub const COGL_TEXTURE_NONE: CoglTextureFlags = 0;
pub const COGL_TEXTURE_NO_AUTO_MIPMAP: CoglTextureFlags = 1;
pub const COGL_TEXTURE_NO_SLICING: CoglTextureFlags = 2;
pub const COGL_TEXTURE_NO_ATLAS: CoglTextureFlags = 4;

// Callbacks
pub type CoglDebugObjectForeachTypeCallback =
    Option<unsafe extern "C" fn(*const CoglDebugObjectTypeInfo, *mut c_void)>;
pub type CoglFeatureCallback = Option<unsafe extern "C" fn(CoglFeatureID, *mut c_void)>;
pub type CoglFenceCallback = Option<unsafe extern "C" fn(*mut CoglFence, *mut c_void)>;
pub type CoglFrameCallback = Option<
    unsafe extern "C" fn(*mut CoglOnscreen, CoglFrameEvent, *mut CoglFrameInfo, *mut c_void),
>;
pub type CoglFuncPtr = Option<unsafe extern "C" fn()>;
pub type CoglMetaTextureCallback =
    Option<unsafe extern "C" fn(*mut CoglTexture, *const c_float, *const c_float, *mut c_void)>;
pub type CoglOnscreenDirtyCallback =
    Option<unsafe extern "C" fn(*mut CoglOnscreen, *const CoglOnscreenDirtyInfo, *mut c_void)>;
pub type CoglOnscreenResizeCallback =
    Option<unsafe extern "C" fn(*mut CoglOnscreen, c_int, c_int, *mut c_void)>;
pub type CoglOnscreenX11MaskCallback =
    Option<unsafe extern "C" fn(*mut CoglOnscreen, u32, *mut c_void)>;
pub type CoglOutputCallback = Option<unsafe extern "C" fn(*mut CoglOutput, *mut c_void)>;
pub type CoglPipelineLayerCallback =
    Option<unsafe extern "C" fn(*mut CoglPipeline, c_int, *mut c_void) -> CoglBool>;
pub type CoglPrimitiveAttributeCallback =
    Option<unsafe extern "C" fn(*mut CoglPrimitive, *mut CoglAttribute, *mut c_void) -> CoglBool>;
pub type CoglSwapBuffersNotify = Option<unsafe extern "C" fn(*mut CoglFramebuffer, *mut c_void)>;
// pub type CoglXlibFilterFunc = Option<unsafe extern "C" fn(*mut XEvent, *mut c_void) -> CoglFilterReturn>;

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
#[derive(Copy, Clone)]
pub struct CoglDebugObjectTypeInfo {
    pub name: *const c_char,
    pub instance_count: c_ulong,
}

impl ::std::fmt::Debug for CoglDebugObjectTypeInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglDebugObjectTypeInfo @ {:?}", self as *const _))
            .field("name", &self.name)
            .field("instance_count", &self.instance_count)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoglDepthState {
    pub private_member_magic: u32,
    pub private_member_test_enabled: CoglBool,
    pub private_member_test_function: CoglDepthTestFunction,
    pub private_member_write_enabled: CoglBool,
    pub private_member_range_near: c_float,
    pub private_member_range_far: c_float,
    pub private_member_padding0: u32,
    pub private_member_padding1: u32,
    pub private_member_padding2: u32,
    pub private_member_padding3: u32,
    pub private_member_padding4: u32,
    pub private_member_padding5: u32,
    pub private_member_padding6: u32,
    pub private_member_padding7: u32,
    pub private_member_padding8: u32,
    pub private_member_padding9: u32,
}

impl ::std::fmt::Debug for CoglDepthState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglDepthState @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoglEuler {
    pub heading: c_float,
    pub pitch: c_float,
    pub roll: c_float,
    pub padding0: c_float,
    pub padding1: c_float,
    pub padding2: c_float,
    pub padding3: c_float,
    pub padding4: c_float,
}

impl ::std::fmt::Debug for CoglEuler {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglEuler @ {:?}", self as *const _))
            .field("heading", &self.heading)
            .field("pitch", &self.pitch)
            .field("roll", &self.roll)
            .finish()
    }
}

#[repr(C)]
pub struct _CoglFence(c_void);

pub type CoglFence = *mut _CoglFence;

#[repr(C)]
pub struct _CoglFenceClosure(c_void);

pub type CoglFenceClosure = *mut _CoglFenceClosure;

#[repr(C)]
pub struct CoglFrameClosure(c_void);

impl ::std::fmt::Debug for CoglFrameClosure {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglFrameClosure @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoglGtypeClass {
    pub base_class: gobject::GTypeClass,
    pub dummy: c_uint,
}

impl ::std::fmt::Debug for CoglGtypeClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglGtypeClass @ {:?}", self as *const _))
            .field("base_class", &self.base_class)
            .field("dummy", &self.dummy)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoglGtypeObject {
    pub parent_instance: gobject::GTypeInstance,
    pub dummy: c_uint,
}

impl ::std::fmt::Debug for CoglGtypeObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglGtypeObject @ {:?}", self as *const _))
            .field("parent_instance", &self.parent_instance)
            .field("dummy", &self.dummy)
            .finish()
    }
}

#[repr(C)]
pub struct CoglKmsCrtc {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    _truncated_record_marker: c_void,
    // /*Ignored*/field mode has incomplete type
}

impl ::std::fmt::Debug for CoglKmsCrtc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglKmsCrtc @ {:?}", self as *const _))
            .field("id", &self.id)
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

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
pub struct CoglMatrixEntry(c_void);

impl ::std::fmt::Debug for CoglMatrixEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglMatrixEntry @ {:?}", self as *const _))
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

#[repr(C)]
pub struct CoglOnscreenDirtyClosure(c_void);

impl ::std::fmt::Debug for CoglOnscreenDirtyClosure {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "CoglOnscreenDirtyClosure @ {:?}",
            self as *const _
        ))
        .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoglOnscreenDirtyInfo {
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
}

impl ::std::fmt::Debug for CoglOnscreenDirtyInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglOnscreenDirtyInfo @ {:?}", self as *const _))
            .field("x", &self.x)
            .field("y", &self.y)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

#[repr(C)]
pub struct CoglOnscreenResizeClosure(c_void);

impl ::std::fmt::Debug for CoglOnscreenResizeClosure {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "CoglOnscreenResizeClosure @ {:?}",
            self as *const _
        ))
        .finish()
    }
}

#[repr(C)]
pub struct CoglPollFD {
    pub fd: c_int,
    _truncated_record_marker: c_void,
    // /*Ignored*/field events has incomplete type
}

impl ::std::fmt::Debug for CoglPollFD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglPollFD @ {:?}", self as *const _))
            .field("fd", &self.fd)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoglQuaternion {
    pub w: c_float,
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub padding0: c_float,
    pub padding1: c_float,
    pub padding2: c_float,
    pub padding3: c_float,
}

impl ::std::fmt::Debug for CoglQuaternion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglQuaternion @ {:?}", self as *const _))
            .field("w", &self.w)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

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
#[derive(Copy, Clone)]
pub struct CoglUserDataKey {
    pub unused: c_int,
}

impl ::std::fmt::Debug for CoglUserDataKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglUserDataKey @ {:?}", self as *const _))
            .field("unused", &self.unused)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoglVertexP2 {
    pub x: c_float,
    pub y: c_float,
}

impl ::std::fmt::Debug for CoglVertexP2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglVertexP2 @ {:?}", self as *const _))
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoglVertexP2C4 {
    pub x: c_float,
    pub y: c_float,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ::std::fmt::Debug for CoglVertexP2C4 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglVertexP2C4 @ {:?}", self as *const _))
            .field("x", &self.x)
            .field("y", &self.y)
            .field("r", &self.r)
            .field("g", &self.g)
            .field("b", &self.b)
            .field("a", &self.a)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoglVertexP2T2 {
    pub x: c_float,
    pub y: c_float,
    pub s: c_float,
    pub t: c_float,
}

impl ::std::fmt::Debug for CoglVertexP2T2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglVertexP2T2 @ {:?}", self as *const _))
            .field("x", &self.x)
            .field("y", &self.y)
            .field("s", &self.s)
            .field("t", &self.t)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoglVertexP2T2C4 {
    pub x: c_float,
    pub y: c_float,
    pub s: c_float,
    pub t: c_float,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ::std::fmt::Debug for CoglVertexP2T2C4 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglVertexP2T2C4 @ {:?}", self as *const _))
            .field("x", &self.x)
            .field("y", &self.y)
            .field("s", &self.s)
            .field("t", &self.t)
            .field("r", &self.r)
            .field("g", &self.g)
            .field("b", &self.b)
            .field("a", &self.a)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoglVertexP3 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

impl ::std::fmt::Debug for CoglVertexP3 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglVertexP3 @ {:?}", self as *const _))
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoglVertexP3C4 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ::std::fmt::Debug for CoglVertexP3C4 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglVertexP3C4 @ {:?}", self as *const _))
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("r", &self.r)
            .field("g", &self.g)
            .field("b", &self.b)
            .field("a", &self.a)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoglVertexP3T2 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub s: c_float,
    pub t: c_float,
}

impl ::std::fmt::Debug for CoglVertexP3T2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglVertexP3T2 @ {:?}", self as *const _))
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("s", &self.s)
            .field("t", &self.t)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoglVertexP3T2C4 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub s: c_float,
    pub t: c_float,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ::std::fmt::Debug for CoglVertexP3T2C4 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglVertexP3T2C4 @ {:?}", self as *const _))
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("s", &self.s)
            .field("t", &self.t)
            .field("r", &self.r)
            .field("g", &self.g)
            .field("b", &self.b)
            .field("a", &self.a)
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
pub struct _CoglEulerSizeCheck {
    _truncated_record_marker: c_void,
    // /*Ignored*/field compile_time_assert_CoglEuler_size has empty c:type
}

impl ::std::fmt::Debug for _CoglEulerSizeCheck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("_CoglEulerSizeCheck @ {:?}", self as *const _))
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
pub struct _CoglQuaternionSizeCheck {
    _truncated_record_marker: c_void,
    // /*Ignored*/field compile_time_assert_CoglQuaternion_size has empty c:type
}

impl ::std::fmt::Debug for _CoglQuaternionSizeCheck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "_CoglQuaternionSizeCheck @ {:?}",
            self as *const _
        ))
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
pub struct CoglAtlasTexture(c_void);

impl ::std::fmt::Debug for CoglAtlasTexture {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglAtlasTexture @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglAttribute(c_void);

impl ::std::fmt::Debug for CoglAttribute {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglAttribute @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglAttributeBuffer(c_void);

impl ::std::fmt::Debug for CoglAttributeBuffer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglAttributeBuffer @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglBitmap(c_void);

impl ::std::fmt::Debug for CoglBitmap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglBitmap @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglContext(c_void);

impl ::std::fmt::Debug for CoglContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglContext @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglDisplay(c_void);

impl ::std::fmt::Debug for CoglDisplay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglDisplay @ {:?}", self as *const _))
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
pub struct CoglFrameInfo(c_void);

impl ::std::fmt::Debug for CoglFrameInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglFrameInfo @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglGLES2Context(c_void);

impl ::std::fmt::Debug for CoglGLES2Context {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglGLES2Context @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglIndexBuffer(c_void);

impl ::std::fmt::Debug for CoglIndexBuffer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglIndexBuffer @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglIndices(c_void);

impl ::std::fmt::Debug for CoglIndices {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglIndices @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglMatrixStack(c_void);

impl ::std::fmt::Debug for CoglMatrixStack {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglMatrixStack @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglObject(c_void);

impl ::std::fmt::Debug for CoglObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglObject @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglOnscreen(c_void);

impl ::std::fmt::Debug for CoglOnscreen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglOnscreen @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglOnscreenTemplate(c_void);

impl ::std::fmt::Debug for CoglOnscreenTemplate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglOnscreenTemplate @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglOutput(c_void);

impl ::std::fmt::Debug for CoglOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglOutput @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglPipeline(c_void);

impl ::std::fmt::Debug for CoglPipeline {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglPipeline @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglPixelBuffer(c_void);

impl ::std::fmt::Debug for CoglPixelBuffer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglPixelBuffer @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglPrimitive(c_void);

impl ::std::fmt::Debug for CoglPrimitive {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglPrimitive @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglRenderer(c_void);

impl ::std::fmt::Debug for CoglRenderer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglRenderer @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglSnippet(c_void);

impl ::std::fmt::Debug for CoglSnippet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglSnippet @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglSubTexture(c_void);

impl ::std::fmt::Debug for CoglSubTexture {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglSubTexture @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglSwapChain(c_void);

impl ::std::fmt::Debug for CoglSwapChain {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglSwapChain @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglTexture2D(c_void);

impl ::std::fmt::Debug for CoglTexture2D {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglTexture2D @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglTexture2DSliced(c_void);

impl ::std::fmt::Debug for CoglTexture2DSliced {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglTexture2DSliced @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglTexture3D(c_void);

impl ::std::fmt::Debug for CoglTexture3D {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglTexture3D @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglTexturePixmapX11(c_void);

impl ::std::fmt::Debug for CoglTexturePixmapX11 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglTexturePixmapX11 @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct CoglTextureRectangle(c_void);

impl ::std::fmt::Debug for CoglTextureRectangle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("CoglTextureRectangle @ {:?}", self as *const _))
            .finish()
    }
}

// Interfaces
#[repr(C)]
pub struct CoglFramebuffer(c_void);

impl ::std::fmt::Debug for CoglFramebuffer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "CoglFramebuffer @ {:?}", self as *const _)
    }
}

#[repr(C)]
pub struct CoglTexture(c_void);

impl ::std::fmt::Debug for CoglTexture {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "CoglTexture @ {:?}", self as *const _)
    }
}

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
    pub fn cogl_renderer_error_quark() -> u32;

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
    // CoglDepthState
    //=========================================================================
    pub fn cogl_depth_state_get_range(
        state: *mut CoglDepthState,
        near_val: *mut c_float,
        far_val: *mut c_float,
    );
    pub fn cogl_depth_state_get_test_enabled(state: *mut CoglDepthState) -> CoglBool;
    pub fn cogl_depth_state_get_test_function(state: *mut CoglDepthState) -> CoglDepthTestFunction;
    pub fn cogl_depth_state_get_write_enabled(state: *mut CoglDepthState) -> CoglBool;
    pub fn cogl_depth_state_init(state: *mut CoglDepthState);
    pub fn cogl_depth_state_set_range(
        state: *mut CoglDepthState,
        near_val: c_float,
        far_val: c_float,
    );
    pub fn cogl_depth_state_set_test_enabled(state: *mut CoglDepthState, enable: CoglBool);
    pub fn cogl_depth_state_set_test_function(
        state: *mut CoglDepthState,
        function: CoglDepthTestFunction,
    );
    pub fn cogl_depth_state_set_write_enabled(state: *mut CoglDepthState, enable: CoglBool);

    //=========================================================================
    // CoglEuler
    //=========================================================================
    pub fn cogl_euler_get_gtype() -> GType;
    pub fn cogl_euler_copy(src: *const CoglEuler) -> *mut CoglEuler;
    pub fn cogl_euler_free(euler: *mut CoglEuler);
    pub fn cogl_euler_init(euler: *mut CoglEuler, heading: c_float, pitch: c_float, roll: c_float);
    pub fn cogl_euler_init_from_matrix(euler: *mut CoglEuler, matrix: *const CoglMatrix);
    pub fn cogl_euler_init_from_quaternion(
        euler: *mut CoglEuler,
        quaternion: *const CoglQuaternion,
    );
    pub fn cogl_euler_equal(v1: *mut c_void, v2: *mut c_void) -> CoglBool;

    //=========================================================================
    // CoglFenceClosure
    //=========================================================================
    pub fn cogl_fence_closure_get_user_data(closure: *mut CoglFenceClosure) -> *mut c_void;

    //=========================================================================
    // CoglFrameClosure
    //=========================================================================
    pub fn cogl_frame_closure_get_gtype() -> GType;

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
    pub fn cogl_matrix_init_from_euler(matrix: *mut CoglMatrix, euler: *const CoglEuler);
    pub fn cogl_matrix_init_from_quaternion(
        matrix: *mut CoglMatrix,
        quaternion: *const CoglQuaternion,
    );
    pub fn cogl_matrix_init_identity(matrix: *mut CoglMatrix);
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
    pub fn cogl_matrix_orthographic(
        matrix: *mut CoglMatrix,
        x_1: c_float,
        y_1: c_float,
        x_2: c_float,
        y_2: c_float,
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
    pub fn cogl_matrix_project_points(
        matrix: *const CoglMatrix,
        n_components: c_int,
        stride_in: size_t,
        points_in: *mut c_void,
        stride_out: size_t,
        points_out: *mut c_void,
        n_points: c_int,
    );
    pub fn cogl_matrix_rotate(
        matrix: *mut CoglMatrix,
        angle: c_float,
        x: c_float,
        y: c_float,
        z: c_float,
    );
    pub fn cogl_matrix_rotate_euler(matrix: *mut CoglMatrix, euler: *const CoglEuler);
    pub fn cogl_matrix_rotate_quaternion(
        matrix: *mut CoglMatrix,
        quaternion: *const CoglQuaternion,
    );
    pub fn cogl_matrix_scale(matrix: *mut CoglMatrix, sx: c_float, sy: c_float, sz: c_float);
    pub fn cogl_matrix_transform_point(
        matrix: *const CoglMatrix,
        x: *mut c_float,
        y: *mut c_float,
        z: *mut c_float,
        w: *mut c_float,
    );
    pub fn cogl_matrix_transform_points(
        matrix: *const CoglMatrix,
        n_components: c_int,
        stride_in: size_t,
        points_in: *mut c_void,
        stride_out: size_t,
        points_out: *mut c_void,
        n_points: c_int,
    );
    pub fn cogl_matrix_translate(matrix: *mut CoglMatrix, x: c_float, y: c_float, z: c_float);
    pub fn cogl_matrix_transpose(matrix: *mut CoglMatrix);
    pub fn cogl_matrix_view_2d_in_frustum(
        matrix: *mut CoglMatrix,
        left: c_float,
        right: c_float,
        bottom: c_float,
        top: c_float,
        z_near: c_float,
        z_2d: c_float,
        width_2d: c_float,
        height_2d: c_float,
    );
    pub fn cogl_matrix_view_2d_in_perspective(
        matrix: *mut CoglMatrix,
        fov_y: c_float,
        aspect: c_float,
        z_near: c_float,
        z_2d: c_float,
        width_2d: c_float,
        height_2d: c_float,
    );
    pub fn cogl_matrix_equal(v1: *mut c_void, v2: *mut c_void) -> CoglBool;

    //=========================================================================
    // CoglMatrixEntry
    //=========================================================================
    pub fn cogl_matrix_entry_get_gtype() -> GType;
    pub fn cogl_matrix_entry_calculate_translation(
        entry0: *mut CoglMatrixEntry,
        entry1: *mut CoglMatrixEntry,
        x: *mut c_float,
        y: *mut c_float,
        z: *mut c_float,
    ) -> CoglBool;
    pub fn cogl_matrix_entry_equal(
        entry0: *mut CoglMatrixEntry,
        entry1: *mut CoglMatrixEntry,
    ) -> CoglBool;
    pub fn cogl_matrix_entry_get(
        entry: *mut CoglMatrixEntry,
        matrix: *mut CoglMatrix,
    ) -> *mut CoglMatrix;
    pub fn cogl_matrix_entry_is_identity(entry: *mut CoglMatrixEntry) -> CoglBool;
    pub fn cogl_matrix_entry_ref(entry: *mut CoglMatrixEntry) -> *mut CoglMatrixEntry;
    pub fn cogl_matrix_entry_unref(entry: *mut CoglMatrixEntry);

    //=========================================================================
    // CoglOnscreenDirtyClosure
    //=========================================================================
    pub fn cogl_onscreen_dirty_closure_get_gtype() -> GType;

    //=========================================================================
    // CoglOnscreenResizeClosure
    //=========================================================================
    pub fn cogl_onscreen_resize_closure_get_gtype() -> GType;

    //=========================================================================
    // CoglQuaternion
    //=========================================================================
    pub fn cogl_quaternion_get_gtype() -> GType;
    pub fn cogl_quaternion_copy(src: *const CoglQuaternion) -> *mut CoglQuaternion;
    pub fn cogl_quaternion_dot_product(
        a: *const CoglQuaternion,
        b: *const CoglQuaternion,
    ) -> c_float;
    pub fn cogl_quaternion_free(quaternion: *mut CoglQuaternion);
    pub fn cogl_quaternion_get_rotation_angle(quaternion: *const CoglQuaternion) -> c_float;
    pub fn cogl_quaternion_get_rotation_axis(
        quaternion: *const CoglQuaternion,
        vector3: *mut c_float,
    );
    pub fn cogl_quaternion_init(
        quaternion: *mut CoglQuaternion,
        angle: c_float,
        x: c_float,
        y: c_float,
        z: c_float,
    );
    pub fn cogl_quaternion_init_from_angle_vector(
        quaternion: *mut CoglQuaternion,
        angle: c_float,
        axis3f: *const c_float,
    );
    pub fn cogl_quaternion_init_from_array(quaternion: *mut CoglQuaternion, array: *const c_float);
    pub fn cogl_quaternion_init_from_euler(
        quaternion: *mut CoglQuaternion,
        euler: *const CoglEuler,
    );
    pub fn cogl_quaternion_init_from_matrix(
        quaternion: *mut CoglQuaternion,
        matrix: *const CoglMatrix,
    );
    pub fn cogl_quaternion_init_from_quaternion(
        quaternion: *mut CoglQuaternion,
        src: *mut CoglQuaternion,
    );
    pub fn cogl_quaternion_init_from_x_rotation(quaternion: *mut CoglQuaternion, angle: c_float);
    pub fn cogl_quaternion_init_from_y_rotation(quaternion: *mut CoglQuaternion, angle: c_float);
    pub fn cogl_quaternion_init_from_z_rotation(quaternion: *mut CoglQuaternion, angle: c_float);
    pub fn cogl_quaternion_init_identity(quaternion: *mut CoglQuaternion);
    pub fn cogl_quaternion_invert(quaternion: *mut CoglQuaternion);
    pub fn cogl_quaternion_multiply(
        result: *mut CoglQuaternion,
        left: *const CoglQuaternion,
        right: *const CoglQuaternion,
    );
    pub fn cogl_quaternion_nlerp(
        result: *mut CoglQuaternion,
        a: *const CoglQuaternion,
        b: *const CoglQuaternion,
        t: c_float,
    );
    pub fn cogl_quaternion_normalize(quaternion: *mut CoglQuaternion);
    pub fn cogl_quaternion_pow(quaternion: *mut CoglQuaternion, exponent: c_float);
    pub fn cogl_quaternion_slerp(
        result: *mut CoglQuaternion,
        a: *const CoglQuaternion,
        b: *const CoglQuaternion,
        t: c_float,
    );
    pub fn cogl_quaternion_squad(
        result: *mut CoglQuaternion,
        prev: *const CoglQuaternion,
        a: *const CoglQuaternion,
        b: *const CoglQuaternion,
        next: *const CoglQuaternion,
        t: c_float,
    );
    pub fn cogl_quaternion_equal(v1: *mut c_void, v2: *mut c_void) -> CoglBool;

    //=========================================================================
    // CoglAtlasTexture
    //=========================================================================
    pub fn cogl_atlas_texture_get_gtype() -> GType;
    pub fn cogl_atlas_texture_new_from_bitmap(bmp: *mut CoglBitmap) -> *mut CoglAtlasTexture;
    pub fn cogl_atlas_texture_new_from_data(
        ctx: *mut CoglContext,
        width: c_int,
        height: c_int,
        format: CoglPixelFormat,
        rowstride: c_int,
        data: *const u8,
        error: *mut *mut glib::GError,
    ) -> *mut CoglAtlasTexture;
    pub fn cogl_atlas_texture_new_from_file(
        ctx: *mut CoglContext,
        filename: *const c_char,
        error: *mut *mut glib::GError,
    ) -> *mut CoglAtlasTexture;
    pub fn cogl_atlas_texture_new_with_size(
        ctx: *mut CoglContext,
        width: c_int,
        height: c_int,
    ) -> *mut CoglAtlasTexture;

    //=========================================================================
    // CoglAttribute
    //=========================================================================
    pub fn cogl_attribute_get_gtype() -> GType;
    pub fn cogl_attribute_new(
        attribute_buffer: *mut CoglAttributeBuffer,
        name: *const c_char,
        stride: size_t,
        offset: size_t,
        components: c_int,
        type_: CoglAttributeType,
    ) -> *mut CoglAttribute;
    pub fn cogl_attribute_new_const_1f(
        context: *mut CoglContext,
        name: *const c_char,
        value: c_float,
    ) -> *mut CoglAttribute;
    pub fn cogl_attribute_new_const_2f(
        context: *mut CoglContext,
        name: *const c_char,
        component0: c_float,
        component1: c_float,
    ) -> *mut CoglAttribute;
    pub fn cogl_attribute_new_const_2fv(
        context: *mut CoglContext,
        name: *const c_char,
        value: *const c_float,
    ) -> *mut CoglAttribute;
    pub fn cogl_attribute_new_const_2x2fv(
        context: *mut CoglContext,
        name: *const c_char,
        matrix2x2: *const c_float,
        transpose: CoglBool,
    ) -> *mut CoglAttribute;
    pub fn cogl_attribute_new_const_3f(
        context: *mut CoglContext,
        name: *const c_char,
        component0: c_float,
        component1: c_float,
        component2: c_float,
    ) -> *mut CoglAttribute;
    pub fn cogl_attribute_new_const_3fv(
        context: *mut CoglContext,
        name: *const c_char,
        value: *const c_float,
    ) -> *mut CoglAttribute;
    pub fn cogl_attribute_new_const_3x3fv(
        context: *mut CoglContext,
        name: *const c_char,
        matrix3x3: *const c_float,
        transpose: CoglBool,
    ) -> *mut CoglAttribute;
    pub fn cogl_attribute_new_const_4f(
        context: *mut CoglContext,
        name: *const c_char,
        component0: c_float,
        component1: c_float,
        component2: c_float,
        component3: c_float,
    ) -> *mut CoglAttribute;
    pub fn cogl_attribute_new_const_4fv(
        context: *mut CoglContext,
        name: *const c_char,
        value: *const c_float,
    ) -> *mut CoglAttribute;
    pub fn cogl_attribute_new_const_4x4fv(
        context: *mut CoglContext,
        name: *const c_char,
        matrix4x4: *const c_float,
        transpose: CoglBool,
    ) -> *mut CoglAttribute;
    pub fn cogl_attribute_get_buffer(attribute: *mut CoglAttribute) -> *mut CoglAttributeBuffer;
    pub fn cogl_attribute_get_normalized(attribute: *mut CoglAttribute) -> CoglBool;
    pub fn cogl_attribute_set_buffer(
        attribute: *mut CoglAttribute,
        attribute_buffer: *mut CoglAttributeBuffer,
    );
    pub fn cogl_attribute_set_normalized(attribute: *mut CoglAttribute, normalized: CoglBool);

    //=========================================================================
    // CoglAttributeBuffer
    //=========================================================================
    pub fn cogl_attribute_buffer_get_gtype() -> GType;
    pub fn cogl_attribute_buffer_new(
        context: *mut CoglContext,
        bytes: size_t,
        data: *mut c_void,
    ) -> *mut CoglAttributeBuffer;
    pub fn cogl_attribute_buffer_new_with_size(
        context: *mut CoglContext,
        bytes: size_t,
    ) -> *mut CoglAttributeBuffer;

    //=========================================================================
    // CoglBitmap
    //=========================================================================
    pub fn cogl_bitmap_get_gtype() -> GType;
    pub fn cogl_bitmap_new_for_data(
        context: *mut CoglContext,
        width: c_int,
        height: c_int,
        format: CoglPixelFormat,
        rowstride: c_int,
        data: *mut u8,
    ) -> *mut CoglBitmap;
    pub fn cogl_bitmap_new_from_buffer(
        buffer: *mut CoglBuffer,
        format: CoglPixelFormat,
        width: c_int,
        height: c_int,
        rowstride: c_int,
        offset: c_int,
    ) -> *mut CoglBitmap;
    pub fn cogl_bitmap_new_from_file(
        filename: *const c_char,
        error: *mut *mut glib::GError,
    ) -> *mut CoglBitmap;
    pub fn cogl_bitmap_new_with_size(
        context: *mut CoglContext,
        width: c_uint,
        height: c_uint,
        format: CoglPixelFormat,
    ) -> *mut CoglBitmap;
    pub fn cogl_bitmap_get_size_from_file(
        filename: *const c_char,
        width: *mut c_int,
        height: *mut c_int,
    ) -> CoglBool;
    pub fn cogl_bitmap_get_buffer(bitmap: *mut CoglBitmap) -> *mut CoglPixelBuffer;
    pub fn cogl_bitmap_get_format(bitmap: *mut CoglBitmap) -> CoglPixelFormat;
    pub fn cogl_bitmap_get_height(bitmap: *mut CoglBitmap) -> c_int;
    pub fn cogl_bitmap_get_rowstride(bitmap: *mut CoglBitmap) -> c_int;
    pub fn cogl_bitmap_get_width(bitmap: *mut CoglBitmap) -> c_int;

    //=========================================================================
    // CoglContext
    //=========================================================================
    pub fn cogl_context_get_gtype() -> GType;
    pub fn cogl_context_new(
        display: *mut CoglDisplay,
        error: *mut *mut glib::GError,
    ) -> *mut CoglContext;
    pub fn cogl_context_get_display(context: *mut CoglContext) -> *mut CoglDisplay;
    pub fn cogl_context_get_renderer(context: *mut CoglContext) -> *mut CoglRenderer;

    //=========================================================================
    // CoglDisplay
    //=========================================================================
    pub fn cogl_display_get_gtype() -> GType;
    pub fn cogl_display_new(
        renderer: *mut CoglRenderer,
        onscreen_template: *mut CoglOnscreenTemplate,
    ) -> *mut CoglDisplay;
    pub fn cogl_display_get_renderer(display: *mut CoglDisplay) -> *mut CoglRenderer;
    pub fn cogl_display_set_onscreen_template(
        display: *mut CoglDisplay,
        onscreen_template: *mut CoglOnscreenTemplate,
    );
    pub fn cogl_display_setup(display: *mut CoglDisplay, error: *mut *mut glib::GError)
        -> CoglBool;

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
    // CoglFrameInfo
    //=========================================================================
    pub fn cogl_frame_info_get_gtype() -> GType;
    pub fn cogl_frame_info_get_frame_counter(info: *mut CoglFrameInfo) -> i64;
    pub fn cogl_frame_info_get_output(info: *mut CoglFrameInfo) -> *mut CoglOutput;
    pub fn cogl_frame_info_get_presentation_time(info: *mut CoglFrameInfo) -> i64;
    pub fn cogl_frame_info_get_refresh_rate(info: *mut CoglFrameInfo) -> c_float;

    //=========================================================================
    // CoglGLES2Context
    //=========================================================================
    pub fn cogl_gles2_context_get_gtype() -> GType;
    pub fn cogl_gles2_context_new(
        ctx: *mut CoglContext,
        error: *mut *mut glib::GError,
    ) -> *mut CoglGLES2Context;
    //pub fn cogl_gles2_context_get_vtable(gles2_ctx: *mut CoglGLES2Context) -> /*Ignored*/*const CoglGLES2Vtable;

    //=========================================================================
    // CoglIndexBuffer
    //=========================================================================
    pub fn cogl_index_buffer_get_gtype() -> GType;
    pub fn cogl_index_buffer_new(context: *mut CoglContext, bytes: size_t) -> *mut CoglIndexBuffer;

    //=========================================================================
    // CoglIndices
    //=========================================================================
    pub fn cogl_indices_get_gtype() -> GType;
    pub fn cogl_indices_new(
        context: *mut CoglContext,
        type_: CoglIndicesType,
        indices_data: *mut c_void,
        n_indices: c_int,
    ) -> *mut CoglIndices;
    pub fn cogl_indices_new_for_buffer(
        type_: CoglIndicesType,
        buffer: *mut CoglIndexBuffer,
        offset: size_t,
    ) -> *mut CoglIndices;
    pub fn cogl_indices_get_buffer(indices: *mut CoglIndices) -> *mut CoglIndexBuffer;
    pub fn cogl_indices_get_offset(indices: *mut CoglIndices) -> size_t;
    pub fn cogl_indices_get_type(indices: *mut CoglIndices) -> CoglIndicesType;
    pub fn cogl_indices_set_offset(indices: *mut CoglIndices, offset: size_t);

    //=========================================================================
    // CoglMatrixStack
    //=========================================================================
    pub fn cogl_matrix_stack_get_gtype() -> GType;
    pub fn cogl_matrix_stack_new(ctx: *mut CoglContext) -> *mut CoglMatrixStack;
    pub fn cogl_matrix_stack_frustum(
        stack: *mut CoglMatrixStack,
        left: c_float,
        right: c_float,
        bottom: c_float,
        top: c_float,
        z_near: c_float,
        z_far: c_float,
    );
    pub fn cogl_matrix_stack_get(
        stack: *mut CoglMatrixStack,
        matrix: *mut CoglMatrix,
    ) -> *mut CoglMatrix;
    pub fn cogl_matrix_stack_get_entry(stack: *mut CoglMatrixStack) -> *mut CoglMatrixEntry;
    pub fn cogl_matrix_stack_get_inverse(
        stack: *mut CoglMatrixStack,
        inverse: *mut CoglMatrix,
    ) -> CoglBool;
    pub fn cogl_matrix_stack_load_identity(stack: *mut CoglMatrixStack);
    pub fn cogl_matrix_stack_multiply(stack: *mut CoglMatrixStack, matrix: *const CoglMatrix);
    pub fn cogl_matrix_stack_orthographic(
        stack: *mut CoglMatrixStack,
        x_1: c_float,
        y_1: c_float,
        x_2: c_float,
        y_2: c_float,
        near: c_float,
        far: c_float,
    );
    pub fn cogl_matrix_stack_perspective(
        stack: *mut CoglMatrixStack,
        fov_y: c_float,
        aspect: c_float,
        z_near: c_float,
        z_far: c_float,
    );
    pub fn cogl_matrix_stack_pop(stack: *mut CoglMatrixStack);
    pub fn cogl_matrix_stack_push(stack: *mut CoglMatrixStack);
    pub fn cogl_matrix_stack_rotate(
        stack: *mut CoglMatrixStack,
        angle: c_float,
        x: c_float,
        y: c_float,
        z: c_float,
    );
    pub fn cogl_matrix_stack_rotate_euler(stack: *mut CoglMatrixStack, euler: *const CoglEuler);
    pub fn cogl_matrix_stack_rotate_quaternion(
        stack: *mut CoglMatrixStack,
        quaternion: *const CoglQuaternion,
    );
    pub fn cogl_matrix_stack_scale(stack: *mut CoglMatrixStack, x: c_float, y: c_float, z: c_float);
    pub fn cogl_matrix_stack_set(stack: *mut CoglMatrixStack, matrix: *const CoglMatrix);
    pub fn cogl_matrix_stack_translate(
        stack: *mut CoglMatrixStack,
        x: c_float,
        y: c_float,
        z: c_float,
    );

    //=========================================================================
    // CoglObject
    //=========================================================================
    pub fn cogl_object_get_gtype() -> GType;
    pub fn cogl_object_ref(object: *mut c_void) -> *mut c_void;
    pub fn cogl_object_unref(object: *mut c_void);
    pub fn cogl_object_value_get_object(value: *const gobject::GValue) -> gpointer;
    pub fn cogl_object_value_set_object(value: *mut gobject::GValue, object: gpointer);
    pub fn cogl_object_get_user_data(
        object: *mut CoglObject,
        key: *mut CoglUserDataKey,
    ) -> *mut c_void;
    pub fn cogl_object_set_user_data(
        object: *mut CoglObject,
        key: *mut CoglUserDataKey,
        user_data: *mut c_void,
        destroy: CoglUserDataDestroyCallback,
    );

    //=========================================================================
    // CoglOffscreen
    //=========================================================================
    pub fn cogl_offscreen_get_gtype() -> GType;
    pub fn cogl_offscreen_new_to_texture(texture: *mut CoglTexture) -> *mut CoglOffscreen;
    pub fn cogl_offscreen_new_with_texture(texture: *mut CoglTexture) -> *mut CoglOffscreen;
    pub fn cogl_offscreen_ref(offscreen: *mut c_void) -> *mut c_void;
    pub fn cogl_offscreen_unref(offscreen: *mut c_void);

    //=========================================================================
    // CoglOnscreen
    //=========================================================================
    pub fn cogl_onscreen_get_gtype() -> GType;
    pub fn cogl_onscreen_new(
        context: *mut CoglContext,
        width: c_int,
        height: c_int,
    ) -> *mut CoglOnscreen;
    pub fn cogl_onscreen_add_dirty_callback(
        onscreen: *mut CoglOnscreen,
        callback: CoglOnscreenDirtyCallback,
        user_data: *mut c_void,
        destroy: CoglUserDataDestroyCallback,
    ) -> *mut CoglOnscreenDirtyClosure;
    pub fn cogl_onscreen_add_frame_callback(
        onscreen: *mut CoglOnscreen,
        callback: CoglFrameCallback,
        user_data: *mut c_void,
        destroy: CoglUserDataDestroyCallback,
    ) -> *mut CoglFrameClosure;
    pub fn cogl_onscreen_add_resize_callback(
        onscreen: *mut CoglOnscreen,
        callback: CoglOnscreenResizeCallback,
        user_data: *mut c_void,
        destroy: CoglUserDataDestroyCallback,
    ) -> *mut CoglOnscreenResizeClosure;
    pub fn cogl_onscreen_add_swap_buffers_callback(
        onscreen: *mut CoglOnscreen,
        callback: CoglSwapBuffersNotify,
        user_data: *mut c_void,
    ) -> c_uint;
    pub fn cogl_onscreen_get_buffer_age(onscreen: *mut CoglOnscreen) -> c_int;
    pub fn cogl_onscreen_get_frame_counter(onscreen: *mut CoglOnscreen) -> i64;
    pub fn cogl_onscreen_get_resizable(onscreen: *mut CoglOnscreen) -> CoglBool;
    pub fn cogl_onscreen_hide(onscreen: *mut CoglOnscreen);
    pub fn cogl_onscreen_remove_dirty_callback(
        onscreen: *mut CoglOnscreen,
        closure: *mut CoglOnscreenDirtyClosure,
    );
    pub fn cogl_onscreen_remove_frame_callback(
        onscreen: *mut CoglOnscreen,
        closure: *mut CoglFrameClosure,
    );
    pub fn cogl_onscreen_remove_resize_callback(
        onscreen: *mut CoglOnscreen,
        closure: *mut CoglOnscreenResizeClosure,
    );
    pub fn cogl_onscreen_remove_swap_buffers_callback(onscreen: *mut CoglOnscreen, id: c_uint);
    pub fn cogl_onscreen_set_resizable(onscreen: *mut CoglOnscreen, resizable: CoglBool);
    pub fn cogl_onscreen_set_swap_throttled(onscreen: *mut CoglOnscreen, throttled: CoglBool);
    pub fn cogl_onscreen_show(onscreen: *mut CoglOnscreen);
    pub fn cogl_onscreen_swap_buffers(onscreen: *mut CoglOnscreen);
    pub fn cogl_onscreen_swap_buffers_with_damage(
        onscreen: *mut CoglOnscreen,
        rectangles: *const c_int,
        n_rectangles: c_int,
    );
    pub fn cogl_onscreen_swap_region(
        onscreen: *mut CoglOnscreen,
        rectangles: *const c_int,
        n_rectangles: c_int,
    );

    //=========================================================================
    // CoglOnscreenTemplate
    //=========================================================================
    pub fn cogl_onscreen_template_get_gtype() -> GType;
    pub fn cogl_onscreen_template_new(swap_chain: *mut CoglSwapChain) -> *mut CoglOnscreenTemplate;
    pub fn cogl_onscreen_template_set_samples_per_pixel(
        onscreen_template: *mut CoglOnscreenTemplate,
        n: c_int,
    );
    pub fn cogl_onscreen_template_set_stereo_enabled(
        onscreen_template: *mut CoglOnscreenTemplate,
        enabled: CoglBool,
    );
    pub fn cogl_onscreen_template_set_swap_throttled(
        onscreen_template: *mut CoglOnscreenTemplate,
        throttled: CoglBool,
    );

    //=========================================================================
    // CoglOutput
    //=========================================================================
    pub fn cogl_output_get_gtype() -> GType;
    pub fn cogl_output_get_height(output: *mut CoglOutput) -> c_int;
    pub fn cogl_output_get_mm_height(output: *mut CoglOutput) -> c_int;
    pub fn cogl_output_get_mm_width(output: *mut CoglOutput) -> c_int;
    pub fn cogl_output_get_refresh_rate(output: *mut CoglOutput) -> c_float;
    pub fn cogl_output_get_subpixel_order(output: *mut CoglOutput) -> CoglSubpixelOrder;
    pub fn cogl_output_get_width(output: *mut CoglOutput) -> c_int;
    pub fn cogl_output_get_x(output: *mut CoglOutput) -> c_int;
    pub fn cogl_output_get_y(output: *mut CoglOutput) -> c_int;

    //=========================================================================
    // CoglPipeline
    //=========================================================================
    pub fn cogl_pipeline_get_gtype() -> GType;
    pub fn cogl_pipeline_new(context: *mut CoglContext) -> *mut CoglPipeline;
    pub fn cogl_pipeline_add_layer_snippet(
        pipeline: *mut CoglPipeline,
        layer: c_int,
        snippet: *mut CoglSnippet,
    );
    pub fn cogl_pipeline_add_snippet(pipeline: *mut CoglPipeline, snippet: *mut CoglSnippet);
    pub fn cogl_pipeline_copy(source: *mut CoglPipeline) -> *mut CoglPipeline;
    pub fn cogl_pipeline_foreach_layer(
        pipeline: *mut CoglPipeline,
        callback: CoglPipelineLayerCallback,
        user_data: *mut c_void,
    );
    pub fn cogl_pipeline_get_alpha_test_function(
        pipeline: *mut CoglPipeline,
    ) -> CoglPipelineAlphaFunc;
    pub fn cogl_pipeline_get_alpha_test_reference(pipeline: *mut CoglPipeline) -> c_float;
    pub fn cogl_pipeline_get_ambient(pipeline: *mut CoglPipeline, ambient: *mut CoglColor);
    pub fn cogl_pipeline_get_color(pipeline: *mut CoglPipeline, color: *mut CoglColor);
    pub fn cogl_pipeline_get_color_mask(pipeline: *mut CoglPipeline) -> CoglColorMask;
    pub fn cogl_pipeline_get_cull_face_mode(
        pipeline: *mut CoglPipeline,
    ) -> CoglPipelineCullFaceMode;
    pub fn cogl_pipeline_get_depth_state(
        pipeline: *mut CoglPipeline,
        state_out: *mut CoglDepthState,
    );
    pub fn cogl_pipeline_get_diffuse(pipeline: *mut CoglPipeline, diffuse: *mut CoglColor);
    pub fn cogl_pipeline_get_emission(pipeline: *mut CoglPipeline, emission: *mut CoglColor);
    pub fn cogl_pipeline_get_front_face_winding(pipeline: *mut CoglPipeline) -> CoglWinding;
    pub fn cogl_pipeline_get_layer_mag_filter(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
    ) -> CoglPipelineFilter;
    pub fn cogl_pipeline_get_layer_min_filter(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
    ) -> CoglPipelineFilter;
    pub fn cogl_pipeline_get_layer_point_sprite_coords_enabled(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
    ) -> CoglBool;
    pub fn cogl_pipeline_get_layer_texture(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
    ) -> *mut CoglTexture;
    pub fn cogl_pipeline_get_layer_wrap_mode_p(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
    ) -> CoglPipelineWrapMode;
    pub fn cogl_pipeline_get_layer_wrap_mode_s(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
    ) -> CoglPipelineWrapMode;
    pub fn cogl_pipeline_get_layer_wrap_mode_t(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
    ) -> CoglPipelineWrapMode;
    pub fn cogl_pipeline_get_n_layers(pipeline: *mut CoglPipeline) -> c_int;
    pub fn cogl_pipeline_get_per_vertex_point_size(pipeline: *mut CoglPipeline) -> CoglBool;
    pub fn cogl_pipeline_get_point_size(pipeline: *mut CoglPipeline) -> c_float;
    pub fn cogl_pipeline_get_shininess(pipeline: *mut CoglPipeline) -> c_float;
    pub fn cogl_pipeline_get_specular(pipeline: *mut CoglPipeline, specular: *mut CoglColor);
    pub fn cogl_pipeline_get_uniform_location(
        pipeline: *mut CoglPipeline,
        uniform_name: *const c_char,
    ) -> c_int;
    pub fn cogl_pipeline_get_user_program(pipeline: *mut CoglPipeline) -> CoglHandle;
    pub fn cogl_pipeline_remove_layer(pipeline: *mut CoglPipeline, layer_index: c_int);
    pub fn cogl_pipeline_set_alpha_test_function(
        pipeline: *mut CoglPipeline,
        alpha_func: CoglPipelineAlphaFunc,
        alpha_reference: c_float,
    );
    pub fn cogl_pipeline_set_ambient(pipeline: *mut CoglPipeline, ambient: *const CoglColor);
    pub fn cogl_pipeline_set_ambient_and_diffuse(
        pipeline: *mut CoglPipeline,
        color: *const CoglColor,
    );
    pub fn cogl_pipeline_set_blend(
        pipeline: *mut CoglPipeline,
        blend_string: *const c_char,
        error: *mut *mut glib::GError,
    ) -> CoglBool;
    pub fn cogl_pipeline_set_blend_constant(
        pipeline: *mut CoglPipeline,
        constant_color: *const CoglColor,
    );
    pub fn cogl_pipeline_set_color(pipeline: *mut CoglPipeline, color: *const CoglColor);
    pub fn cogl_pipeline_set_color4f(
        pipeline: *mut CoglPipeline,
        red: c_float,
        green: c_float,
        blue: c_float,
        alpha: c_float,
    );
    pub fn cogl_pipeline_set_color4ub(
        pipeline: *mut CoglPipeline,
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
    );
    pub fn cogl_pipeline_set_color_mask(pipeline: *mut CoglPipeline, color_mask: CoglColorMask);
    pub fn cogl_pipeline_set_cull_face_mode(
        pipeline: *mut CoglPipeline,
        cull_face_mode: CoglPipelineCullFaceMode,
    );
    pub fn cogl_pipeline_set_depth_state(
        pipeline: *mut CoglPipeline,
        state: *const CoglDepthState,
        error: *mut *mut glib::GError,
    ) -> CoglBool;
    pub fn cogl_pipeline_set_diffuse(pipeline: *mut CoglPipeline, diffuse: *const CoglColor);
    pub fn cogl_pipeline_set_emission(pipeline: *mut CoglPipeline, emission: *const CoglColor);
    pub fn cogl_pipeline_set_front_face_winding(
        pipeline: *mut CoglPipeline,
        front_winding: CoglWinding,
    );
    pub fn cogl_pipeline_set_layer_combine(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
        blend_string: *const c_char,
        error: *mut *mut glib::GError,
    ) -> CoglBool;
    pub fn cogl_pipeline_set_layer_combine_constant(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
        constant: *const CoglColor,
    );
    pub fn cogl_pipeline_set_layer_filters(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
        min_filter: CoglPipelineFilter,
        mag_filter: CoglPipelineFilter,
    );
    pub fn cogl_pipeline_set_layer_matrix(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
        matrix: *const CoglMatrix,
    );
    pub fn cogl_pipeline_set_layer_null_texture(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
        texture_type: CoglTextureType,
    );
    pub fn cogl_pipeline_set_layer_point_sprite_coords_enabled(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
        enable: CoglBool,
        error: *mut *mut glib::GError,
    ) -> CoglBool;
    pub fn cogl_pipeline_set_layer_texture(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
        texture: *mut CoglTexture,
    );
    pub fn cogl_pipeline_set_layer_wrap_mode(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
        mode: CoglPipelineWrapMode,
    );
    pub fn cogl_pipeline_set_layer_wrap_mode_p(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
        mode: CoglPipelineWrapMode,
    );
    pub fn cogl_pipeline_set_layer_wrap_mode_s(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
        mode: CoglPipelineWrapMode,
    );
    pub fn cogl_pipeline_set_layer_wrap_mode_t(
        pipeline: *mut CoglPipeline,
        layer_index: c_int,
        mode: CoglPipelineWrapMode,
    );
    pub fn cogl_pipeline_set_per_vertex_point_size(
        pipeline: *mut CoglPipeline,
        enable: CoglBool,
        error: *mut *mut glib::GError,
    ) -> CoglBool;
    pub fn cogl_pipeline_set_point_size(pipeline: *mut CoglPipeline, point_size: c_float);
    pub fn cogl_pipeline_set_shininess(pipeline: *mut CoglPipeline, shininess: c_float);
    pub fn cogl_pipeline_set_specular(pipeline: *mut CoglPipeline, specular: *const CoglColor);
    pub fn cogl_pipeline_set_uniform_1f(
        pipeline: *mut CoglPipeline,
        uniform_location: c_int,
        value: c_float,
    );
    pub fn cogl_pipeline_set_uniform_1i(
        pipeline: *mut CoglPipeline,
        uniform_location: c_int,
        value: c_int,
    );
    pub fn cogl_pipeline_set_uniform_float(
        pipeline: *mut CoglPipeline,
        uniform_location: c_int,
        n_components: c_int,
        count: c_int,
        value: *const c_float,
    );
    pub fn cogl_pipeline_set_uniform_int(
        pipeline: *mut CoglPipeline,
        uniform_location: c_int,
        n_components: c_int,
        count: c_int,
        value: *const c_int,
    );
    pub fn cogl_pipeline_set_uniform_matrix(
        pipeline: *mut CoglPipeline,
        uniform_location: c_int,
        dimensions: c_int,
        count: c_int,
        transpose: CoglBool,
        value: *const c_float,
    );
    pub fn cogl_pipeline_set_user_program(pipeline: *mut CoglPipeline, program: CoglHandle);

    //=========================================================================
    // CoglPixelBuffer
    //=========================================================================
    pub fn cogl_pixel_buffer_get_gtype() -> GType;
    pub fn cogl_pixel_buffer_new(
        context: *mut CoglContext,
        size: size_t,
        data: *mut c_void,
    ) -> *mut CoglPixelBuffer;

    //=========================================================================
    // CoglPrimitive
    //=========================================================================
    pub fn cogl_primitive_get_gtype() -> GType;
    pub fn cogl_primitive_new(mode: CoglVerticesMode, n_vertices: c_int, ...)
        -> *mut CoglPrimitive;
    pub fn cogl_primitive_new_p2(
        context: *mut CoglContext,
        mode: CoglVerticesMode,
        n_vertices: c_int,
        data: *const CoglVertexP2,
    ) -> *mut CoglPrimitive;
    pub fn cogl_primitive_new_p2c4(
        context: *mut CoglContext,
        mode: CoglVerticesMode,
        n_vertices: c_int,
        data: *const CoglVertexP2C4,
    ) -> *mut CoglPrimitive;
    pub fn cogl_primitive_new_p2t2(
        context: *mut CoglContext,
        mode: CoglVerticesMode,
        n_vertices: c_int,
        data: *const CoglVertexP2T2,
    ) -> *mut CoglPrimitive;
    pub fn cogl_primitive_new_p2t2c4(
        context: *mut CoglContext,
        mode: CoglVerticesMode,
        n_vertices: c_int,
        data: *const CoglVertexP2T2C4,
    ) -> *mut CoglPrimitive;
    pub fn cogl_primitive_new_p3(
        context: *mut CoglContext,
        mode: CoglVerticesMode,
        n_vertices: c_int,
        data: *const CoglVertexP3,
    ) -> *mut CoglPrimitive;
    pub fn cogl_primitive_new_p3c4(
        context: *mut CoglContext,
        mode: CoglVerticesMode,
        n_vertices: c_int,
        data: *const CoglVertexP3C4,
    ) -> *mut CoglPrimitive;
    pub fn cogl_primitive_new_p3t2(
        context: *mut CoglContext,
        mode: CoglVerticesMode,
        n_vertices: c_int,
        data: *const CoglVertexP3T2,
    ) -> *mut CoglPrimitive;
    pub fn cogl_primitive_new_p3t2c4(
        context: *mut CoglContext,
        mode: CoglVerticesMode,
        n_vertices: c_int,
        data: *const CoglVertexP3T2C4,
    ) -> *mut CoglPrimitive;
    pub fn cogl_primitive_new_with_attributes(
        mode: CoglVerticesMode,
        n_vertices: c_int,
        attributes: *mut *mut CoglAttribute,
        n_attributes: c_int,
    ) -> *mut CoglPrimitive;
    pub fn cogl_primitive_texture_set_auto_mipmap(
        primitive_texture: *mut CoglPrimitiveTexture,
        value: CoglBool,
    );
    pub fn cogl_primitive_copy(primitive: *mut CoglPrimitive) -> *mut CoglPrimitive;
    pub fn cogl_primitive_draw(
        primitive: *mut CoglPrimitive,
        framebuffer: *mut CoglFramebuffer,
        pipeline: *mut CoglPipeline,
    );
    pub fn cogl_primitive_foreach_attribute(
        primitive: *mut CoglPrimitive,
        callback: CoglPrimitiveAttributeCallback,
        user_data: *mut c_void,
    );
    pub fn cogl_primitive_get_first_vertex(primitive: *mut CoglPrimitive) -> c_int;
    pub fn cogl_primitive_get_indices(primitive: *mut CoglPrimitive) -> *mut CoglIndices;
    pub fn cogl_primitive_get_mode(primitive: *mut CoglPrimitive) -> CoglVerticesMode;
    pub fn cogl_primitive_get_n_vertices(primitive: *mut CoglPrimitive) -> c_int;
    pub fn cogl_primitive_set_attributes(
        primitive: *mut CoglPrimitive,
        attributes: *mut *mut CoglAttribute,
        n_attributes: c_int,
    );
    pub fn cogl_primitive_set_first_vertex(primitive: *mut CoglPrimitive, first_vertex: c_int);
    pub fn cogl_primitive_set_indices(
        primitive: *mut CoglPrimitive,
        indices: *mut CoglIndices,
        n_indices: c_int,
    );
    pub fn cogl_primitive_set_mode(primitive: *mut CoglPrimitive, mode: CoglVerticesMode);
    pub fn cogl_primitive_set_n_vertices(primitive: *mut CoglPrimitive, n_vertices: c_int);

    //=========================================================================
    // CoglRenderer
    //=========================================================================
    pub fn cogl_renderer_get_gtype() -> GType;
    pub fn cogl_renderer_new() -> *mut CoglRenderer;
    pub fn cogl_renderer_add_constraint(
        renderer: *mut CoglRenderer,
        constraint: CoglRendererConstraint,
    );
    pub fn cogl_renderer_check_onscreen_template(
        renderer: *mut CoglRenderer,
        onscreen_template: *mut CoglOnscreenTemplate,
        error: *mut *mut glib::GError,
    ) -> CoglBool;
    pub fn cogl_renderer_connect(
        renderer: *mut CoglRenderer,
        error: *mut *mut glib::GError,
    ) -> CoglBool;
    pub fn cogl_renderer_foreach_output(
        renderer: *mut CoglRenderer,
        callback: CoglOutputCallback,
        user_data: *mut c_void,
    );
    pub fn cogl_renderer_get_driver(renderer: *mut CoglRenderer) -> CoglDriver;
    pub fn cogl_renderer_get_n_fragment_texture_units(renderer: *mut CoglRenderer) -> c_int;
    pub fn cogl_renderer_get_winsys_id(renderer: *mut CoglRenderer) -> CoglWinsysID;
    pub fn cogl_renderer_remove_constraint(
        renderer: *mut CoglRenderer,
        constraint: CoglRendererConstraint,
    );
    pub fn cogl_renderer_set_driver(renderer: *mut CoglRenderer, driver: CoglDriver);
    pub fn cogl_renderer_set_winsys_id(renderer: *mut CoglRenderer, winsys_id: CoglWinsysID);

    //=========================================================================
    // CoglSnippet
    //=========================================================================
    pub fn cogl_snippet_get_gtype() -> GType;
    pub fn cogl_snippet_new(
        hook: CoglSnippetHook,
        declarations: *const c_char,
        post: *const c_char,
    ) -> *mut CoglSnippet;
    pub fn cogl_snippet_get_declarations(snippet: *mut CoglSnippet) -> *const c_char;
    pub fn cogl_snippet_get_hook(snippet: *mut CoglSnippet) -> CoglSnippetHook;
    pub fn cogl_snippet_get_post(snippet: *mut CoglSnippet) -> *const c_char;
    pub fn cogl_snippet_get_pre(snippet: *mut CoglSnippet) -> *const c_char;
    pub fn cogl_snippet_get_replace(snippet: *mut CoglSnippet) -> *const c_char;
    pub fn cogl_snippet_set_declarations(snippet: *mut CoglSnippet, declarations: *const c_char);
    pub fn cogl_snippet_set_post(snippet: *mut CoglSnippet, post: *const c_char);
    pub fn cogl_snippet_set_pre(snippet: *mut CoglSnippet, pre: *const c_char);
    pub fn cogl_snippet_set_replace(snippet: *mut CoglSnippet, replace: *const c_char);

    //=========================================================================
    // CoglSubTexture
    //=========================================================================
    pub fn cogl_sub_texture_get_gtype() -> GType;
    pub fn cogl_sub_texture_new(
        ctx: *mut CoglContext,
        parent_texture: *mut CoglTexture,
        sub_x: c_int,
        sub_y: c_int,
        sub_width: c_int,
        sub_height: c_int,
    ) -> *mut CoglSubTexture;
    pub fn cogl_sub_texture_get_parent(sub_texture: *mut CoglSubTexture) -> *mut CoglTexture;

    //=========================================================================
    // CoglSwapChain
    //=========================================================================
    pub fn cogl_swap_chain_get_gtype() -> GType;
    pub fn cogl_swap_chain_new() -> *mut CoglSwapChain;
    pub fn cogl_swap_chain_set_has_alpha(swap_chain: *mut CoglSwapChain, has_alpha: CoglBool);
    pub fn cogl_swap_chain_set_length(swap_chain: *mut CoglSwapChain, length: c_int);

    //=========================================================================
    // CoglTexture2D
    //=========================================================================
    pub fn cogl_texture_2d_get_gtype() -> GType;
    pub fn cogl_texture_2d_gl_new_from_foreign(
        ctx: *mut CoglContext,
        gl_handle: c_uint,
        width: c_int,
        height: c_int,
        format: CoglPixelFormat,
    ) -> *mut CoglTexture2D;
    pub fn cogl_texture_2d_new_from_bitmap(bitmap: *mut CoglBitmap) -> *mut CoglTexture2D;
    pub fn cogl_texture_2d_new_from_data(
        ctx: *mut CoglContext,
        width: c_int,
        height: c_int,
        format: CoglPixelFormat,
        rowstride: c_int,
        data: *const u8,
        error: *mut *mut glib::GError,
    ) -> *mut CoglTexture2D;
    pub fn cogl_texture_2d_new_from_file(
        ctx: *mut CoglContext,
        filename: *const c_char,
        error: *mut *mut glib::GError,
    ) -> *mut CoglTexture2D;
    pub fn cogl_texture_2d_new_with_size(
        ctx: *mut CoglContext,
        width: c_int,
        height: c_int,
    ) -> *mut CoglTexture2D;

    //=========================================================================
    // CoglTexture2DSliced
    //=========================================================================
    pub fn cogl_texture_2d_sliced_get_gtype() -> GType;
    pub fn cogl_texture_2d_sliced_new_from_bitmap(
        bmp: *mut CoglBitmap,
        max_waste: c_int,
    ) -> *mut CoglTexture2DSliced;
    pub fn cogl_texture_2d_sliced_new_from_data(
        ctx: *mut CoglContext,
        width: c_int,
        height: c_int,
        max_waste: c_int,
        format: CoglPixelFormat,
        rowstride: c_int,
        data: *const u8,
        error: *mut *mut glib::GError,
    ) -> *mut CoglTexture2DSliced;
    pub fn cogl_texture_2d_sliced_new_from_file(
        ctx: *mut CoglContext,
        filename: *const c_char,
        max_waste: c_int,
        error: *mut *mut glib::GError,
    ) -> *mut CoglTexture2DSliced;
    pub fn cogl_texture_2d_sliced_new_with_size(
        ctx: *mut CoglContext,
        width: c_int,
        height: c_int,
        max_waste: c_int,
    ) -> *mut CoglTexture2DSliced;

    //=========================================================================
    // CoglTexture3D
    //=========================================================================
    pub fn cogl_texture_3d_get_gtype() -> GType;
    pub fn cogl_texture_3d_new_from_bitmap(
        bitmap: *mut CoglBitmap,
        height: c_int,
        depth: c_int,
    ) -> *mut CoglTexture3D;
    pub fn cogl_texture_3d_new_from_data(
        context: *mut CoglContext,
        width: c_int,
        height: c_int,
        depth: c_int,
        format: CoglPixelFormat,
        rowstride: c_int,
        image_stride: c_int,
        data: *const u8,
        error: *mut *mut glib::GError,
    ) -> *mut CoglTexture3D;
    pub fn cogl_texture_3d_new_with_size(
        context: *mut CoglContext,
        width: c_int,
        height: c_int,
        depth: c_int,
    ) -> *mut CoglTexture3D;

    //=========================================================================
    // CoglTexturePixmapX11
    //=========================================================================
    pub fn cogl_texture_pixmap_x11_get_gtype() -> GType;
    pub fn cogl_texture_pixmap_x11_new(
        context: *mut CoglContext,
        pixmap: u32,
        automatic_updates: CoglBool,
        error: *mut *mut glib::GError,
    ) -> *mut CoglTexturePixmapX11;
    pub fn cogl_texture_pixmap_x11_new_left(
        context: *mut CoglContext,
        pixmap: u32,
        automatic_updates: CoglBool,
        error: *mut *mut glib::GError,
    ) -> *mut CoglTexturePixmapX11;
    pub fn cogl_texture_pixmap_x11_error_quark() -> u32;
    pub fn cogl_texture_pixmap_x11_is_using_tfp_extension(
        texture: *mut CoglTexturePixmapX11,
    ) -> CoglBool;
    pub fn cogl_texture_pixmap_x11_new_right(
        left_texture: *mut CoglTexturePixmapX11,
    ) -> *mut CoglTexturePixmapX11;
    pub fn cogl_texture_pixmap_x11_set_damage_object(
        texture: *mut CoglTexturePixmapX11,
        damage: u32,
        report_level: CoglTexturePixmapX11ReportLevel,
    );
    pub fn cogl_texture_pixmap_x11_update_area(
        texture: *mut CoglTexturePixmapX11,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    );

    //=========================================================================
    // CoglTextureRectangle
    //=========================================================================
    pub fn cogl_texture_rectangle_get_gtype() -> GType;
    pub fn cogl_texture_rectangle_new_from_bitmap(
        bitmap: *mut CoglBitmap,
    ) -> *mut CoglTextureRectangle;
    pub fn cogl_texture_rectangle_new_from_foreign(
        ctx: *mut CoglContext,
        gl_handle: c_uint,
        width: c_int,
        height: c_int,
        format: CoglPixelFormat,
    ) -> *mut CoglTextureRectangle;
    pub fn cogl_texture_rectangle_new_with_size(
        ctx: *mut CoglContext,
        width: c_int,
        height: c_int,
    ) -> *mut CoglTextureRectangle;

    //=========================================================================
    // CoglFramebuffer
    //=========================================================================
    pub fn cogl_framebuffer_get_gtype() -> GType;
    pub fn cogl_framebuffer_error_quark() -> u32;
    pub fn cogl_framebuffer_add_fence_callback(
        framebuffer: *mut CoglFramebuffer,
        callback: CoglFenceCallback,
        user_data: *mut c_void,
    ) -> *mut CoglFenceClosure;
    pub fn cogl_framebuffer_allocate(
        framebuffer: *mut CoglFramebuffer,
        error: *mut *mut glib::GError,
    ) -> CoglBool;
    pub fn cogl_framebuffer_cancel_fence_callback(
        framebuffer: *mut CoglFramebuffer,
        closure: *mut CoglFenceClosure,
    );
    pub fn cogl_framebuffer_clear(
        framebuffer: *mut CoglFramebuffer,
        buffers: c_ulong,
        color: *const CoglColor,
    );
    pub fn cogl_framebuffer_clear4f(
        framebuffer: *mut CoglFramebuffer,
        buffers: c_ulong,
        red: c_float,
        green: c_float,
        blue: c_float,
        alpha: c_float,
    );
    pub fn cogl_framebuffer_discard_buffers(framebuffer: *mut CoglFramebuffer, buffers: c_ulong);
    pub fn cogl_framebuffer_draw_attributes(
        framebuffer: *mut CoglFramebuffer,
        pipeline: *mut CoglPipeline,
        mode: CoglVerticesMode,
        first_vertex: c_int,
        n_vertices: c_int,
        attributes: *mut *mut CoglAttribute,
        n_attributes: c_int,
    );
    pub fn cogl_framebuffer_draw_indexed_attributes(
        framebuffer: *mut CoglFramebuffer,
        pipeline: *mut CoglPipeline,
        mode: CoglVerticesMode,
        first_vertex: c_int,
        n_vertices: c_int,
        indices: *mut CoglIndices,
        attributes: *mut *mut CoglAttribute,
        n_attributes: c_int,
    );
    pub fn cogl_framebuffer_draw_multitextured_rectangle(
        framebuffer: *mut CoglFramebuffer,
        pipeline: *mut CoglPipeline,
        x_1: c_float,
        y_1: c_float,
        x_2: c_float,
        y_2: c_float,
        tex_coords: *const c_float,
        tex_coords_len: c_int,
    );
    pub fn cogl_framebuffer_draw_primitive(
        framebuffer: *mut CoglFramebuffer,
        pipeline: *mut CoglPipeline,
        primitive: *mut CoglPrimitive,
    );
    pub fn cogl_framebuffer_draw_rectangle(
        framebuffer: *mut CoglFramebuffer,
        pipeline: *mut CoglPipeline,
        x_1: c_float,
        y_1: c_float,
        x_2: c_float,
        y_2: c_float,
    );
    pub fn cogl_framebuffer_draw_rectangles(
        framebuffer: *mut CoglFramebuffer,
        pipeline: *mut CoglPipeline,
        coordinates: *const c_float,
        n_rectangles: c_uint,
    );
    pub fn cogl_framebuffer_draw_textured_rectangle(
        framebuffer: *mut CoglFramebuffer,
        pipeline: *mut CoglPipeline,
        x_1: c_float,
        y_1: c_float,
        x_2: c_float,
        y_2: c_float,
        s_1: c_float,
        t_1: c_float,
        s_2: c_float,
        t_2: c_float,
    );
    pub fn cogl_framebuffer_draw_textured_rectangles(
        framebuffer: *mut CoglFramebuffer,
        pipeline: *mut CoglPipeline,
        coordinates: *const c_float,
        n_rectangles: c_uint,
    );
    pub fn cogl_framebuffer_finish(framebuffer: *mut CoglFramebuffer);
    pub fn cogl_framebuffer_frustum(
        framebuffer: *mut CoglFramebuffer,
        left: c_float,
        right: c_float,
        bottom: c_float,
        top: c_float,
        z_near: c_float,
        z_far: c_float,
    );
    pub fn cogl_framebuffer_get_alpha_bits(framebuffer: *mut CoglFramebuffer) -> c_int;
    pub fn cogl_framebuffer_get_blue_bits(framebuffer: *mut CoglFramebuffer) -> c_int;
    pub fn cogl_framebuffer_get_color_mask(framebuffer: *mut CoglFramebuffer) -> CoglColorMask;
    pub fn cogl_framebuffer_get_context(framebuffer: *mut CoglFramebuffer) -> *mut CoglContext;
    pub fn cogl_framebuffer_get_depth_bits(framebuffer: *mut CoglFramebuffer) -> c_int;
    pub fn cogl_framebuffer_get_depth_texture(
        framebuffer: *mut CoglFramebuffer,
    ) -> *mut CoglTexture;
    pub fn cogl_framebuffer_get_depth_texture_enabled(
        framebuffer: *mut CoglFramebuffer,
    ) -> CoglBool;
    pub fn cogl_framebuffer_get_depth_write_enabled(framebuffer: *mut CoglFramebuffer) -> CoglBool;
    pub fn cogl_framebuffer_get_dither_enabled(framebuffer: *mut CoglFramebuffer) -> CoglBool;
    pub fn cogl_framebuffer_get_green_bits(framebuffer: *mut CoglFramebuffer) -> c_int;
    pub fn cogl_framebuffer_get_height(framebuffer: *mut CoglFramebuffer) -> c_int;
    pub fn cogl_framebuffer_get_is_stereo(framebuffer: *mut CoglFramebuffer) -> CoglBool;
    pub fn cogl_framebuffer_get_modelview_matrix(
        framebuffer: *mut CoglFramebuffer,
        matrix: *mut CoglMatrix,
    );
    pub fn cogl_framebuffer_get_projection_matrix(
        framebuffer: *mut CoglFramebuffer,
        matrix: *mut CoglMatrix,
    );
    pub fn cogl_framebuffer_get_red_bits(framebuffer: *mut CoglFramebuffer) -> c_int;
    pub fn cogl_framebuffer_get_samples_per_pixel(framebuffer: *mut CoglFramebuffer) -> c_int;
    pub fn cogl_framebuffer_get_stereo_mode(framebuffer: *mut CoglFramebuffer) -> CoglStereoMode;
    pub fn cogl_framebuffer_get_viewport4fv(
        framebuffer: *mut CoglFramebuffer,
        viewport: *mut [c_float; 4],
    );
    pub fn cogl_framebuffer_get_viewport_height(framebuffer: *mut CoglFramebuffer) -> c_float;
    pub fn cogl_framebuffer_get_viewport_width(framebuffer: *mut CoglFramebuffer) -> c_float;
    pub fn cogl_framebuffer_get_viewport_x(framebuffer: *mut CoglFramebuffer) -> c_float;
    pub fn cogl_framebuffer_get_viewport_y(framebuffer: *mut CoglFramebuffer) -> c_float;
    pub fn cogl_framebuffer_get_width(framebuffer: *mut CoglFramebuffer) -> c_int;
    pub fn cogl_framebuffer_identity_matrix(framebuffer: *mut CoglFramebuffer);
    pub fn cogl_framebuffer_orthographic(
        framebuffer: *mut CoglFramebuffer,
        x_1: c_float,
        y_1: c_float,
        x_2: c_float,
        y_2: c_float,
        near: c_float,
        far: c_float,
    );
    pub fn cogl_framebuffer_perspective(
        framebuffer: *mut CoglFramebuffer,
        fov_y: c_float,
        aspect: c_float,
        z_near: c_float,
        z_far: c_float,
    );
    pub fn cogl_framebuffer_pop_clip(framebuffer: *mut CoglFramebuffer);
    pub fn cogl_framebuffer_pop_matrix(framebuffer: *mut CoglFramebuffer);
    pub fn cogl_framebuffer_push_matrix(framebuffer: *mut CoglFramebuffer);
    pub fn cogl_framebuffer_push_primitive_clip(
        framebuffer: *mut CoglFramebuffer,
        primitive: *mut CoglPrimitive,
        bounds_x1: c_float,
        bounds_y1: c_float,
        bounds_x2: c_float,
        bounds_y2: c_float,
    );
    pub fn cogl_framebuffer_push_rectangle_clip(
        framebuffer: *mut CoglFramebuffer,
        x_1: c_float,
        y_1: c_float,
        x_2: c_float,
        y_2: c_float,
    );
    pub fn cogl_framebuffer_push_scissor_clip(
        framebuffer: *mut CoglFramebuffer,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    );
    pub fn cogl_framebuffer_read_pixels(
        framebuffer: *mut CoglFramebuffer,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        format: CoglPixelFormat,
        pixels: *mut u8,
    ) -> CoglBool;
    pub fn cogl_framebuffer_read_pixels_into_bitmap(
        framebuffer: *mut CoglFramebuffer,
        x: c_int,
        y: c_int,
        source: CoglReadPixelsFlags,
        bitmap: *mut CoglBitmap,
    ) -> CoglBool;
    pub fn cogl_framebuffer_resolve_samples(framebuffer: *mut CoglFramebuffer);
    pub fn cogl_framebuffer_resolve_samples_region(
        framebuffer: *mut CoglFramebuffer,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    );
    pub fn cogl_framebuffer_rotate(
        framebuffer: *mut CoglFramebuffer,
        angle: c_float,
        x: c_float,
        y: c_float,
        z: c_float,
    );
    pub fn cogl_framebuffer_rotate_euler(
        framebuffer: *mut CoglFramebuffer,
        euler: *const CoglEuler,
    );
    pub fn cogl_framebuffer_rotate_quaternion(
        framebuffer: *mut CoglFramebuffer,
        quaternion: *const CoglQuaternion,
    );
    pub fn cogl_framebuffer_scale(
        framebuffer: *mut CoglFramebuffer,
        x: c_float,
        y: c_float,
        z: c_float,
    );
    pub fn cogl_framebuffer_set_color_mask(
        framebuffer: *mut CoglFramebuffer,
        color_mask: CoglColorMask,
    );
    pub fn cogl_framebuffer_set_depth_texture_enabled(
        framebuffer: *mut CoglFramebuffer,
        enabled: CoglBool,
    );
    pub fn cogl_framebuffer_set_depth_write_enabled(
        framebuffer: *mut CoglFramebuffer,
        depth_write_enabled: CoglBool,
    );
    pub fn cogl_framebuffer_set_dither_enabled(
        framebuffer: *mut CoglFramebuffer,
        dither_enabled: CoglBool,
    );
    pub fn cogl_framebuffer_set_modelview_matrix(
        framebuffer: *mut CoglFramebuffer,
        matrix: *const CoglMatrix,
    );
    pub fn cogl_framebuffer_set_projection_matrix(
        framebuffer: *mut CoglFramebuffer,
        matrix: *const CoglMatrix,
    );
    pub fn cogl_framebuffer_set_samples_per_pixel(
        framebuffer: *mut CoglFramebuffer,
        samples_per_pixel: c_int,
    );
    pub fn cogl_framebuffer_set_stereo_mode(
        framebuffer: *mut CoglFramebuffer,
        stereo_mode: CoglStereoMode,
    );
    pub fn cogl_framebuffer_set_viewport(
        framebuffer: *mut CoglFramebuffer,
        x: c_float,
        y: c_float,
        width: c_float,
        height: c_float,
    );
    pub fn cogl_framebuffer_transform(framebuffer: *mut CoglFramebuffer, matrix: *const CoglMatrix);
    pub fn cogl_framebuffer_translate(
        framebuffer: *mut CoglFramebuffer,
        x: c_float,
        y: c_float,
        z: c_float,
    );
    pub fn cogl_framebuffer_vdraw_attributes(
        framebuffer: *mut CoglFramebuffer,
        pipeline: *mut CoglPipeline,
        mode: CoglVerticesMode,
        first_vertex: c_int,
        n_vertices: c_int,
        ...
    );
    pub fn cogl_framebuffer_vdraw_indexed_attributes(
        framebuffer: *mut CoglFramebuffer,
        pipeline: *mut CoglPipeline,
        mode: CoglVerticesMode,
        first_vertex: c_int,
        n_vertices: c_int,
        indices: *mut CoglIndices,
        ...
    );

    //=========================================================================
    // CoglTexture
    //=========================================================================
    pub fn cogl_texture_get_gtype() -> GType;
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
    pub fn cogl_texture_set_data(
        texture: *mut CoglTexture,
        format: CoglPixelFormat,
        rowstride: c_int,
        data: *const u8,
        level: c_int,
        error: *mut *mut glib::GError,
    ) -> CoglBool;
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
    pub fn cogl_texture_set_region_from_bitmap(
        texture: *mut CoglTexture,
        src_x: c_int,
        src_y: c_int,
        dst_x: c_int,
        dst_y: c_int,
        dst_width: c_uint,
        dst_height: c_uint,
        bitmap: *mut CoglBitmap,
    ) -> CoglBool;
    pub fn cogl_texture_ref(texture: *mut c_void) -> *mut c_void;
    pub fn cogl_texture_unref(texture: *mut c_void);

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn cogl_angle_cos(angle: CoglAngle) -> CoglFixed;
    pub fn cogl_angle_sin(angle: CoglAngle) -> CoglFixed;
    pub fn cogl_angle_tan(angle: CoglAngle) -> CoglFixed;
    pub fn cogl_begin_gl();
    pub fn cogl_buffer_get_size(buffer: *mut CoglBuffer) -> c_uint;
    pub fn cogl_buffer_get_update_hint(buffer: *mut CoglBuffer) -> CoglBufferUpdateHint;
    pub fn cogl_buffer_map(
        buffer: *mut CoglBuffer,
        access: CoglBufferAccess,
        hints: CoglBufferMapHint,
    ) -> *mut c_void;
    pub fn cogl_buffer_map_range(
        buffer: *mut CoglBuffer,
        offset: size_t,
        size: size_t,
        access: CoglBufferAccess,
        hints: CoglBufferMapHint,
        error: *mut *mut glib::GError,
    ) -> *mut c_void;
    pub fn cogl_buffer_set_data(
        buffer: *mut CoglBuffer,
        offset: size_t,
        data: *mut c_void,
        size: size_t,
    ) -> CoglBool;
    pub fn cogl_buffer_set_update_hint(buffer: *mut CoglBuffer, hint: CoglBufferUpdateHint);
    pub fn cogl_buffer_unmap(buffer: *mut CoglBuffer);
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
    pub fn cogl_debug_matrix_entry_print(entry: *mut CoglMatrixEntry);
    pub fn cogl_debug_matrix_print(matrix: *const CoglMatrix);
    pub fn cogl_debug_object_foreach_type(
        func: CoglDebugObjectForeachTypeCallback,
        user_data: *mut c_void,
    );
    pub fn cogl_debug_object_print_instances();
    pub fn cogl_disable_fog();
    pub fn cogl_double_to_fixed(value: c_double) -> CoglFixed;
    pub fn cogl_double_to_int(value: c_double) -> c_int;
    pub fn cogl_double_to_uint(value: c_double) -> c_uint;
    pub fn cogl_egl_context_get_egl_context(context: *mut CoglContext);
    pub fn cogl_egl_context_get_egl_display(context: *mut CoglContext);
    pub fn cogl_end_gl();
    pub fn cogl_error_copy(error: *mut glib::GError) -> *mut glib::GError;
    pub fn cogl_error_free(error: *mut glib::GError);
    pub fn cogl_error_matches(error: *mut glib::GError, domain: u32, code: c_int) -> CoglBool;
    pub fn cogl_features_available(features: CoglFeatureFlags) -> CoglBool;
    pub fn cogl_flush();
    pub fn cogl_foreach_feature(
        context: *mut CoglContext,
        callback: CoglFeatureCallback,
        user_data: *mut c_void,
    );
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
    pub fn cogl_get_clock_time(context: *mut CoglContext) -> i64;
    pub fn cogl_get_depth_test_enabled() -> CoglBool;
    pub fn cogl_get_draw_framebuffer() -> *mut CoglFramebuffer;
    pub fn cogl_get_features() -> CoglFeatureFlags;
    pub fn cogl_get_modelview_matrix(matrix: *mut CoglMatrix);
    pub fn cogl_get_option_group() -> *mut glib::GOptionGroup;
    pub fn cogl_get_proc_address(name: *const c_char) -> CoglFuncPtr;
    pub fn cogl_get_projection_matrix(matrix: *mut CoglMatrix);
    pub fn cogl_get_rectangle_indices(
        context: *mut CoglContext,
        n_rectangles: c_int,
    ) -> *mut CoglIndices;
    pub fn cogl_get_source() -> *mut c_void;
    pub fn cogl_get_static_identity_quaternion() -> *const CoglQuaternion;
    pub fn cogl_get_static_zero_quaternion() -> *const CoglQuaternion;
    pub fn cogl_get_viewport(v: *mut [c_float; 4]);
    //pub fn cogl_gles2_get_current_vtable() -> /*Ignored*/*mut CoglGLES2Vtable;
    pub fn cogl_gles2_texture_2d_new_from_handle(
        ctx: *mut CoglContext,
        gles2_ctx: *mut CoglGLES2Context,
        handle: c_uint,
        width: c_int,
        height: c_int,
        format: CoglPixelFormat,
    ) -> *mut CoglTexture2D;
    pub fn cogl_gles2_texture_get_handle(
        texture: *mut CoglTexture,
        handle: *mut c_uint,
        target: *mut c_uint,
    ) -> CoglBool;
    pub fn cogl_glib_renderer_source_new(
        renderer: *mut CoglRenderer,
        priority: c_int,
    ) -> *mut glib::GSource;
    pub fn cogl_glib_source_new(context: *mut CoglContext, priority: c_int) -> *mut glib::GSource;
    pub fn cogl_glx_context_get_glx_context(context: *mut CoglContext);
    pub fn cogl_gtype_matrix_get_type() -> GType;
    pub fn cogl_handle_get_type() -> GType;
    pub fn cogl_handle_ref(handle: CoglHandle) -> CoglHandle;
    pub fn cogl_handle_unref(handle: CoglHandle);
    pub fn cogl_has_feature(context: *mut CoglContext, feature: CoglFeatureID) -> CoglBool;
    pub fn cogl_has_features(context: *mut CoglContext, ...) -> CoglBool;
    pub fn cogl_is_atlas_texture(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_attribute(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_attribute_buffer(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_bitmap(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_buffer(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_context(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_display(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_frame_info(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_framebuffer(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_gles2_context(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_index_buffer(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_indices(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_matrix_stack(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_material(handle: CoglHandle) -> CoglBool;
    pub fn cogl_is_offscreen(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_onscreen(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_onscreen_template(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_output(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_pipeline(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_pixel_buffer(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_primitive(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_primitive_texture(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_program(handle: CoglHandle) -> CoglBool;
    pub fn cogl_is_renderer(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_shader(handle: CoglHandle) -> CoglBool;
    pub fn cogl_is_snippet(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_sub_texture(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_swap_chain(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_texture(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_texture_2d(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_texture_2d_sliced(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_texture_3d(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_texture_pixmap_x11(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_texture_rectangle(object: *mut c_void) -> CoglBool;
    pub fn cogl_is_vertex_buffer(handle: CoglHandle) -> CoglBool;
    pub fn cogl_is_vertex_buffer_indices(handle: CoglHandle) -> CoglBool;
    pub fn cogl_kms_display_queue_modes_reset(display: *mut CoglDisplay);
    pub fn cogl_kms_display_set_ignore_crtc(display: *mut CoglDisplay, id: u32, ignore: CoglBool);
    pub fn cogl_kms_display_set_layout(
        display: *mut CoglDisplay,
        width: c_int,
        height: c_int,
        crtcs: *mut *mut CoglKmsCrtc,
        n_crtcs: c_int,
        error: *mut *mut glib::GError,
    ) -> CoglBool;

    // FIXME:
    // pub fn cogl_kms_renderer_get_gbm(renderer: *mut CoglRenderer) -> *mut gbm_device;

    pub fn cogl_kms_renderer_get_kms_fd(renderer: *mut CoglRenderer) -> c_int;
    pub fn cogl_kms_renderer_set_kms_fd(renderer: *mut CoglRenderer, fd: c_int);
    pub fn cogl_meta_texture_foreach_in_region(
        meta_texture: *mut CoglMetaTexture,
        tx_1: c_float,
        ty_1: c_float,
        tx_2: c_float,
        ty_2: c_float,
        wrap_s: CoglPipelineWrapMode,
        wrap_t: CoglPipelineWrapMode,
        callback: CoglMetaTextureCallback,
        user_data: *mut c_void,
    );
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
    pub fn cogl_poll_renderer_dispatch(
        renderer: *mut CoglRenderer,
        poll_fds: *const CoglPollFD,
        n_poll_fds: c_int,
    );
    pub fn cogl_poll_renderer_get_info(
        renderer: *mut CoglRenderer,
        poll_fds: *mut *mut CoglPollFD,
        n_poll_fds: *mut c_int,
        timeout: *mut i64,
    ) -> c_int;
    pub fn cogl_polygon(
        vertices: *const CoglTextureVertex,
        n_vertices: c_uint,
        use_color: CoglBool,
    );
    pub fn cogl_pop_draw_buffer();
    pub fn cogl_pop_gles2_context(ctx: *mut CoglContext);
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
    pub fn cogl_push_gles2_context(
        ctx: *mut CoglContext,
        gles2_ctx: *mut CoglGLES2Context,
        read_buffer: *mut CoglFramebuffer,
        write_buffer: *mut CoglFramebuffer,
        error: *mut *mut glib::GError,
    ) -> CoglBool;
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
    pub fn cogl_vector3_add(result: *mut c_float, a: *const c_float, b: *const c_float);
    pub fn cogl_vector3_copy(vector: *const c_float) -> *mut c_float;
    pub fn cogl_vector3_cross_product(result: *mut c_float, u: *const c_float, v: *const c_float);
    pub fn cogl_vector3_distance(a: *const c_float, b: *const c_float) -> c_float;
    pub fn cogl_vector3_divide_scalar(vector: *mut c_float, scalar: c_float);
    pub fn cogl_vector3_dot_product(a: *const c_float, b: *const c_float) -> c_float;
    pub fn cogl_vector3_equal(v1: *mut c_void, v2: *mut c_void) -> CoglBool;
    pub fn cogl_vector3_equal_with_epsilon(
        vector0: *const c_float,
        vector1: *const c_float,
        epsilon: c_float,
    ) -> CoglBool;
    pub fn cogl_vector3_free(vector: *mut c_float);
    pub fn cogl_vector3_init(vector: *mut c_float, x: c_float, y: c_float, z: c_float);
    pub fn cogl_vector3_init_zero(vector: *mut c_float);
    pub fn cogl_vector3_invert(vector: *mut c_float);
    pub fn cogl_vector3_magnitude(vector: *const c_float) -> c_float;
    pub fn cogl_vector3_multiply_scalar(vector: *mut c_float, scalar: c_float);
    pub fn cogl_vector3_normalize(vector: *mut c_float);
    pub fn cogl_vector3_subtract(result: *mut c_float, a: *const c_float, b: *const c_float);
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

    // FIXME:
    // pub fn cogl_wayland_display_set_compositor_display(display: *mut CoglDisplay, wayland_display: *mut wl_display);
    // pub fn cogl_wayland_onscreen_get_shell_surface(onscreen: *mut CoglOnscreen) -> *mut wl_shell_surface;
    // pub fn cogl_wayland_onscreen_get_surface(onscreen: *mut CoglOnscreen) -> *mut wl_surface;
    // pub fn cogl_wayland_onscreen_resize(onscreen: *mut CoglOnscreen, width: c_int, height: c_int, offset_x: c_int, offset_y: c_int);
    // pub fn cogl_wayland_onscreen_set_foreign_surface(onscreen: *mut CoglOnscreen, surface: *mut wl_surface);
    // pub fn cogl_wayland_renderer_get_display(renderer: *mut CoglRenderer) -> *mut wl_display;
    // pub fn cogl_wayland_renderer_set_event_dispatch_enabled(renderer: *mut CoglRenderer, enable: CoglBool);
    // pub fn cogl_wayland_renderer_set_foreign_display(renderer: *mut CoglRenderer, display: *mut wl_display);
    // pub fn cogl_wayland_texture_2d_new_from_buffer(ctx: *mut CoglContext, buffer: *mut wl_resource, error: *mut *mut glib::GError) -> *mut CoglTexture2D;
    // pub fn cogl_wayland_texture_set_region_from_shm_buffer(texture: *mut CoglTexture, src_x: c_int, src_y: c_int, width: c_int, height: c_int, shm_buffer: *mut wl_shm_buffer, dst_x: c_int, dst_y: c_int, level: c_int, error: *mut *mut glib::GError) -> CoglBool;

    pub fn cogl_x11_onscreen_get_visual_xid(onscreen: *mut CoglOnscreen) -> u32;
    pub fn cogl_x11_onscreen_get_window_xid(onscreen: *mut CoglOnscreen) -> u32;
    pub fn cogl_x11_onscreen_set_foreign_window_xid(
        onscreen: *mut CoglOnscreen,
        xid: u32,
        update: CoglOnscreenX11MaskCallback,
        user_data: *mut c_void,
    );
    pub fn cogl_xlib_get_display();
    pub fn cogl_xlib_handle_event(xevent: *mut c_void) -> CoglFilterReturn;
    pub fn cogl_xlib_set_display(display: *mut c_void);
    // pub fn cogl_xlib_renderer_add_filter(renderer: *mut c_void, func: CoglXlibFilterFunc, data: *mut c_void);
    pub fn cogl_xlib_renderer_get_display(renderer: *mut c_void);
    pub fn cogl_xlib_renderer_get_foreign_display(renderer: *mut c_void);
    pub fn cogl_xlib_renderer_get_visual_info(renderer: *mut c_void);
    pub fn cogl_xlib_renderer_handle_event(
        renderer: *mut c_void,
        event: *mut c_void,
    ) -> CoglFilterReturn;
    // pub fn cogl_xlib_renderer_remove_filter(renderer: *mut c_void, func: CoglXlibFilterFunc, data: *mut c_void);
    pub fn cogl_xlib_renderer_set_event_retrieval_enabled(renderer: *mut c_void, enable: CoglBool);
    pub fn cogl_xlib_renderer_set_foreign_display(renderer: *mut c_void, display: *mut c_void);
}

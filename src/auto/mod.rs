#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

mod atlas_texture;
pub use self::atlas_texture::{AtlasTexture, AtlasTextureClass};

mod attribute;
pub use self::attribute::{Attribute, AttributeClass};

mod attribute_buffer;
pub use self::attribute_buffer::{AttributeBuffer, AttributeBufferClass};

mod bitmap;
pub use self::bitmap::{Bitmap, BitmapClass};

mod context;
pub use self::context::{Context, ContextClass};

mod display;
pub use self::display::{Display, DisplayClass};

mod fixed;
pub use self::fixed::{Fixed, FixedClass};

mod frame_info;
pub use self::frame_info::{FrameInfo, FrameInfoClass};

mod framebuffer;
pub use self::framebuffer::FramebufferExt;
pub use self::framebuffer::{Framebuffer, NONE_FRAMEBUFFER};

mod gles2_context;
pub use self::gles2_context::{GLES2Context, GLES2ContextClass};

mod index_buffer;
pub use self::index_buffer::{IndexBuffer, IndexBufferClass};

mod indices;
pub use self::indices::{Indices, IndicesClass};

mod matrix_stack;
pub use self::matrix_stack::{MatrixStack, MatrixStackClass};

mod object;
pub use self::object::ObjectExt;
pub use self::object::{Object, ObjectClass, NONE_OBJECT};

mod offscreen;
pub use self::offscreen::{Offscreen, OffscreenClass};

mod onscreen;
pub use self::onscreen::{Onscreen, OnscreenClass};

mod onscreen_template;
pub use self::onscreen_template::{OnscreenTemplate, OnscreenTemplateClass};

mod output;
pub use self::output::{Output, OutputClass};

mod pipeline;
pub use self::pipeline::{Pipeline, PipelineClass};

mod pixel_buffer;
pub use self::pixel_buffer::{PixelBuffer, PixelBufferClass};

mod primitive;
pub use self::primitive::{Primitive, PrimitiveClass};

mod renderer;
pub use self::renderer::{Renderer, RendererClass};

mod snippet;
pub use self::snippet::{Snippet, SnippetClass};

mod sub_texture;
pub use self::sub_texture::{SubTexture, SubTextureClass};

mod swap_chain;
pub use self::swap_chain::{SwapChain, SwapChainClass};

mod texture;
pub use self::texture::TextureExt;
pub use self::texture::{Texture, NONE_TEXTURE};

mod texture2_d;
pub use self::texture2_d::{Texture2D, Texture2DClass};

mod texture2_dsliced;
pub use self::texture2_dsliced::{Texture2DSliced, Texture2DSlicedClass};

mod texture3_d;
pub use self::texture3_d::{Texture3D, Texture3DClass};

mod texture_pixmap_x11;
pub use self::texture_pixmap_x11::{TexturePixmapX11, TexturePixmapX11Class};

mod texture_rectangle;
pub use self::texture_rectangle::{TextureRectangle, TextureRectangleClass};

mod color;
pub use self::color::Color;

mod euler;
pub use self::euler::Euler;

mod frame_closure;
pub use self::frame_closure::FrameClosure;

mod matrix;
pub use self::matrix::Matrix;

mod matrix_entry;
pub use self::matrix_entry::MatrixEntry;

mod onscreen_dirty_closure;
pub use self::onscreen_dirty_closure::OnscreenDirtyClosure;

mod onscreen_resize_closure;
pub use self::onscreen_resize_closure::OnscreenResizeClosure;

mod quaternion;
pub use self::quaternion::Quaternion;

mod enums;
pub use self::enums::AttributeType;
pub use self::enums::BitmapError;
pub use self::enums::BlendStringError;
pub use self::enums::BufferError;
pub use self::enums::BufferUpdateHint;
pub use self::enums::DepthTestFunction;
pub use self::enums::Driver;
pub use self::enums::FeatureID;
pub use self::enums::FilterReturn;
pub use self::enums::FogMode;
pub use self::enums::FrameEvent;
pub use self::enums::FramebufferError;
pub use self::enums::GLES2ContextError;
pub use self::enums::IndicesType;
pub use self::enums::MaterialAlphaFunc;
pub use self::enums::MaterialFilter;
pub use self::enums::MaterialLayerType;
pub use self::enums::MaterialWrapMode;
pub use self::enums::PipelineAlphaFunc;
pub use self::enums::PipelineCullFaceMode;
pub use self::enums::PipelineFilter;
pub use self::enums::PipelineWrapMode;
pub use self::enums::PixelFormat;
pub use self::enums::PollFDEvent;
pub use self::enums::RendererError;
pub use self::enums::ShaderType;
pub use self::enums::SnippetHook;
pub use self::enums::StereoMode;
pub use self::enums::SubpixelOrder;
pub use self::enums::SystemError;
pub use self::enums::TextureComponents;
pub use self::enums::TextureError;
pub use self::enums::TexturePixmapX11Error;
pub use self::enums::TexturePixmapX11ReportLevel;
pub use self::enums::TextureType;
pub use self::enums::VerticesMode;
pub use self::enums::Winding;
pub use self::enums::WinsysFeature;
pub use self::enums::WinsysID;

mod flags;
pub use self::flags::BufferAccess;
pub use self::flags::BufferBit;
pub use self::flags::BufferMapHint;
pub use self::flags::BufferTarget;
pub use self::flags::ColorMask;
pub use self::flags::FeatureFlags;
pub use self::flags::ReadPixelsFlags;
pub use self::flags::RendererConstraint;
pub use self::flags::TextureFlags;

mod alias;
pub use alias::Angle;
// pub use alias::Bool;
pub use alias::Buffer;
// pub use alias::Handle;
pub use alias::MetaTexture;
pub use alias::PrimitiveTexture;
pub use alias::UserDataDestroyCallback;

#[doc(hidden)]
pub mod traits {
    pub use crate::FramebufferExt;
    pub use crate::ObjectExt;
    pub use crate::TextureExt;
}

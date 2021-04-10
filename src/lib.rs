#![cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
#![cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref))]

use glib::translate::*;

#[macro_use]
extern crate glib;

#[macro_use]
extern crate bitflags;

#[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
mod auto;
pub use auto::*;

mod debug_object_type_info;
pub use debug_object_type_info::DebugObjectTypeInfo;

mod depth_state;
pub use depth_state::DepthState;

mod fence_closure;
pub use fence_closure::FenceClosure;

mod fence;
pub use fence::Fence;

mod gles2_vtable;
pub use gles2_vtable::GLES2Vtable;

mod gtype_object;
pub use gtype_object::GtypeObject;

mod handle;
pub use handle::Handle;

mod kms_crtc;
pub use kms_crtc::KmsCrtc;

mod material;
pub use material::Material;

mod material_layer;
pub use material_layer::MaterialLayer;

mod onscreen_dirty_info;
pub use onscreen_dirty_info::OnscreenDirtyInfo;

mod poll_fd;
pub use poll_fd::PollFD;

mod texture_vertex;
pub use texture_vertex::TextureVertex;

mod user_data_key;
pub use user_data_key::UserDataKey;

mod vertex_p2;
pub use vertex_p2::VertexP2;

mod vertex_p2c4;
pub use vertex_p2c4::VertexP2C4;

mod vertex_p2t2;
pub use vertex_p2t2::VertexP2T2;

mod vertex_p2t2c4;
pub use vertex_p2t2c4::VertexP2T2C4;

mod vertex_p3;
pub use vertex_p3::VertexP3;

mod vertex_p3c4;
pub use vertex_p3c4::VertexP3C4;

mod vertex_p3t2;
pub use vertex_p3t2::VertexP3T2;

mod vertex_p3t2c4;
pub use vertex_p3t2c4::VertexP3T2C4;

pub(crate) const TRUE: i32 = ::glib_sys::GTRUE;
// pub(crate) const FALSE:i32 = ::glib_sys::GFALSE;

pub fn source_new(context: &Context, priority: glib::Priority) -> glib::Source {
    unsafe {
        let source = ffi::cogl_glib_source_new(context.to_glib_none().0, priority.to_glib());
        from_glib_full(source)
    }
}

// let source = glib_sys::g_timeout_source_new(interval);
// glib_sys::g_source_set_callback(
//     source,
//     Some(trampoline::<F>),
//     into_raw(func),
//     Some(destroy_closure::<F>),
// );
// glib_sys::g_source_set_priority(source, priority.to_glib());

// if let Some(name) = name {
//     glib_sys::g_source_set_name(source, name.to_glib_none().0);
// }

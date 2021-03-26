use crate::{
    Bitmap, Color, ColorMask, Context, Euler, Fence, FenceClosure, Matrix, Object, Pipeline,
    PixelFormat, Primitive, ReadPixelsFlags, StereoMode, Texture,
};

use crate::Quaternion;

use glib;
use glib::object::IsA;
use glib::translate::*;
use libc;
use std::boxed::Box as Box_;
use std::{fmt, ptr};

glib_wrapper! {
    pub struct Framebuffer(Interface<ffi::CoglFramebuffer>) @requires Object;

    match fn {
        get_type => || ffi::cogl_framebuffer_get_gtype(),
    }
}

impl Framebuffer {
    pub fn error_quark() -> u32 {
        unsafe { ffi::cogl_framebuffer_error_quark() }
    }
}

pub const NONE_FRAMEBUFFER: Option<&Framebuffer> = None;

/// Trait containing all `Framebuffer` methods.
///
/// # Implementors
///
/// [`Framebuffer`](struct.Framebuffer.html), [`Onscreen`](struct.Onscreen.html)
pub trait FramebufferExt: 'static {
    /// Calls the provided callback when all previously-submitted commands have
    /// been executed by the GPU.
    ///
    /// Returns non-NULL if the fence succeeded, or `None` if it was unable to
    /// be inserted and the callback will never be called. The user does not
    /// need to free the closure; it will be freed automatically when the
    /// callback is called, or cancelled.
    ///
    /// ## `callback`
    /// A `CoglFenceCallback` to be called when
    ///  all commands submitted to Cogl have been executed
    /// ## `user_data`
    /// Private data that will be passed to the callback
    fn add_fence_callback<P: Fn(&Fence) + 'static>(&self, callback: P) -> Option<FenceClosure>;

    /// Explicitly allocates a configured `Framebuffer` allowing developers to
    /// check and handle any errors that might arise from an unsupported
    /// configuration so that fallback configurations may be tried.
    ///
    /// `<note>`Many applications don't support any fallback options at least when
    /// they are initially developed and in that case the don't need to use this API
    /// since Cogl will automatically allocate a framebuffer when it first gets
    /// used. The disadvantage of relying on automatic allocation is that the
    /// program will abort with an error message if there is an error during
    /// automatic allocation.`</note>`
    ///
    /// # Returns
    ///
    /// `true` if there were no error allocating the framebuffer, else `false`.
    fn allocate(&self) -> Result<bool, glib::Error>;

    /// Removes a fence previously submitted with
    /// `Framebuffer::add_fence_callback`; the callback will not be
    /// called.
    ///
    /// ## `closure`
    /// The `FenceClosure` returned from
    ///  `Framebuffer::add_fence_callback`
    fn cancel_fence_callback(&self, closure: &mut FenceClosure);

    /// Clears all the auxiliary buffers identified in the `buffers` mask, and if
    /// that includes the color buffer then the specified `color` is used.
    /// ## `buffers`
    /// A mask of `BufferBit`<!-- -->'s identifying which auxiliary
    ///  buffers to clear
    /// ## `color`
    /// The color to clear the color buffer too if specified in
    ///  `buffers`.
    fn clear(&self, buffers: libc::c_ulong, color: &Color);

    /// Clears all the auxiliary buffers identified in the `buffers` mask, and if
    /// that includes the color buffer then the specified `color` is used.
    /// ## `buffers`
    /// A mask of `BufferBit`<!-- -->'s identifying which auxiliary
    ///  buffers to clear
    /// ## `red`
    /// The red component of color to clear the color buffer too if
    ///  specified in `buffers`.
    /// ## `green`
    /// The green component of color to clear the color buffer too if
    ///  specified in `buffers`.
    /// ## `blue`
    /// The blue component of color to clear the color buffer too if
    ///  specified in `buffers`.
    /// ## `alpha`
    /// The alpha component of color to clear the color buffer too if
    ///  specified in `buffers`.
    fn clear4f(&self, buffers: libc::c_ulong, red: f32, green: f32, blue: f32, alpha: f32);

    /// Declares that the specified `buffers` no longer need to be referenced
    /// by any further rendering commands. This can be an important
    /// optimization to avoid subsequent frames of rendering depending on
    /// the results of a previous frame.
    ///
    /// For example; some tile-based rendering GPUs are able to avoid allocating and
    /// accessing system memory for the depth and stencil buffer so long as these
    /// buffers are not required as input for subsequent frames and that can save a
    /// significant amount of memory bandwidth used to save and restore their
    /// contents to system memory between frames.
    ///
    /// It is currently considered an error to try and explicitly discard the color
    /// buffer by passing `BufferBit::Color`. This is because the color buffer is
    /// already implicitly discard when you finish rendering to a `Onscreen`
    /// framebuffer, and it's not meaningful to try and discard the color buffer of
    /// a `CoglOffscreen` framebuffer since they are single-buffered.
    /// ## `buffers`
    /// A `BufferBit` mask of which ancillary buffers you want
    ///  to discard.
    fn discard_buffers(&self, buffers: libc::c_ulong);

    /// Draws a textured rectangle to `self` with the given `pipeline`
    /// state with the top left corner positioned at (`x_1`, `y_1`) and the
    /// bottom right corner positioned at (`x_2`, `y_2`). As a pipeline may
    /// contain multiple texture layers this interface lets you supply
    /// texture coordinates for each layer of the pipeline.
    ///
    /// `<note>`The position is the position before the rectangle has been
    /// transformed by the model-view matrix and the projection
    /// matrix.`</note>`
    ///
    /// This is a high level drawing api that can handle any kind of
    /// `MetaTexture` texture for the first layer such as
    /// `Texture2DSliced` textures which may internally be comprised of
    /// multiple low-level textures. This is unlike low-level drawing apis
    /// such as `Primitive::draw` which only support low level texture
    /// types that are directly supported by GPUs such as `Texture2D`.
    ///
    /// `<note>`This api can not currently handle multiple high-level meta
    /// texture layers. The first layer may be a high level meta texture
    /// such as `Texture2DSliced` but all other layers much be low
    /// level textures such as `Texture2D` and additionally they
    /// should be textures that can be sampled using normalized coordinates
    /// (so not `TextureRectangle` textures).`</note>`
    ///
    /// The top left texture coordinate for layer 0 of any pipeline will be
    /// (tex_coords[0], tex_coords[1]) and the bottom right coordinate will
    /// be (tex_coords[2], tex_coords[3]). The coordinates for layer 1
    /// would be (tex_coords[4], tex_coords[5]) (tex_coords[6],
    /// tex_coords[7]) and so on...
    ///
    /// The given texture coordinates should always be normalized such that
    /// (0, 0) corresponds to the top left and (1, 1) corresponds to the
    /// bottom right. To map an entire texture across the rectangle pass
    /// in tex_coords[0]=0, tex_coords[1]=0, tex_coords[2]=1,
    /// tex_coords[3]=1.
    ///
    /// `<note>`Even if you have associated a `TextureRectangle` texture
    /// which normally implies working with non-normalized texture
    /// coordinates this api should still be passed normalized texture
    /// coordinates.`</note>`
    ///
    /// The first pair of coordinates are for the first layer (with the
    /// smallest layer index) and if you supply less texture coordinates
    /// than there are layers in the current source material then default
    /// texture coordinates (0.0, 0.0, 1.0, 1.0) are generated.
    /// ## `pipeline`
    /// A `Pipeline` state object
    /// ## `x_1`
    /// x coordinate upper left on screen.
    /// ## `y_1`
    /// y coordinate upper left on screen.
    /// ## `x_2`
    /// x coordinate lower right on screen.
    /// ## `y_2`
    /// y coordinate lower right on screen.
    /// ## `tex_coords`
    /// An array containing groups of
    ///  4 float values: [s_1, t_1, s_2, t_2] that are interpreted as two texture
    ///  coordinates; one for the top left texel, and one for the bottom right
    ///  texel. Each value should be between 0.0 and 1.0, where the coordinate
    ///  (0.0, 0.0) represents the top left of the texture, and (1.0, 1.0) the
    ///  bottom right.
    /// ## `tex_coords_len`
    /// The length of the `tex_coords` array. (For one layer
    ///  and one group of texture coordinates, this would be 4)
    fn draw_multitextured_rectangle(
        &self,
        pipeline: &Pipeline,
        x_1: f32,
        y_1: f32,
        x_2: f32,
        y_2: f32,
        tex_coords: &[f32],
    );

    /// Draws a rectangle to `self` with the given `pipeline` state
    /// and with the top left corner positioned at (`x_1`, `y_1`) and the
    /// bottom right corner positioned at (`x_2`, `y_2`).
    ///
    /// `<note>`The position is the position before the rectangle has been
    /// transformed by the model-view matrix and the projection
    /// matrix.`</note>`
    ///
    /// `<note>`If you want to describe a rectangle with a texture mapped on
    /// it then you can use
    /// `Framebuffer::draw_textured_rectangle`.`</note>`
    /// ## `pipeline`
    /// A `Pipeline` state object
    /// ## `x_1`
    /// X coordinate of the top-left corner
    /// ## `y_1`
    /// Y coordinate of the top-left corner
    /// ## `x_2`
    /// X coordinate of the bottom-right corner
    /// ## `y_2`
    /// Y coordinate of the bottom-right corner
    fn draw_rectangle(&self, pipeline: &Pipeline, x_1: f32, y_1: f32, x_2: f32, y_2: f32);

    //fn draw_rectangles(&self, pipeline: &Pipeline, coordinates: &[f32], n_rectangles: u32);

    /// Draws a textured rectangle to `self` using the given
    /// `pipeline` state with the top left corner positioned at (`x_1`, `y_1`)
    /// and the bottom right corner positioned at (`x_2`, `y_2`). The top
    /// left corner will have texture coordinates of (`s_1`, `t_1`) and the
    /// bottom right corner will have texture coordinates of (`s_2`, `t_2`).
    ///
    /// `<note>`The position is the position before the rectangle has been
    /// transformed by the model-view matrix and the projection
    /// matrix.`</note>`
    ///
    /// This is a high level drawing api that can handle any kind of
    /// `MetaTexture` texture such as `Texture2DSliced` textures
    /// which may internally be comprised of multiple low-level textures.
    /// This is unlike low-level drawing apis such as `Primitive::draw`
    /// which only support low level texture types that are directly
    /// supported by GPUs such as `Texture2D`.
    ///
    /// `<note>`The given texture coordinates will only be used for the first
    /// texture layer of the pipeline and if your pipeline has more than
    /// one layer then all other layers will have default texture
    /// coordinates of `s_1`=0.0 `t_1`=0.0 `s_2`=1.0 `t_2`=1.0 `</note>`
    ///
    /// The given texture coordinates should always be normalized such that
    /// (0, 0) corresponds to the top left and (1, 1) corresponds to the
    /// bottom right. To map an entire texture across the rectangle pass
    /// in `s_1`=0, `t_1`=0, `s_2`=1, `t_2`=1.
    ///
    /// `<note>`Even if you have associated a `TextureRectangle` texture
    /// with one of your `pipeline` layers which normally implies working
    /// with non-normalized texture coordinates this api should still be
    /// passed normalized texture coordinates.`</note>`
    /// ## `pipeline`
    /// A `Pipeline` state object
    /// ## `x_1`
    /// x coordinate upper left on screen.
    /// ## `y_1`
    /// y coordinate upper left on screen.
    /// ## `x_2`
    /// x coordinate lower right on screen.
    /// ## `y_2`
    /// y coordinate lower right on screen.
    /// ## `s_1`
    /// S texture coordinate of the top-left coorner
    /// ## `t_1`
    /// T texture coordinate of the top-left coorner
    /// ## `s_2`
    /// S texture coordinate of the bottom-right coorner
    /// ## `t_2`
    /// T texture coordinate of the bottom-right coorner
    fn draw_textured_rectangle(
        &self,
        pipeline: &Pipeline,
        x_1: f32,
        y_1: f32,
        x_2: f32,
        y_2: f32,
        s_1: f32,
        t_1: f32,
        s_2: f32,
        t_2: f32,
    );

    //fn draw_textured_rectangles(&self, pipeline: &Pipeline, coordinates: &[f32], n_rectangles: u32);

    /// This blocks the CPU until all pending rendering associated with the
    /// specified framebuffer has completed. It's very rare that developers should
    /// ever need this level of synchronization with the GPU and should never be
    /// used unless you clearly understand why you need to explicitly force
    /// synchronization.
    ///
    /// One example might be for benchmarking purposes to be sure timing
    /// measurements reflect the time that the GPU is busy for not just the time it
    /// takes to queue rendering commands.
    fn finish(&self);

    /// Replaces the current projection matrix with a perspective matrix
    /// for a given viewing frustum defined by 4 side clip planes that
    /// all cross through the origin and 2 near and far clip planes.
    /// ## `left`
    /// X position of the left clipping plane where it
    ///  intersects the near clipping plane
    /// ## `right`
    /// X position of the right clipping plane where it
    ///  intersects the near clipping plane
    /// ## `bottom`
    /// Y position of the bottom clipping plane where it
    ///  intersects the near clipping plane
    /// ## `top`
    /// Y position of the top clipping plane where it intersects
    ///  the near clipping plane
    /// ## `z_near`
    /// The distance to the near clipping plane (Must be positive)
    /// ## `z_far`
    /// The distance to the far clipping plane (Must be positive)
    fn frustum(&self, left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32);

    /// Retrieves the number of alpha bits of `self`
    ///
    /// # Returns
    ///
    /// the number of bits
    fn get_alpha_bits(&self) -> i32;

    /// Retrieves the number of blue bits of `self`
    ///
    /// # Returns
    ///
    /// the number of bits
    fn get_blue_bits(&self) -> i32;

    /// Gets the current `ColorMask` of which channels would be written to the
    /// current framebuffer. Each bit set in the mask means that the
    /// corresponding color would be written.
    ///
    /// # Returns
    ///
    /// A `ColorMask`
    fn get_color_mask(&self) -> ColorMask;

    /// Can be used to query the `Context` a given `self` was
    /// instantiated within. This is the `Context` that was passed to
    /// `Onscreen::new` for example.
    ///
    /// # Returns
    ///
    /// The `Context` that the given
    ///  `self` was instantiated within.
    fn get_context(&self) -> Option<Context>;

    /// Retrieves the number of depth bits of `self`
    ///
    ///
    /// # Returns
    ///
    /// the number of bits
    fn get_depth_bits(&self) -> i32;

    /// Retrieves the depth buffer of `self` as a `Texture`. You need to
    /// call cogl_framebuffer_get_depth_texture(fb, TRUE); before using this
    /// function.
    ///
    /// `<note>`Calling this function implicitely allocates the framebuffer.`</note>`
    /// `<note>`The texture returned stays valid as long as the framebuffer stays
    /// valid.`</note>`
    ///
    /// # Returns
    ///
    /// the depth texture
    fn get_depth_texture(&self) -> Option<Texture>;

    /// Queries whether texture based depth buffer has been enabled via
    /// `Framebuffer::set_depth_texture_enabled`.
    ///
    /// # Returns
    ///
    /// `true` if a depth texture has been enabled, else
    ///  `false`.
    fn get_depth_texture_enabled(&self) -> bool;

    /// Queries whether depth buffer writing is enabled for `self`. This
    /// can be controlled via `Framebuffer::set_depth_write_enabled`.
    ///
    /// # Returns
    ///
    /// `true` if depth writing is enabled or `false` if not.
    fn get_depth_write_enabled(&self) -> bool;

    /// Returns whether dithering has been requested for the given `self`.
    /// See `Framebuffer::set_dither_enabled` for more details about dithering.
    ///
    /// `<note>`This may return `true` even when the underlying `self`
    /// display pipeline does not support dithering. This value only represents
    /// the user's request for dithering.`</note>`
    ///
    /// # Returns
    ///
    /// `true` if dithering has been requested or `false` if not.
    fn get_dither_enabled(&self) -> bool;

    /// Retrieves the number of green bits of `self`
    ///
    /// # Returns
    ///
    /// the number of bits
    fn get_green_bits(&self) -> i32;

    /// Queries the current height of the given `self`.
    ///
    /// # Returns
    ///
    /// The height of `self`.
    fn get_height(&self) -> i32;

    fn get_is_stereo(&self) -> bool;

    /// Stores the current model-view matrix in `matrix`.
    /// ## `matrix`
    /// return location for the model-view matrix
    fn get_modelview_matrix(&self) -> Matrix;

    /// Stores the current projection matrix in `matrix`.
    /// ## `matrix`
    /// return location for the projection matrix
    fn get_projection_matrix(&self) -> Matrix;

    /// Retrieves the number of red bits of `self`
    ///
    /// # Returns
    ///
    /// the number of bits
    fn get_red_bits(&self) -> i32;

    /// Gets the number of points that are sampled per-pixel when
    /// rasterizing geometry. Usually by default this will return 0 which
    /// means that single-sample not multisample rendering has been chosen.
    /// When using a GPU supporting multisample rendering it's possible to
    /// increase the number of samples per pixel using
    /// `Framebuffer::set_samples_per_pixel`.
    ///
    /// Calling `Framebuffer::get_samples_per_pixel` before the
    /// framebuffer has been allocated will simply return the value set
    /// using `Framebuffer::set_samples_per_pixel`. After the
    /// framebuffer has been allocated the value will reflect the actual
    /// number of samples that will be made by the GPU.
    ///
    /// # Returns
    ///
    /// The number of point samples made per pixel when
    ///  rasterizing geometry or 0 if single-sample rendering
    ///  has been chosen.
    fn get_samples_per_pixel(&self) -> i32;

    /// Gets the current `StereoMode`, which defines which stereo buffers
    /// should be drawn to. See `Framebuffer::set_stereo_mode`.
    ///
    /// # Returns
    ///
    /// A `StereoMode`
    fn get_stereo_mode(&self) -> StereoMode;

    //fn get_viewport4fv(&self, viewport: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 4);

    /// Queries the height of the viewport as set using `Framebuffer::set_viewport`
    /// or the default value which is the height of the framebuffer.
    ///
    /// # Returns
    ///
    /// The height of the viewport.
    fn get_viewport_height(&self) -> f32;

    /// Queries the width of the viewport as set using `Framebuffer::set_viewport`
    /// or the default value which is the width of the framebuffer.
    ///
    /// # Returns
    ///
    /// The width of the viewport.
    fn get_viewport_width(&self) -> f32;

    /// Queries the x coordinate of the viewport origin as set using `Framebuffer::set_viewport`
    /// or the default value which is 0.
    ///
    /// # Returns
    ///
    /// The x coordinate of the viewport origin.
    fn get_viewport_x(&self) -> f32;

    /// Queries the y coordinate of the viewport origin as set using `Framebuffer::set_viewport`
    /// or the default value which is 0.
    ///
    /// # Returns
    ///
    /// The y coordinate of the viewport origin.
    fn get_viewport_y(&self) -> f32;

    /// Queries the current width of the given `self`.
    ///
    /// # Returns
    ///
    /// The width of `self`.
    fn get_width(&self) -> i32;

    /// Resets the current model-view matrix to the identity matrix.
    fn identity_matrix(&self);

    /// Replaces the current projection matrix with an orthographic projection
    /// matrix.
    /// ## `x_1`
    /// The x coordinate for the first vertical clipping plane
    /// ## `y_1`
    /// The y coordinate for the first horizontal clipping plane
    /// ## `x_2`
    /// The x coordinate for the second vertical clipping plane
    /// ## `y_2`
    /// The y coordinate for the second horizontal clipping plane
    /// ## `near`
    /// The `<emphasis>`distance`</emphasis>` to the near clipping
    ///  plane (will be `<emphasis>`negative`</emphasis>` if the plane is
    ///  behind the viewer)
    /// ## `far`
    /// The `<emphasis>`distance`</emphasis>` to the far clipping
    ///  plane (will be `<emphasis>`negative`</emphasis>` if the plane is
    ///  behind the viewer)
    fn orthographic(&self, x_1: f32, y_1: f32, x_2: f32, y_2: f32, near: f32, far: f32);

    /// Replaces the current projection matrix with a perspective matrix
    /// based on the provided values.
    ///
    /// `<note>`You should be careful not to have to great a `z_far` / `z_near`
    /// ratio since that will reduce the effectiveness of depth testing
    /// since there wont be enough precision to identify the depth of
    /// objects near to each other.`</note>`
    /// ## `fov_y`
    /// Vertical field of view angle in degrees.
    /// ## `aspect`
    /// The (width over height) aspect ratio for display
    /// ## `z_near`
    /// The distance to the near clipping plane (Must be positive,
    ///  and must not be 0)
    /// ## `z_far`
    /// The distance to the far clipping plane (Must be positive)
    fn perspective(&self, fov_y: f32, aspect: f32, z_near: f32, z_far: f32);

    /// Reverts the clipping region to the state before the last call to
    /// `Framebuffer::push_scissor_clip`, `Framebuffer::push_rectangle_clip`
    /// `cogl_framebuffer_push_path_clip`, or `Framebuffer::push_primitive_clip`.
    fn pop_clip(&self);

    /// Restores the model-view matrix on the top of the matrix stack.
    fn pop_matrix(&self);

    /// Copies the current model-view matrix onto the matrix stack. The matrix
    /// can later be restored with `Framebuffer::pop_matrix`.
    fn push_matrix(&self);

    /// Sets a new clipping area using a 2D shaped described with a
    /// `Primitive`. The shape must not contain self overlapping
    /// geometry and must lie on a single 2D plane. A bounding box of the
    /// 2D shape in local coordinates (the same coordinates used to
    /// describe the shape) must be given. It is acceptable for the bounds
    /// to be larger than the true bounds but behaviour is undefined if the
    /// bounds are smaller than the true bounds.
    ///
    /// The primitive is transformed by the current model-view matrix and
    /// the silhouette is intersected with the previous clipping area. To
    /// restore the previous clipping area, call
    /// `Framebuffer::pop_clip`.
    /// ## `primitive`
    /// A `Primitive` describing a flat 2D shape
    /// ## `bounds_x1`
    /// x coordinate for the top-left corner of the primitives
    ///  bounds
    /// ## `bounds_y1`
    /// y coordinate for the top-left corner of the primitives
    ///  bounds
    /// ## `bounds_x2`
    /// x coordinate for the bottom-right corner of the
    ///  primitives bounds.
    /// ## `bounds_y2`
    /// y coordinate for the bottom-right corner of the
    ///  primitives bounds.
    fn push_primitive_clip(
        &self,
        primitive: &Primitive,
        bounds_x1: f32,
        bounds_y1: f32,
        bounds_x2: f32,
        bounds_y2: f32,
    );

    /// Specifies a modelview transformed rectangular clipping area for all
    /// subsequent drawing operations. Any drawing commands that extend
    /// outside the rectangle will be clipped so that only the portion
    /// inside the rectangle will be displayed. The rectangle dimensions
    /// are transformed by the current model-view matrix.
    ///
    /// The rectangle is intersected with the current clip region. To undo
    /// the effect of this function, call `Framebuffer::pop_clip`.
    /// ## `x_1`
    /// x coordinate for top left corner of the clip rectangle
    /// ## `y_1`
    /// y coordinate for top left corner of the clip rectangle
    /// ## `x_2`
    /// x coordinate for bottom right corner of the clip rectangle
    /// ## `y_2`
    /// y coordinate for bottom right corner of the clip rectangle
    fn push_rectangle_clip(&self, x_1: f32, y_1: f32, x_2: f32, y_2: f32);

    /// Specifies a rectangular clipping area for all subsequent drawing
    /// operations. Any drawing commands that extend outside the rectangle
    /// will be clipped so that only the portion inside the rectangle will
    /// be displayed. The rectangle dimensions are not transformed by the
    /// current model-view matrix.
    ///
    /// The rectangle is intersected with the current clip region. To undo
    /// the effect of this function, call `Framebuffer::pop_clip`.
    /// ## `x`
    /// left edge of the clip rectangle in window coordinates
    /// ## `y`
    /// top edge of the clip rectangle in window coordinates
    /// ## `width`
    /// width of the clip rectangle
    /// ## `height`
    /// height of the clip rectangle
    fn push_scissor_clip(&self, x: i32, y: i32, width: i32, height: i32);

    /// This is a convenience wrapper around
    /// `Framebuffer::read_pixels_into_bitmap` which allocates a
    /// temporary `Bitmap` to read pixel data directly into the given
    /// buffer. The rowstride of the buffer is assumed to be the width of
    /// the region times the bytes per pixel of the format. The source for
    /// the data is always taken from the color buffer. If you want to use
    /// any other rowstride or source, please use the
    /// `Framebuffer::read_pixels_into_bitmap` function directly.
    ///
    /// The implementation of the function looks like this:
    ///
    ///
    /// ```text
    /// bitmap = cogl_bitmap_new_for_data (context,
    ///                                    width, height,
    ///                                    format,
    ///                                    /<!-- -->* rowstride *<!-- -->/
    ///                                    bpp * width,
    ///                                    pixels);
    /// cogl_framebuffer_read_pixels_into_bitmap (framebuffer,
    ///                                           x, y,
    ///                                           COGL_READ_PIXELS_COLOR_BUFFER,
    ///                                           bitmap);
    /// cogl_object_unref (bitmap);
    /// ```
    /// ## `x`
    /// The x position to read from
    /// ## `y`
    /// The y position to read from
    /// ## `width`
    /// The width of the region of rectangles to read
    /// ## `height`
    /// The height of the region of rectangles to read
    /// ## `format`
    /// The pixel format to store the data in
    /// ## `pixels`
    /// The address of the buffer to store the data in
    ///
    /// # Returns
    ///
    /// `true` if the read succeeded or `false` otherwise.
    fn read_pixels(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: PixelFormat,
        pixels: &[u8],
    ) -> bool;

    /// This reads a rectangle of pixels from the given framebuffer where
    /// position (0, 0) is the top left. The pixel at (x, y) is the first
    /// read, and a rectangle of pixels with the same size as the bitmap is
    /// read right and downwards from that point.
    ///
    /// Currently Cogl assumes that the framebuffer is in a premultiplied
    /// format so if the format of `bitmap` is non-premultiplied it will
    /// convert it. To read the pixel values without any conversion you
    /// should either specify a format that doesn't use an alpha channel or
    /// use one of the formats ending in PRE.
    /// ## `x`
    /// The x position to read from
    /// ## `y`
    /// The y position to read from
    /// ## `source`
    /// Identifies which auxillary buffer you want to read
    ///  (only COGL_READ_PIXELS_COLOR_BUFFER supported currently)
    /// ## `bitmap`
    /// The bitmap to store the results in.
    ///
    /// # Returns
    ///
    /// `true` if the read succeeded or `false` otherwise. The
    ///  function is only likely to fail if the bitmap points to a pixel
    ///  buffer and it could not be mapped.
    fn read_pixels_into_bitmap(
        &self,
        x: i32,
        y: i32,
        source: ReadPixelsFlags,
        bitmap: &Bitmap,
    ) -> bool;

    /// When point sample rendering (also known as multisample rendering)
    /// has been enabled via `Framebuffer::set_samples_per_pixel`
    /// then you can optionally call this function (or
    /// `Framebuffer::resolve_samples_region`) to explicitly resolve
    /// the point samples into values for the final color buffer.
    ///
    /// Some GPUs will implicitly resolve the point samples during
    /// rendering and so this function is effectively a nop, but with other
    /// architectures it is desirable to defer the resolve step until the
    /// end of the frame.
    ///
    /// Since Cogl will automatically ensure samples are resolved if the
    /// target color buffer is used as a source this API only needs to be
    /// used if explicit control is desired - perhaps because you want to
    /// ensure that the resolve is completed in advance to avoid later
    /// having to wait for the resolve to complete.
    ///
    /// If you are performing incremental updates to a framebuffer you
    /// should consider using `Framebuffer::resolve_samples_region`
    /// instead to avoid resolving redundant pixels.
    fn resolve_samples(&self);

    /// When point sample rendering (also known as multisample rendering)
    /// has been enabled via `Framebuffer::set_samples_per_pixel`
    /// then you can optionally call this function (or
    /// `Framebuffer::resolve_samples`) to explicitly resolve the point
    /// samples into values for the final color buffer.
    ///
    /// Some GPUs will implicitly resolve the point samples during
    /// rendering and so this function is effectively a nop, but with other
    /// architectures it is desirable to defer the resolve step until the
    /// end of the frame.
    ///
    /// Use of this API is recommended if incremental, small updates to
    /// a framebuffer are being made because by default Cogl will
    /// implicitly resolve all the point samples of the framebuffer which
    /// can result in redundant work if only a small number of samples have
    /// changed.
    ///
    /// Because some GPUs implicitly resolve point samples this function
    /// only guarantees that at-least the region specified will be resolved
    /// and if you have rendered to a larger region then it's possible that
    /// other samples may be implicitly resolved.
    /// ## `x`
    /// top-left x coordinate of region to resolve
    /// ## `y`
    /// top-left y coordinate of region to resolve
    /// ## `width`
    /// width of region to resolve
    /// ## `height`
    /// height of region to resolve
    fn resolve_samples_region(&self, x: i32, y: i32, width: i32, height: i32);

    /// Multiplies the current model-view matrix by one that rotates the
    /// model around the axis-vector specified by `x`, `y` and `z`. The
    /// rotation follows the right-hand thumb rule so for example rotating
    /// by 10 degrees about the axis-vector (0, 0, 1) causes a small
    /// counter-clockwise rotation.
    /// ## `angle`
    /// Angle in degrees to rotate.
    /// ## `x`
    /// X-component of vertex to rotate around.
    /// ## `y`
    /// Y-component of vertex to rotate around.
    /// ## `z`
    /// Z-component of vertex to rotate around.
    fn rotate(&self, angle: f32, x: f32, y: f32, z: f32);

    /// Multiplies the current model-view matrix by one that rotates
    /// according to the rotation described by `euler`.
    ///
    /// ## `euler`
    /// A `Euler`
    fn rotate_euler(&self, euler: &Euler);

    /// Multiplies the current model-view matrix by one that rotates
    /// according to the rotation described by `quaternion`.
    ///
    /// ## `quaternion`
    /// A `Quaternion`
    fn rotate_quaternion(&self, quaternion: &Quaternion);

    /// Multiplies the current model-view matrix by one that scales the x,
    /// y and z axes by the given values.
    /// ## `x`
    /// Amount to scale along the x-axis
    /// ## `y`
    /// Amount to scale along the y-axis
    /// ## `z`
    /// Amount to scale along the z-axis
    fn scale(&self, x: f32, y: f32, z: f32);

    /// Defines a bit mask of which color channels should be written to the
    /// given `self`. If a bit is set in `color_mask` that means that
    /// color will be written.
    /// ## `color_mask`
    /// A `ColorMask` of which color channels to write to
    ///  the current framebuffer.
    fn set_color_mask(&self, color_mask: ColorMask);

    /// If `enabled` is `true`, the depth buffer used when rendering to `self`
    /// is available as a texture. You can retrieve the texture with
    /// `Framebuffer::get_depth_texture`.
    ///
    /// `<note>`It's possible that your GPU does not support depth textures. You
    /// should check the `FeatureID::OglFeatureIdDepthTexture` feature before using this
    /// function.`</note>`
    /// `<note>`It's not valid to call this function after the framebuffer has been
    /// allocated as the creation of the depth texture is done at allocation time.
    /// `</note>`
    /// ## `enabled`
    /// TRUE or FALSE
    fn set_depth_texture_enabled(&self, enabled: bool);

    /// Enables or disables depth buffer writing when rendering to `self`.
    /// If depth writing is enabled for both the framebuffer and the rendering
    /// pipeline, and the framebuffer has an associated depth buffer, depth
    /// information will be written to this buffer during rendering.
    ///
    /// Depth buffer writing is enabled by default.
    /// ## `depth_write_enabled`
    /// `true` to enable depth writing or `false` to disable
    fn set_depth_write_enabled(&self, depth_write_enabled: bool);

    /// Enables or disabled dithering if supported by the hardware.
    ///
    /// Dithering is a hardware dependent technique to increase the visible
    /// color resolution beyond what the underlying hardware supports by playing
    /// tricks with the colors placed into the framebuffer to give the illusion
    /// of other colors. (For example this can be compared to half-toning used
    /// by some news papers to show varying levels of grey even though their may
    /// only be black and white are available).
    ///
    /// If the current display pipeline for `self` does not support dithering
    /// then this has no affect.
    ///
    /// Dithering is enabled by default.
    /// ## `dither_enabled`
    /// `true` to enable dithering or `false` to disable
    fn set_dither_enabled(&self, dither_enabled: bool);

    /// Sets `matrix` as the new model-view matrix.
    /// ## `matrix`
    /// the new model-view matrix
    fn set_modelview_matrix(&self, matrix: &Matrix);

    /// Sets `matrix` as the new projection matrix.
    /// ## `matrix`
    /// the new projection matrix
    fn set_projection_matrix(&self, matrix: &Matrix);

    /// Requires that when rendering to `self` then `n` point samples
    /// should be made per pixel which will all contribute to the final
    /// resolved color for that pixel. The idea is that the hardware aims
    /// to get quality similar to what you would get if you rendered
    /// everything twice as big (for 4 samples per pixel) and then scaled
    /// that image back down with filtering. It can effectively remove the
    /// jagged edges of polygons and should be more efficient than if you
    /// were to manually render at a higher resolution and downscale
    /// because the hardware is often able to take some shortcuts. For
    /// example the GPU may only calculate a single texture sample for all
    /// points of a single pixel, and for tile based architectures all the
    /// extra sample data (such as depth and stencil samples) may be
    /// handled on-chip and so avoid increased demand on system memory
    /// bandwidth.
    ///
    /// By default this value is usually set to 0 and that is referred to
    /// as "single-sample" rendering. A value of 1 or greater is referred
    /// to as "multisample" rendering.
    ///
    /// `<note>`There are some semantic differences between single-sample
    /// rendering and multisampling with just 1 point sample such as it
    /// being redundant to use the `Framebuffer::resolve_samples` and
    /// `Framebuffer::resolve_samples_region` apis with single-sample
    /// rendering.`</note>`
    ///
    /// `<note>`It's recommended that
    /// `Framebuffer::resolve_samples_region` be explicitly used at the
    /// end of rendering to a point sample buffer to minimize the number of
    /// samples that get resolved. By default Cogl will implicitly resolve
    /// all framebuffer samples but if only a small region of a
    /// framebuffer has changed this can lead to redundant work being
    /// done.`</note>`
    /// ## `samples_per_pixel`
    /// The minimum number of samples per pixel
    fn set_samples_per_pixel(&self, samples_per_pixel: i32);

    /// Sets which stereo buffers should be drawn to. The default
    /// is `StereoMode::Both`, which means that both the left and
    /// right buffers will be affected by drawing. For this to have
    /// an effect, the display system must support stereo drawables,
    /// and the framebuffer must have been created with stereo
    /// enabled. (See `OnscreenTemplate::set_stereo_enabled`,
    /// `Framebuffer::get_is_stereo`.)
    /// ## `stereo_mode`
    /// A `StereoMode` specifying which stereo buffers
    ///  should be drawn tow.
    fn set_stereo_mode(&self, stereo_mode: StereoMode);

    /// Defines a scale and offset for everything rendered relative to the
    /// top-left of the destination framebuffer.
    ///
    /// By default the viewport has an origin of (0,0) and width and height
    /// that match the framebuffer's size. Assuming a default projection and
    /// modelview matrix then you could translate the contents of a window
    /// down and right by leaving the viewport size unchanged by moving the
    /// offset to (10,10). The viewport coordinates are measured in pixels.
    /// If you left the x and y origin as (0,0) you could scale the windows
    /// contents down by specify and width and height that's half the real
    /// size of the framebuffer.
    ///
    /// `<note>`Although the function takes floating point arguments, existing
    /// drivers only allow the use of integer values. In the future floating
    /// point values will be exposed via a checkable feature.`</note>`
    /// ## `x`
    /// The top-left x coordinate of the viewport origin (only integers
    ///  supported currently)
    /// ## `y`
    /// The top-left y coordinate of the viewport origin (only integers
    ///  supported currently)
    /// ## `width`
    /// The width of the viewport (only integers supported currently)
    /// ## `height`
    /// The height of the viewport (only integers supported currently)
    fn set_viewport(&self, x: f32, y: f32, width: f32, height: f32);

    /// Multiplies the current model-view matrix by the given matrix.
    /// ## `matrix`
    /// the matrix to multiply with the current model-view
    fn transform(&self, matrix: &Matrix);

    /// Multiplies the current model-view matrix by one that translates the
    /// model along all three axes according to the given values.
    /// ## `x`
    /// Distance to translate along the x-axis
    /// ## `y`
    /// Distance to translate along the y-axis
    /// ## `z`
    /// Distance to translate along the z-axis
    fn translate(&self, x: f32, y: f32, z: f32);
}

impl<O: IsA<Framebuffer>> FramebufferExt for O {
    fn add_fence_callback<P: Fn(&Fence) + 'static>(&self, callback: P) -> Option<FenceClosure> {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<P: Fn(&Fence) + 'static>(
            fence: *mut ffi::CoglFence,
            user_data: glib_sys::gpointer,
        ) {
            let fence = from_glib_borrow(fence);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(&fence);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            from_glib_none(ffi::cogl_framebuffer_add_fence_callback(
                self.as_ref().to_glib_none().0,
                callback,
                Box_::into_raw(super_callback0) as *mut _,
            ))
        }
    }

    fn allocate(&self) -> Result<bool, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_framebuffer_allocate(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(ret == crate::TRUE)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn cancel_fence_callback(&self, closure: &mut FenceClosure) {
        unsafe {
            ffi::cogl_framebuffer_cancel_fence_callback(
                self.as_ref().to_glib_none().0,
                closure.to_glib_none_mut().0,
            );
        }
    }

    fn clear(&self, buffers: libc::c_ulong, color: &Color) {
        unsafe {
            ffi::cogl_framebuffer_clear(
                self.as_ref().to_glib_none().0,
                buffers,
                color.to_glib_none().0,
            );
        }
    }

    fn clear4f(&self, buffers: libc::c_ulong, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            ffi::cogl_framebuffer_clear4f(
                self.as_ref().to_glib_none().0,
                buffers,
                red,
                green,
                blue,
                alpha,
            );
        }
    }

    fn discard_buffers(&self, buffers: libc::c_ulong) {
        unsafe {
            ffi::cogl_framebuffer_discard_buffers(self.as_ref().to_glib_none().0, buffers);
        }
    }

    fn draw_multitextured_rectangle(
        &self,
        pipeline: &Pipeline,
        x_1: f32,
        y_1: f32,
        x_2: f32,
        y_2: f32,
        tex_coords: &[f32],
    ) {
        let tex_coords_len = tex_coords.len() as i32;
        unsafe {
            ffi::cogl_framebuffer_draw_multitextured_rectangle(
                self.as_ref().to_glib_none().0,
                pipeline.to_glib_none().0,
                x_1,
                y_1,
                x_2,
                y_2,
                tex_coords.to_glib_none().0,
                tex_coords_len,
            );
        }
    }

    fn draw_rectangle(&self, pipeline: &Pipeline, x_1: f32, y_1: f32, x_2: f32, y_2: f32) {
        unsafe {
            ffi::cogl_framebuffer_draw_rectangle(
                self.as_ref().to_glib_none().0,
                pipeline.to_glib_none().0,
                x_1,
                y_1,
                x_2,
                y_2,
            );
        }
    }

    //fn draw_rectangles(&self, pipeline: &Pipeline, coordinates: &[f32], n_rectangles: u32) {
    //    unsafe { TODO: call cogl_sys:cogl_framebuffer_draw_rectangles() }
    //}

    fn draw_textured_rectangle(
        &self,
        pipeline: &Pipeline,
        x_1: f32,
        y_1: f32,
        x_2: f32,
        y_2: f32,
        s_1: f32,
        t_1: f32,
        s_2: f32,
        t_2: f32,
    ) {
        unsafe {
            ffi::cogl_framebuffer_draw_textured_rectangle(
                self.as_ref().to_glib_none().0,
                pipeline.to_glib_none().0,
                x_1,
                y_1,
                x_2,
                y_2,
                s_1,
                t_1,
                s_2,
                t_2,
            );
        }
    }

    //fn draw_textured_rectangles(&self, pipeline: &Pipeline, coordinates: &[f32], n_rectangles: u32) {
    //    unsafe { TODO: call cogl_sys:cogl_framebuffer_draw_textured_rectangles() }
    //}

    fn finish(&self) {
        unsafe {
            ffi::cogl_framebuffer_finish(self.as_ref().to_glib_none().0);
        }
    }

    fn frustum(&self, left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) {
        unsafe {
            ffi::cogl_framebuffer_frustum(
                self.as_ref().to_glib_none().0,
                left,
                right,
                bottom,
                top,
                z_near,
                z_far,
            );
        }
    }

    fn get_alpha_bits(&self) -> i32 {
        unsafe { ffi::cogl_framebuffer_get_alpha_bits(self.as_ref().to_glib_none().0) }
    }

    fn get_blue_bits(&self) -> i32 {
        unsafe { ffi::cogl_framebuffer_get_blue_bits(self.as_ref().to_glib_none().0) }
    }

    fn get_color_mask(&self) -> ColorMask {
        unsafe {
            from_glib(ffi::cogl_framebuffer_get_color_mask(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_context(&self) -> Option<Context> {
        unsafe {
            from_glib_none(ffi::cogl_framebuffer_get_context(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_depth_bits(&self) -> i32 {
        unsafe { ffi::cogl_framebuffer_get_depth_bits(self.as_ref().to_glib_none().0) }
    }

    fn get_depth_texture(&self) -> Option<Texture> {
        unsafe {
            from_glib_none(ffi::cogl_framebuffer_get_depth_texture(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_depth_texture_enabled(&self) -> bool {
        unsafe {
            ffi::cogl_framebuffer_get_depth_texture_enabled(self.as_ref().to_glib_none().0)
                == crate::TRUE
        }
    }

    fn get_depth_write_enabled(&self) -> bool {
        unsafe {
            ffi::cogl_framebuffer_get_depth_write_enabled(self.as_ref().to_glib_none().0)
                == crate::TRUE
        }
    }

    fn get_dither_enabled(&self) -> bool {
        unsafe {
            ffi::cogl_framebuffer_get_dither_enabled(self.as_ref().to_glib_none().0) == crate::TRUE
        }
    }

    fn get_green_bits(&self) -> i32 {
        unsafe { ffi::cogl_framebuffer_get_green_bits(self.as_ref().to_glib_none().0) }
    }

    fn get_height(&self) -> i32 {
        unsafe { ffi::cogl_framebuffer_get_height(self.as_ref().to_glib_none().0) }
    }

    fn get_is_stereo(&self) -> bool {
        unsafe {
            ffi::cogl_framebuffer_get_is_stereo(self.as_ref().to_glib_none().0) == crate::TRUE
        }
    }

    fn get_modelview_matrix(&self) -> Matrix {
        unsafe {
            let mut matrix = Matrix::uninitialized();
            ffi::cogl_framebuffer_get_modelview_matrix(
                self.as_ref().to_glib_none().0,
                matrix.to_glib_none_mut().0,
            );
            matrix
        }
    }

    fn get_projection_matrix(&self) -> Matrix {
        unsafe {
            let mut matrix = Matrix::uninitialized();
            ffi::cogl_framebuffer_get_projection_matrix(
                self.as_ref().to_glib_none().0,
                matrix.to_glib_none_mut().0,
            );
            matrix
        }
    }

    fn get_red_bits(&self) -> i32 {
        unsafe { ffi::cogl_framebuffer_get_red_bits(self.as_ref().to_glib_none().0) }
    }

    fn get_samples_per_pixel(&self) -> i32 {
        unsafe { ffi::cogl_framebuffer_get_samples_per_pixel(self.as_ref().to_glib_none().0) }
    }

    fn get_stereo_mode(&self) -> StereoMode {
        unsafe {
            from_glib(ffi::cogl_framebuffer_get_stereo_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn get_viewport4fv(&self, viewport: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 4) {
    //    unsafe { TODO: call cogl_sys:cogl_framebuffer_get_viewport4fv() }
    //}

    fn get_viewport_height(&self) -> f32 {
        unsafe { ffi::cogl_framebuffer_get_viewport_height(self.as_ref().to_glib_none().0) }
    }

    fn get_viewport_width(&self) -> f32 {
        unsafe { ffi::cogl_framebuffer_get_viewport_width(self.as_ref().to_glib_none().0) }
    }

    fn get_viewport_x(&self) -> f32 {
        unsafe { ffi::cogl_framebuffer_get_viewport_x(self.as_ref().to_glib_none().0) }
    }

    fn get_viewport_y(&self) -> f32 {
        unsafe { ffi::cogl_framebuffer_get_viewport_y(self.as_ref().to_glib_none().0) }
    }

    fn get_width(&self) -> i32 {
        unsafe { ffi::cogl_framebuffer_get_width(self.as_ref().to_glib_none().0) }
    }

    fn identity_matrix(&self) {
        unsafe {
            ffi::cogl_framebuffer_identity_matrix(self.as_ref().to_glib_none().0);
        }
    }

    fn orthographic(&self, x_1: f32, y_1: f32, x_2: f32, y_2: f32, near: f32, far: f32) {
        unsafe {
            ffi::cogl_framebuffer_orthographic(
                self.as_ref().to_glib_none().0,
                x_1,
                y_1,
                x_2,
                y_2,
                near,
                far,
            );
        }
    }

    fn perspective(&self, fov_y: f32, aspect: f32, z_near: f32, z_far: f32) {
        unsafe {
            ffi::cogl_framebuffer_perspective(
                self.as_ref().to_glib_none().0,
                fov_y,
                aspect,
                z_near,
                z_far,
            );
        }
    }

    fn pop_clip(&self) {
        unsafe {
            ffi::cogl_framebuffer_pop_clip(self.as_ref().to_glib_none().0);
        }
    }

    fn pop_matrix(&self) {
        unsafe {
            ffi::cogl_framebuffer_pop_matrix(self.as_ref().to_glib_none().0);
        }
    }

    fn push_matrix(&self) {
        unsafe {
            ffi::cogl_framebuffer_push_matrix(self.as_ref().to_glib_none().0);
        }
    }

    fn push_primitive_clip(
        &self,
        primitive: &Primitive,
        bounds_x1: f32,
        bounds_y1: f32,
        bounds_x2: f32,
        bounds_y2: f32,
    ) {
        unsafe {
            ffi::cogl_framebuffer_push_primitive_clip(
                self.as_ref().to_glib_none().0,
                primitive.to_glib_none().0,
                bounds_x1,
                bounds_y1,
                bounds_x2,
                bounds_y2,
            );
        }
    }

    fn push_rectangle_clip(&self, x_1: f32, y_1: f32, x_2: f32, y_2: f32) {
        unsafe {
            ffi::cogl_framebuffer_push_rectangle_clip(
                self.as_ref().to_glib_none().0,
                x_1,
                y_1,
                x_2,
                y_2,
            );
        }
    }

    fn push_scissor_clip(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            ffi::cogl_framebuffer_push_scissor_clip(
                self.as_ref().to_glib_none().0,
                x,
                y,
                width,
                height,
            );
        }
    }

    fn read_pixels(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: PixelFormat,
        pixels: &[u8],
    ) -> bool {
        unsafe {
            ffi::cogl_framebuffer_read_pixels(
                self.as_ref().to_glib_none().0,
                x,
                y,
                width,
                height,
                format.to_glib(),
                pixels.to_glib_none().0,
            ) == crate::TRUE
        }
    }

    fn read_pixels_into_bitmap(
        &self,
        x: i32,
        y: i32,
        source: ReadPixelsFlags,
        bitmap: &Bitmap,
    ) -> bool {
        unsafe {
            ffi::cogl_framebuffer_read_pixels_into_bitmap(
                self.as_ref().to_glib_none().0,
                x,
                y,
                source.to_glib(),
                bitmap.to_glib_none().0,
            ) == crate::TRUE
        }
    }

    fn resolve_samples(&self) {
        unsafe {
            ffi::cogl_framebuffer_resolve_samples(self.as_ref().to_glib_none().0);
        }
    }

    fn resolve_samples_region(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            ffi::cogl_framebuffer_resolve_samples_region(
                self.as_ref().to_glib_none().0,
                x,
                y,
                width,
                height,
            );
        }
    }

    fn rotate(&self, angle: f32, x: f32, y: f32, z: f32) {
        unsafe {
            ffi::cogl_framebuffer_rotate(self.as_ref().to_glib_none().0, angle, x, y, z);
        }
    }

    fn rotate_euler(&self, euler: &Euler) {
        unsafe {
            ffi::cogl_framebuffer_rotate_euler(
                self.as_ref().to_glib_none().0,
                euler.to_glib_none().0,
            );
        }
    }

    fn rotate_quaternion(&self, quaternion: &Quaternion) {
        unsafe {
            ffi::cogl_framebuffer_rotate_quaternion(
                self.as_ref().to_glib_none().0,
                quaternion.to_glib_none().0,
            );
        }
    }

    fn scale(&self, x: f32, y: f32, z: f32) {
        unsafe {
            ffi::cogl_framebuffer_scale(self.as_ref().to_glib_none().0, x, y, z);
        }
    }

    fn set_color_mask(&self, color_mask: ColorMask) {
        unsafe {
            ffi::cogl_framebuffer_set_color_mask(
                self.as_ref().to_glib_none().0,
                color_mask.to_glib(),
            );
        }
    }

    fn set_depth_texture_enabled(&self, enabled: bool) {
        unsafe {
            ffi::cogl_framebuffer_set_depth_texture_enabled(
                self.as_ref().to_glib_none().0,
                enabled as i32,
            );
        }
    }

    fn set_depth_write_enabled(&self, depth_write_enabled: bool) {
        unsafe {
            ffi::cogl_framebuffer_set_depth_write_enabled(
                self.as_ref().to_glib_none().0,
                depth_write_enabled as i32,
            );
        }
    }

    fn set_dither_enabled(&self, dither_enabled: bool) {
        unsafe {
            ffi::cogl_framebuffer_set_dither_enabled(
                self.as_ref().to_glib_none().0,
                dither_enabled as i32,
            );
        }
    }

    fn set_modelview_matrix(&self, matrix: &Matrix) {
        unsafe {
            ffi::cogl_framebuffer_set_modelview_matrix(
                self.as_ref().to_glib_none().0,
                matrix.to_glib_none().0,
            );
        }
    }

    fn set_projection_matrix(&self, matrix: &Matrix) {
        unsafe {
            ffi::cogl_framebuffer_set_projection_matrix(
                self.as_ref().to_glib_none().0,
                matrix.to_glib_none().0,
            );
        }
    }

    fn set_samples_per_pixel(&self, samples_per_pixel: i32) {
        unsafe {
            ffi::cogl_framebuffer_set_samples_per_pixel(
                self.as_ref().to_glib_none().0,
                samples_per_pixel,
            );
        }
    }

    fn set_stereo_mode(&self, stereo_mode: StereoMode) {
        unsafe {
            ffi::cogl_framebuffer_set_stereo_mode(
                self.as_ref().to_glib_none().0,
                stereo_mode.to_glib(),
            );
        }
    }

    fn set_viewport(&self, x: f32, y: f32, width: f32, height: f32) {
        unsafe {
            ffi::cogl_framebuffer_set_viewport(self.as_ref().to_glib_none().0, x, y, width, height);
        }
    }

    fn transform(&self, matrix: &Matrix) {
        unsafe {
            ffi::cogl_framebuffer_transform(
                self.as_ref().to_glib_none().0,
                matrix.to_glib_none().0,
            );
        }
    }

    fn translate(&self, x: f32, y: f32, z: f32) {
        unsafe {
            ffi::cogl_framebuffer_translate(self.as_ref().to_glib_none().0, x, y, z);
        }
    }
}

impl fmt::Display for Framebuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Framebuffer")
    }
}

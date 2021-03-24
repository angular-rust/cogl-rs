use ffi;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    #[derive(Debug, Hash)] // PartialOrd, Ord,
    pub struct Color(Boxed<ffi::CoglColor>);

    match fn {
        copy => |ptr| ffi::cogl_color_copy(mut_override(ptr)),
        free => |ptr| ffi::cogl_color_free(ptr),
        get_type => || ffi::cogl_color_get_gtype(),
    }
}

impl Color {
    /// Creates a new (empty) color
    ///
    /// # Returns
    ///
    /// a newly-allocated `Color`. Use `Color::free`
    ///  to free the allocated resources
    pub fn new() -> Color {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::cogl_color_new())
        }
    }

    /// Retrieves the alpha channel of `self` as a fixed point
    /// value between 0 and 1.0.
    ///
    /// # Returns
    ///
    /// the alpha channel of the passed color
    pub fn get_alpha(&self) -> f32 {
        unsafe {
            ffi::cogl_color_get_alpha(self.to_glib_none().0)
        }
    }

    /// Retrieves the alpha channel of `self` as a byte value
    /// between 0 and 255
    ///
    /// # Returns
    ///
    /// the alpha channel of the passed color
    pub fn get_alpha_byte(&self) -> u8 {
        unsafe {
            ffi::cogl_color_get_alpha_byte(self.to_glib_none().0)
        }
    }

    /// Retrieves the alpha channel of `self` as a floating point
    /// value between 0.0 and 1.0
    ///
    /// # Returns
    ///
    /// the alpha channel of the passed color
    pub fn get_alpha_float(&self) -> f32 {
        unsafe {
            ffi::cogl_color_get_alpha_float(self.to_glib_none().0)
        }
    }

    /// Retrieves the blue channel of `self` as a fixed point
    /// value between 0 and 1.0.
    ///
    /// # Returns
    ///
    /// the blue channel of the passed color
    pub fn get_blue(&self) -> f32 {
        unsafe {
            ffi::cogl_color_get_blue(self.to_glib_none().0)
        }
    }

    /// Retrieves the blue channel of `self` as a byte value
    /// between 0 and 255
    ///
    /// # Returns
    ///
    /// the blue channel of the passed color
    pub fn get_blue_byte(&self) -> u8 {
        unsafe {
            ffi::cogl_color_get_blue_byte(self.to_glib_none().0)
        }
    }

    /// Retrieves the blue channel of `self` as a floating point
    /// value between 0.0 and 1.0
    ///
    /// # Returns
    ///
    /// the blue channel of the passed color
    pub fn get_blue_float(&self) -> f32 {
        unsafe {
            ffi::cogl_color_get_blue_float(self.to_glib_none().0)
        }
    }

    /// Retrieves the green channel of `self` as a fixed point
    /// value between 0 and 1.0.
    ///
    /// # Returns
    ///
    /// the green channel of the passed color
    pub fn get_green(&self) -> f32 {
        unsafe {
            ffi::cogl_color_get_green(self.to_glib_none().0)
        }
    }

    /// Retrieves the green channel of `self` as a byte value
    /// between 0 and 255
    ///
    /// # Returns
    ///
    /// the green channel of the passed color
    pub fn get_green_byte(&self) -> u8 {
        unsafe {
            ffi::cogl_color_get_green_byte(self.to_glib_none().0)
        }
    }

    /// Retrieves the green channel of `self` as a floating point
    /// value between 0.0 and 1.0
    ///
    /// # Returns
    ///
    /// the green channel of the passed color
    pub fn get_green_float(&self) -> f32 {
        unsafe {
            ffi::cogl_color_get_green_float(self.to_glib_none().0)
        }
    }

    /// Retrieves the red channel of `self` as a fixed point
    /// value between 0 and 1.0.
    ///
    /// # Returns
    ///
    /// the red channel of the passed color
    pub fn get_red(&self) -> f32 {
        unsafe {
            ffi::cogl_color_get_red(self.to_glib_none().0)
        }
    }

    /// Retrieves the red channel of `self` as a byte value
    /// between 0 and 255
    ///
    /// # Returns
    ///
    /// the red channel of the passed color
    pub fn get_red_byte(&self) -> u8 {
        unsafe {
            ffi::cogl_color_get_red_byte(self.to_glib_none().0)
        }
    }

    /// Retrieves the red channel of `self` as a floating point
    /// value between 0.0 and 1.0
    ///
    /// # Returns
    ///
    /// the red channel of the passed color
    pub fn get_red_float(&self) -> f32 {
        unsafe {
            ffi::cogl_color_get_red_float(self.to_glib_none().0)
        }
    }

    /// Sets the values of the passed channels into a `Color`
    /// ## `red`
    /// value of the red channel, between 0 and 1.0
    /// ## `green`
    /// value of the green channel, between 0 and 1.0
    /// ## `blue`
    /// value of the blue channel, between 0 and 1.0
    /// ## `alpha`
    /// value of the alpha channel, between 0 and 1.0
    pub fn init_from_4f(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            ffi::cogl_color_init_from_4f(self.to_glib_none_mut().0, red, green, blue, alpha);
        }
    }

    //TODO:
    // /// Sets the values of the passed channels into a `Color`
    // /// ## `color_array`
    // /// a pointer to an array of 4 float color components
    // pub fn init_from_4fv(&mut self, color_array: f32) {
    //     // unsafe {
    //     //     ffi::cogl_color_init_from_4fv(self.to_glib_none_mut().0, color_array);
    //     // }
    // }

    /// Sets the values of the passed channels into a `Color`.
    /// ## `red`
    /// value of the red channel, between 0 and 255
    /// ## `green`
    /// value of the green channel, between 0 and 255
    /// ## `blue`
    /// value of the blue channel, between 0 and 255
    /// ## `alpha`
    /// value of the alpha channel, between 0 and 255
    pub fn init_from_4ub(&mut self, red: u8, green: u8, blue: u8, alpha: u8) {
        unsafe {
            ffi::cogl_color_init_from_4ub(self.to_glib_none_mut().0, red, green, blue, alpha);
        }
    }

    /// Converts a non-premultiplied color to a pre-multiplied color. For
    /// example, semi-transparent red is (1.0, 0, 0, 0.5) when non-premultiplied
    /// and (0.5, 0, 0, 0.5) when premultiplied.
    pub fn premultiply(&mut self) {
        unsafe {
            ffi::cogl_color_premultiply(self.to_glib_none_mut().0);
        }
    }

    /// Sets the alpha channel of `self` to `alpha`.
    /// ## `alpha`
    /// a float value between 0.0f and 1.0f
    pub fn set_alpha(&mut self, alpha: f32) {
        unsafe {
            ffi::cogl_color_set_alpha(self.to_glib_none_mut().0, alpha);
        }
    }

    /// Sets the alpha channel of `self` to `alpha`.
    /// ## `alpha`
    /// a byte value between 0 and 255
    pub fn set_alpha_byte(&mut self, alpha: u8) {
        unsafe {
            ffi::cogl_color_set_alpha_byte(self.to_glib_none_mut().0, alpha);
        }
    }

    /// Sets the alpha channel of `self` to `alpha`.
    /// ## `alpha`
    /// a float value between 0.0f and 1.0f
    pub fn set_alpha_float(&mut self, alpha: f32) {
        unsafe {
            ffi::cogl_color_set_alpha_float(self.to_glib_none_mut().0, alpha);
        }
    }

    /// Sets the blue channel of `self` to `blue`.
    /// ## `blue`
    /// a float value between 0.0f and 1.0f
    pub fn set_blue(&mut self, blue: f32) {
        unsafe {
            ffi::cogl_color_set_blue(self.to_glib_none_mut().0, blue);
        }
    }

    /// Sets the blue channel of `self` to `blue`.
    /// ## `blue`
    /// a byte value between 0 and 255
    pub fn set_blue_byte(&mut self, blue: u8) {
        unsafe {
            ffi::cogl_color_set_blue_byte(self.to_glib_none_mut().0, blue);
        }
    }

    /// Sets the blue channel of `self` to `blue`.
    /// ## `blue`
    /// a float value between 0.0f and 1.0f
    pub fn set_blue_float(&mut self, blue: f32) {
        unsafe {
            ffi::cogl_color_set_blue_float(self.to_glib_none_mut().0, blue);
        }
    }

    /// Sets the green channel of `self` to `green`.
    /// ## `green`
    /// a float value between 0.0f and 1.0f
    pub fn set_green(&mut self, green: f32) {
        unsafe {
            ffi::cogl_color_set_green(self.to_glib_none_mut().0, green);
        }
    }

    /// Sets the green channel of `self` to `green`.
    /// ## `green`
    /// a byte value between 0 and 255
    pub fn set_green_byte(&mut self, green: u8) {
        unsafe {
            ffi::cogl_color_set_green_byte(self.to_glib_none_mut().0, green);
        }
    }

    /// Sets the green channel of `self` to `green`.
    /// ## `green`
    /// a float value between 0.0f and 1.0f
    pub fn set_green_float(&mut self, green: f32) {
        unsafe {
            ffi::cogl_color_set_green_float(self.to_glib_none_mut().0, green);
        }
    }

    /// Sets the red channel of `self` to `red`.
    /// ## `red`
    /// a float value between 0.0f and 1.0f
    pub fn set_red(&mut self, red: f32) {
        unsafe {
            ffi::cogl_color_set_red(self.to_glib_none_mut().0, red);
        }
    }

    /// Sets the red channel of `self` to `red`.
    /// ## `red`
    /// a byte value between 0 and 255
    pub fn set_red_byte(&mut self, red: u8) {
        unsafe {
            ffi::cogl_color_set_red_byte(self.to_glib_none_mut().0, red);
        }
    }

    /// Sets the red channel of `self` to `red`.
    /// ## `red`
    /// a float value between 0.0f and 1.0f
    pub fn set_red_float(&mut self, red: f32) {
        unsafe {
            ffi::cogl_color_set_red_float(self.to_glib_none_mut().0, red);
        }
    }

    /// Converts `self` to the HLS format.
    ///
    /// The `hue` value is in the 0 .. 360 range. The `luminance` and
    /// `saturation` values are in the 0 .. 1 range.
    /// ## `hue`
    /// return location for the hue value or `None`
    /// ## `saturation`
    /// return location for the saturation value or `None`
    /// ## `luminance`
    /// return location for the luminance value or `None`
    pub fn to_hsl(&self) -> (f32, f32, f32) {
        unsafe {
            let mut hue = mem::MaybeUninit::uninit();
            let mut saturation = mem::MaybeUninit::uninit();
            let mut luminance = mem::MaybeUninit::uninit();
            ffi::cogl_color_to_hsl(self.to_glib_none().0, hue.as_mut_ptr(), saturation.as_mut_ptr(), luminance.as_mut_ptr());
            let hue = hue.assume_init();
            let saturation = saturation.assume_init();
            let luminance = luminance.assume_init();
            (hue, saturation, luminance)
        }
    }

    /// Converts a pre-multiplied color to a non-premultiplied color. For
    /// example, semi-transparent red is (0.5, 0, 0, 0.5) when premultiplied
    /// and (1.0, 0, 0, 0.5) when non-premultiplied.
    pub fn unpremultiply(&mut self) {
        unsafe {
            ffi::cogl_color_unpremultiply(self.to_glib_none_mut().0);
        }
    }

    //pub fn equal(v1: /*Unimplemented*/Option<Fundamental: Pointer>, v2: /*Unimplemented*/Option<Fundamental: Pointer>) -> Bool {
    //    unsafe { TODO: call cogl_sys:cogl_color_equal() }
    //}

    //TODO:
    // /// Converts a color expressed in HLS (hue, luminance and saturation)
    // /// values into a `Color`.
    // /// ## `color`
    // /// return location for a `Color`
    // /// ## `hue`
    // /// hue value, in the 0 .. 360 range
    // /// ## `saturation`
    // /// saturation value, in the 0 .. 1 range
    // /// ## `luminance`
    // /// luminance value, in the 0 .. 1 range
    // pub fn init_from_hsl(hue: f32, saturation: f32, luminance: f32) -> Color {
    //     // assert_initialized_main_thread!();
    //     // unsafe {
    //     //     let mut color = Color::uninitialized();
    //     //     ffi::cogl_color_init_from_hsl(color.to_glib_none_mut().0, hue, saturation, luminance);
    //     //     color
    //     // }
    // }
}

impl Default for Color {
    fn default() -> Self {
        Self::new()
    }
}

//TODO:
// impl PartialEq for Color {
//     #[inline]
//     fn eq(&self, other: &Self) -> bool {
//         // self.equal(other)
//     }
// }

// impl Eq for Color {}

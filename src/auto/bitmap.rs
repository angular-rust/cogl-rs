use crate::Bool;
use ffi;
use glib;
use glib::translate::*;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Bitmap(Object<ffi::CoglBitmap, BitmapClass>);

    match fn {
        get_type => || ffi::cogl_bitmap_get_gtype(),
    }
}

impl Bitmap {
    pub fn from_file(filename: &str) -> Result<Bitmap, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::cogl_bitmap_new_from_file(filename.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn get_size_from_file(filename: &str) -> (Bool, i32, i32) {
        assert_initialized_main_thread!();
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            let ret = ffi::cogl_bitmap_get_size_from_file(
                filename.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            let width = width.assume_init();
            let height = height.assume_init();
            (ret, width, height)
        }
    }
}

impl fmt::Display for Bitmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bitmap")
    }
}

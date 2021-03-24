use ffi;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Fixed(Object<ffi::CoglFixed, FixedClass>);

    match fn {
        get_type => || ffi::cogl_fixed_get_type(),
    }
}

impl Fixed {}

impl fmt::Display for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fixed")
    }
}

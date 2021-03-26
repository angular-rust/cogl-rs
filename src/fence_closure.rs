use glib::translate::*;
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FenceClosure {
    //TODO:
}

#[doc(hidden)]
impl Uninitialized for FenceClosure {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::CoglFenceClosure> for FenceClosure {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::CoglFenceClosure, Self> {
        let ptr: *const FenceClosure = &*self;
        Stash(ptr as *const ffi::CoglFenceClosure, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglFenceClosure> for FenceClosure {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglFenceClosure, Self> {
        let ptr: *mut FenceClosure = &mut *self;
        StashMut(ptr as *mut ffi::CoglFenceClosure, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::CoglFenceClosure> for FenceClosure {
    unsafe fn from_glib_none(ptr: *const ffi::CoglFenceClosure) -> Self {
        *(ptr as *const FenceClosure)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::CoglFenceClosure> for FenceClosure {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglFenceClosure) -> Self {
        *(ptr as *mut FenceClosure)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::CoglFenceClosure> for FenceClosure {
    unsafe fn from_glib_borrow(ptr: *mut ffi::CoglFenceClosure) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut FenceClosure))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::CoglFenceClosure> for FenceClosure {
    unsafe fn from_glib_borrow(
        ptr: *const ffi::CoglFenceClosure,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const FenceClosure))
    }
}

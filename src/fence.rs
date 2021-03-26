use glib::translate::*;
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Fence {
    //TODO:
}

#[doc(hidden)]
impl Uninitialized for Fence {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::CoglFence> for Fence {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::CoglFence, Self> {
        let ptr: *const Fence = &*self;
        Stash(ptr as *const ffi::CoglFence, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglFence> for Fence {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglFence, Self> {
        let ptr: *mut Fence = &mut *self;
        StashMut(ptr as *mut ffi::CoglFence, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::CoglFence> for Fence {
    unsafe fn from_glib_none(ptr: *const ffi::CoglFence) -> Self {
        *(ptr as *const Fence)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::CoglFence> for Fence {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglFence) -> Self {
        *(ptr as *mut Fence)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::CoglFence> for Fence {
    unsafe fn from_glib_borrow(ptr: *mut ffi::CoglFence) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut Fence))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::CoglFence> for Fence {
    unsafe fn from_glib_borrow(ptr: *const ffi::CoglFence) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const Fence))
    }
}

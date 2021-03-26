use glib::translate::*;
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DebugObjectTypeInfo {
    //TODO:
// pub name: *const c_char,
// pub instance_count: c_ulong,
}

#[doc(hidden)]
impl Uninitialized for DebugObjectTypeInfo {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::CoglDebugObjectTypeInfo> for DebugObjectTypeInfo {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::CoglDebugObjectTypeInfo, Self> {
        let ptr: *const DebugObjectTypeInfo = &*self;
        Stash(ptr as *const ffi::CoglDebugObjectTypeInfo, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::CoglDebugObjectTypeInfo> for DebugObjectTypeInfo {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::CoglDebugObjectTypeInfo, Self> {
        let ptr: *mut DebugObjectTypeInfo = &mut *self;
        StashMut(ptr as *mut ffi::CoglDebugObjectTypeInfo, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::CoglDebugObjectTypeInfo> for DebugObjectTypeInfo {
    unsafe fn from_glib_none(ptr: *const ffi::CoglDebugObjectTypeInfo) -> Self {
        *(ptr as *const DebugObjectTypeInfo)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::CoglDebugObjectTypeInfo> for DebugObjectTypeInfo {
    unsafe fn from_glib_none(ptr: *mut ffi::CoglDebugObjectTypeInfo) -> Self {
        *(ptr as *mut DebugObjectTypeInfo)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::CoglDebugObjectTypeInfo> for DebugObjectTypeInfo {
    unsafe fn from_glib_borrow(
        ptr: *mut ffi::CoglDebugObjectTypeInfo,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut DebugObjectTypeInfo))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::CoglDebugObjectTypeInfo> for DebugObjectTypeInfo {
    unsafe fn from_glib_borrow(
        ptr: *const ffi::CoglDebugObjectTypeInfo,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const DebugObjectTypeInfo))
    }
}

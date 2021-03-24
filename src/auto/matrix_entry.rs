#![allow(dead_code)]

use crate::{Bool, Matrix};
use ffi;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    #[derive(Debug, Hash)] // PartialOrd, Ord,
    pub struct MatrixEntry(Shared<ffi::CoglMatrixEntry>);

    match fn {
        ref => |ptr| ffi::cogl_matrix_entry_ref(ptr),
        unref => |ptr| ffi::cogl_matrix_entry_unref(ptr),
        get_type => || ffi::cogl_matrix_entry_get_gtype(),
    }
}

impl MatrixEntry {
    /// Determines if the only difference between two transforms is a
    /// translation and if so returns what the `x`, `y`, and `z` components of
    /// the translation are.
    ///
    /// If the difference between the two translations involves anything
    /// other than a translation then the function returns `false`.
    /// ## `entry1`
    /// A second reference transform
    /// ## `x`
    /// The destination for the x-component of the translation
    /// ## `y`
    /// The destination for the y-component of the translation
    /// ## `z`
    /// The destination for the z-component of the translation
    ///
    /// # Returns
    ///
    /// `true` if the only difference between the transform of
    ///  `self` and the transform of `entry1` is a translation,
    ///  otherwise `false`.
    pub fn calculate_translation(&self, entry1: &MatrixEntry) -> (Bool, f32, f32, f32) {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let mut z = mem::MaybeUninit::uninit();
            let ret = ffi::cogl_matrix_entry_calculate_translation(
                self.to_glib_none().0,
                entry1.to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                z.as_mut_ptr(),
            );
            let x = x.assume_init();
            let y = y.assume_init();
            let z = z.assume_init();
            (ret, x, y, z)
        }
    }

    /// Compares two arbitrary `MatrixEntry` transforms for equality
    /// returning `true` if they are equal or `false` otherwise.
    ///
    /// `<note>`In many cases it is unnecessary to use this api and instead
    /// direct pointer comparisons of entries are good enough and much
    /// cheaper too.`</note>`
    /// ## `entry1`
    /// A second `MatrixEntry` to compare
    ///
    /// # Returns
    ///
    /// `true` if `self` represents the same transform as
    ///  `entry1`, otherwise `false`.
    fn equal(&self, entry1: &MatrixEntry) -> Bool {
        unsafe { ffi::cogl_matrix_entry_equal(self.to_glib_none().0, entry1.to_glib_none().0) }
    }

    //TODO:
    // /// Resolves the current `self` transform into a `Matrix` by
    // /// combining the sequence of operations that have been applied to
    // /// build up the current transform.
    // ///
    // /// There are two possible ways that this function may return its
    // /// result depending on whether it's possible to directly point
    // /// to an internal `Matrix` or whether the result needs to be
    // /// composed of multiple operations.
    // ///
    // /// If an internal matrix contains the required result then this
    // /// function will directly return a pointer to that matrix, otherwise
    // /// if the function returns `None` then `matrix` will be initialized
    // /// to match the transform of `self`.
    // ///
    // /// `<note>``matrix` will be left untouched if a direct pointer is
    // /// returned.`</note>`
    // /// ## `matrix`
    // /// The potential destination for the transform as
    // ///  a matrix
    // ///
    // /// # Returns
    // ///
    // /// A direct pointer to a `Matrix` transform or `None`
    // ///  and in that case `matrix` will be initialized with
    // ///  the effective transform represented by `self`.
    // pub fn get(&self) -> (Matrix, Matrix) {
    //     // unsafe {
    //     //     let mut matrix = Matrix::uninitialized();
    //     //     let ret = from_glib_full(ffi::cogl_matrix_entry_get(
    //     //         self.to_glib_none().0,
    //     //         matrix.to_glib_none_mut().0,
    //     //     ));
    //     //     (ret, matrix)
    //     // }
    // }

    /// Determines whether `self` is known to represent an identity
    /// transform.
    ///
    /// If this returns `true` then the entry is definitely the identity
    /// matrix. If it returns `false` it may or may not be the identity
    /// matrix but no expensive comparison is performed to verify it.
    ///
    /// # Returns
    ///
    /// `true` if `self` is definitely an identity transform,
    ///  otherwise `false`.
    pub fn is_identity(&self) -> Bool {
        unsafe { ffi::cogl_matrix_entry_is_identity(self.to_glib_none().0) }
    }
}

//TODO:
// impl PartialEq for MatrixEntry {
//     #[inline]
//     fn eq(&self, other: &Self) -> bool {
//         // self.equal(other)
//     }
// }

// impl Eq for MatrixEntry {}

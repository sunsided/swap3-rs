//! # swap3
//!
//! Provides utility functions for simultaneously swapping three values by rotating them
//! either left (`abc` → `bca`) or right (`abc` → `cab`). These functions can come in handy e.g.
//! when rotating elements of a binary tree in list representation.
//!
//! The provided functions work on arbitrary types and do *not* require the type to be [`Clone`], [`Copy`]
//! or [`Default`].
//!
//! ## Crate features
//!
//! * `unsafe` - The `unsafe` feature enables the use of (potentially faster) unsafe code.
//!              It is disabled by default; when disabled, `forbid(unsafe_code)` is implied.
//!
//! ## Examples
//!
//! For individual references, the [`swap3_bca`] (rotate left) and [`swap3_cab`] (rotate right)
//! functions are available:
//!
//! ```
//! let mut a = 10;
//! let mut b = 20;
//! let mut c = 30;
//!
//! swap3::swap3_bca(&mut a, &mut b, &mut c);
//! assert_eq!([a, b, c], [20, 30, 10]);
//! ```
//!
//! For slices, the [`swap3_bca_slice`] and [`swap3_cab_slice`] functions can be used:
//!
//! ```
//! let mut vec = vec![10, 20, 30, 40, 50, 60];
//! swap3::swap3_bca_slice(&mut vec, 0, 1, 4);
//! assert_eq!(vec, &[30, 50, 30, 40, 10, 60]);
//! ```

//  SPDX-FileCopyrightText: 2023 Markus Mayer
//  SPDX-License-Identifier: MIT

#![cfg_attr(feature = "unsafe", allow(unsafe_code))]
#![cfg_attr(not(feature = "unsafe"), forbid(unsafe_code))]
// only enables the `doc_cfg` feature when
// the `docsrs` configuration attribute is defined
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Rotates three values to the left.
///
/// ## Arguments
///
/// * `a` - The first value, to be assigned with the value of `b`.
/// * `b` - The second value, to be assigned with the value of `c`.
/// * `c` - The third value, to be assigned with the value of `a`.
///
/// ## Example
///
/// ```
/// let mut a = 10;
/// let mut b = 20;
/// let mut c = 30;
/// swap3::swap3_bca(&mut a, &mut b, &mut c);
/// assert_eq!([a, b, c], [20, 30, 10]);
/// ```
pub fn swap3_bca<T>(a: &mut T, b: &mut T, c: &mut T) {
    std::mem::swap(a, b);
    std::mem::swap(b, c);
}

/// Rotates three values to the right.
///
/// ## Arguments
///
/// * `a` - The first value, to be assigned with the value of `c`.
/// * `b` - The second value, to be assigned with the value of `a`.
/// * `c` - The third value, to be assigned with the value of `b`.
///
/// ## Example
///
/// ```
/// let mut a = 10;
/// let mut b = 20;
/// let mut c = 30;
/// swap3::swap3_cab(&mut a, &mut b, &mut c);
/// assert_eq!([a, b, c], [30, 10, 20]);
/// ```
pub fn swap3_cab<T>(a: &mut T, b: &mut T, c: &mut T) {
    std::mem::swap(a, c);
    std::mem::swap(b, c);
}

/// Rotates three values to the left.
///
/// ## Arguments
///
/// * `data` - The slice whose elements to swap.
/// * `a` - The first index, to be assigned with the value of `data[b]`.
/// * `b` - The second index, to be assigned with the value of `data[c]`.
/// * `c` - The third index, to be assigned with the value of `data[a]`.
///
/// ## Example
///
/// ```
/// let mut vec = vec![50, 10, 90, 25, 30, 75];
/// swap3::swap3_bca_slice(&mut vec, 0, 1, 4);
/// assert_eq!(vec, &[10, 30, 90, 25, 50, 75]);
/// ```
#[inline(always)]
pub fn swap3_bca_slice<T>(data: &mut [T], a: usize, b: usize, c: usize) {
    #[cfg(feature = "unsafe")]
    slice::bca_unsafe(data, a, b, c);
    #[cfg(not(feature = "unsafe"))]
    slice::bca_safe(data, a, b, c);
}

/// Rotates three values to the right.
///
/// ## Arguments
///
/// * `data` - The slice whose elements to swap.
/// * `a` - The first index, to be assigned with the value of `data[c]`.
/// * `b` - The second index, to be assigned with the value of `data[a]`.
/// * `c` - The third index, to be assigned with the value of `data[b]`.
///
/// ## Example
///
/// ```
/// let mut vec = vec![50, 10, 90, 25, 30, 75];
/// swap3::swap3_bca_slice(&mut vec, 0, 1, 4);
/// assert_eq!(vec, &[10, 30, 90, 25, 50, 75]);
/// ```
#[inline(always)]
pub fn swap3_cab_slice<T>(data: &mut [T], a: usize, b: usize, c: usize) {
    #[cfg(feature = "unsafe")]
    slice::cab_unsafe(data, a, b, c);
    #[cfg(not(feature = "unsafe"))]
    slice::cab_safe(data, a, b, c);
}

pub mod slice {
    /// Rotates three values to the left.
    ///
    /// ## Arguments
    ///
    /// * `data` - The slice whose elements to swap.
    /// * `a` - The first index, to be assigned with the value of `data[b]`.
    /// * `b` - The second index, to be assigned with the value of `data[c]`.
    /// * `c` - The third index, to be assigned with the value of `data[a]`.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut vec = vec![50, 10, 90, 25, 30, 75];
    /// swap3::slice::bca_safe(&mut vec, 0, 1, 4);
    /// assert_eq!(vec, &[10, 30, 90, 25, 50, 75]);
    /// ```
    #[inline(always)]
    pub fn bca_safe<T>(data: &mut [T], a: usize, b: usize, c: usize) {
        data.swap(a, b);
        data.swap(b, c);
    }

    /// Rotates three values to the left.
    ///
    /// ## Arguments
    ///
    /// * `data` - The slice whose elements to swap.
    /// * `a` - The first index, to be assigned with the value of `data[b]`.
    /// * `b` - The second index, to be assigned with the value of `data[c]`.
    /// * `c` - The third index, to be assigned with the value of `data[a]`.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut vec = vec![50, 10, 90, 25, 30, 75];
    /// swap3::slice::bca_unsafe(&mut vec, 0, 1, 4);
    /// assert_eq!(vec, &[10, 30, 90, 25, 50, 75]);
    /// ```
    #[cfg_attr(docsrs, doc(cfg(feature = "unsafe")))]
    #[cfg(feature = "unsafe")]
    #[inline(always)]
    pub fn bca_unsafe<T>(data: &mut [T], a: usize, b: usize, c: usize) {
        // NOTE: This code is taken from the implementation of slice::swap and extended for three values.
        //       The original code was licensed under an MIT license by The Rust Core Library authors.
        use std::ptr;

        let pa = ptr::addr_of_mut!(data[a]);
        let pb = ptr::addr_of_mut!(data[b]);
        let pc = ptr::addr_of_mut!(data[c]);
        // SAFETY: `pa`, `pb` and `pc` have been created from safe mutable references and refer
        // to elements in the slice and therefore are guaranteed to be valid and aligned.
        // Note that accessing the elements behind `a`, `b` and `c` is checked and will
        // panic when out of bounds.
        unsafe {
            ptr::swap(pa, pb);
            ptr::swap(pb, pc);
            // ptr::swap_nonoverlapping(pa, pb, 1);
            // ptr::swap_nonoverlapping(pb, pc, 1);
        }
    }

    /// Rotates three values to the right.
    ///
    /// ## Arguments
    ///
    /// * `data` - The slice whose elements to swap.
    /// * `a` - The first index, to be assigned with the value of `data[c]`.
    /// * `b` - The second index, to be assigned with the value of `data[a]`.
    /// * `c` - The third index, to be assigned with the value of `data[b]`.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut vec = vec![50, 10, 90, 25, 30, 75];
    /// swap3::slice::cab_safe(&mut vec, 0, 1, 4);
    /// assert_eq!(vec, &[30, 50, 90, 25, 10, 75]);
    /// ```
    #[inline(always)]
    pub fn cab_safe<T>(data: &mut [T], a: usize, b: usize, c: usize) {
        data.swap(a, c);
        data.swap(b, c);
    }

    /// Rotates three values to the left.
    ///
    /// ## Arguments
    ///
    /// * `data` - The slice whose elements to swap.
    /// * `a` - The first index, to be assigned with the value of `data[b]`.
    /// * `b` - The second index, to be assigned with the value of `data[c]`.
    /// * `c` - The third index, to be assigned with the value of `data[a]`.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut vec = vec![50, 10, 90, 25, 30, 75];
    /// swap3::slice::cab_unsafe(&mut vec, 0, 1, 4);
    /// assert_eq!(vec, &[30, 50, 90, 25, 10, 75]);
    /// ```
    #[cfg_attr(docsrs, doc(cfg(feature = "unsafe")))]
    #[cfg(feature = "unsafe")]
    #[inline(always)]
    pub fn cab_unsafe<T>(data: &mut [T], a: usize, b: usize, c: usize) {
        // NOTE: This code is taken from the implementation of slice::swap and extended for three values.
        //       The original code was licensed under an MIT license by The Rust Core Library authors.
        use std::ptr;

        let pa = ptr::addr_of_mut!(data[a]);
        let pb = ptr::addr_of_mut!(data[b]);
        let pc = ptr::addr_of_mut!(data[c]);
        // SAFETY: `pa`, `pb` and `pc` have been created from safe mutable references and refer
        // to elements in the slice and therefore are guaranteed to be valid and aligned.
        // Note that accessing the elements behind `a`, `b` and `c` is checked and will
        // panic when out of bounds.
        unsafe {
            ptr::swap(pa, pc);
            ptr::swap(pb, pc);
            // ptr::swap_nonoverlapping(pa, pb, 1);
            // ptr::swap_nonoverlapping(pb, pc, 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap3_bca() {
        let mut a = 65;
        let mut b = 66;
        let mut c = 67;
        swap3_bca(&mut a, &mut b, &mut c);
        assert_eq!([a, b, c], [66, 67, 65]);
    }

    #[test]
    fn test_swap3_cab() {
        let mut a = 65;
        let mut b = 66;
        let mut c = 67;
        swap3_cab(&mut a, &mut b, &mut c);
        assert_eq!([a, b, c], [67, 65, 66]);
    }

    #[test]
    fn test_swap3_bca_vec() {
        let mut vec = vec![50, 10, 90, 25, 30, 75];
        slice::bca_safe(&mut vec, 0, 1, 4);
        assert_eq!(vec, &[10, 30, 90, 25, 50, 75]);
    }

    #[test]
    #[cfg(feature = "unsafe")]
    fn test_swap3_bca_vec_unsafe() {
        let mut vec = vec![50, 10, 90, 25, 30, 75];
        slice::bca_unsafe(&mut vec, 0, 1, 4);
        assert_eq!(vec, &[10, 30, 90, 25, 50, 75]);
    }

    #[test]
    fn test_swap3_cab_vec() {
        let mut vec = vec![50, 10, 90, 25, 30, 75];
        slice::cab_safe(&mut vec, 0, 1, 4);
        assert_eq!(vec, &[30, 50, 90, 25, 10, 75]);
    }

    #[test]
    #[cfg(feature = "unsafe")]
    fn test_swap3_cab_vec_unsafe() {
        let mut vec = vec![50, 10, 90, 25, 30, 75];
        slice::cab_unsafe(&mut vec, 0, 1, 4);
        assert_eq!(vec, &[30, 50, 90, 25, 10, 75]);
    }
}

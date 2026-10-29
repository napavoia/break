use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use crate::math::error::{Error, Result};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Vector<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> Vector<T, N> {
    #[inline]
    pub const fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    /// Sets the element at the given index to a new value.
    ///
    /// # Errors
    ///
    /// Returns [`Error::IndexOutOfBounds`] if `idx >= self.len()`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = r#break::vector!(10, 40, 30);
    /// let _ = v.set(1, 20);
    /// assert_eq!(Some(&20), v.get(1));
    ///
    /// assert!(v.set(5, 0).is_err());
    /// ```
    #[inline]
    pub fn try_set(&mut self, idx: usize, val: T) -> Result<()> {
        if idx >= self.len() {
            return Err(Error::IndexOutOfBounds {
                index: idx,
                len: self.len(),
            });
        }

        self.data[idx] = val;

        Ok(())
    }

    /// Sets the element at the given index to a new value.
    ///
    /// # Panics
    ///
    /// Panics if `idx` is out of bounds (i.e., `idx >= self.len()`).
    /// # Examples
    ///
    /// ```
    /// let mut v = r#break::vector!(10, 40, 30);
    /// v.set(1, 20);
    /// assert_eq!(Some(&20), v.get(1));
    /// assert_eq!(Some(&[10, 20, 30][..]), v.get(..));
    /// ```
    #[inline]
    pub fn set(&mut self, idx: usize, val: T) {
        self.data[idx] = val;
    }

    /// Returns a reference to an element or subslice depending on the type of
    /// index.
    ///
    /// - If given a position, returns a reference to the element at that
    ///   position or `None` if out of bounds.
    /// - If given a range, returns the subslice corresponding to that range,
    ///   or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = r#break::vector!(10, 40, 30);
    /// assert_eq!(Some(&40), v.get(1));
    /// assert_eq!(Some(&[10, 40][..]), v.get(0..2));
    /// assert_eq!(None, v.get(3));
    /// assert_eq!(None, v.get(0..4));
    /// ```
    #[inline]
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[T]>,
    {
        self.data.get(index)
    }

    /// Returns a raw pointer to the slice’s buffer.
    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    /// Returns an mutable pointer to the slice’s buffer.
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }
}

impl<T: Default, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        Self::from(core::array::from_fn(|_| T::default()))
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(data: [T; N]) -> Self {
        Self { data }
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<Axis> for Vector<T, N> {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

impl<T, const N: usize> Index<Axis> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: Axis) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl<T: Debug, const N: usize> ToString for Vector<T, N> {
    fn to_string(&self) -> String {
        format!("Vector{:?}", self.data)
    }
}

macro_rules! impl_split {
    (2; $in:expr => $out:expr) => {
        impl<T: Copy> Vector<T, $in> {
            pub fn split(self) -> (Vector<T, $out>, Vector<T, $out>) {
                let mut it = self.data.into_iter();
                (
                    $crate::math::Vector::from(core::array::from_fn(|_| it.next().unwrap())),
                    $crate::math::Vector::from(core::array::from_fn(|_| it.next().unwrap())),
                )
            }
        }
    };

    (3; $in:expr => $out:expr) => {
        impl<T: Copy> Vector<T, $in> {
            pub fn split(self) -> (Vector<T, $out>, Vector<T, $out>, Vector<T, $out>) {
                let mut it = self.data.into_iter();
                (
                    $crate::math::Vector::from(core::array::from_fn(|_| it.next().unwrap())),
                    $crate::math::Vector::from(core::array::from_fn(|_| it.next().unwrap())),
                    $crate::math::Vector::from(core::array::from_fn(|_| it.next().unwrap())),
                )
            }
        }
    };

    (2; $($a:expr => $b:expr),+) => {
        $(impl_split!(2; $a => $b);)+
    };

    (3; $($a:expr => $b:expr),+) => {
        $(impl_split!(3; $a => $b);)+
    };
}

impl_split!(2;
    16 => 8,
    8 => 4,
    6 => 3,
    4 => 2,
    2 => 1
);

impl_split!(3;
    9 => 3,
    3 => 1
);

/// A coordinate axis in a multidimensional vector.
///
/// Used as an index to access components of a [`Vector`] via the
/// [`Index`] implementation. Discriminant values correspond to the
/// position of the component in the `data` array (see `#[repr(usize)]`).
///
/// # Example
///
/// ```
/// use r#break::math::{Vector, Axis};
///
/// let mut v = Vector::<f32, 4>::default();
/// v[Axis::X] = 1.0;
/// v[Axis::Y] = 2.0;
/// v[Axis::Z] = 3.0;
/// v[Axis::W] = 4.0;
///
/// assert_eq!(v[Axis::X], 1.0);
/// assert_eq!(v[Axis::W], 4.0);
/// ```
#[repr(usize)]
pub enum Axis {
    /// The X axis (index `0`).
    X = 0,
    /// The Y axis (index `1`).
    Y = 1,
    /// The Z axis (index `2`).
    Z = 2,
    /// The W axis (index `3`).
    W = 3,
}

impl From<Axis> for usize {
    fn from(val: Axis) -> Self {
        val as usize
    }
}

#[inline]
pub const fn zeroed() -> Vector<u8, 0> {
    Vector { data: [] }
}

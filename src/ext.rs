use {
    crate::{consumed::Consumed, joined::Joined, repeated::Repeated},
    core::fmt,
};

pub trait DisplayExt: fmt::Display {
    #[inline]
    fn repeated(&self, n: usize) -> Repeated<'_, Self> {
        Repeated { display: self, n }
    }
}

impl<T> DisplayExt for T where T: fmt::Display {}

/// Extension trait for references that are
/// [`IntoIterator`](::core::iter::IntoIterator).
///
/// This trait is implemented for any type from a reference
/// to which an iterator can be created. Also the implementation
/// requires item to be a [`Display`](::core::fmt::Display).
/// If a type doesn't satisfy this requirement, consider to create
/// a [`consumed`](crate::ext::IntoIteratorExt::consumed) iterator
/// that can be used by shared reference.
pub trait IntoIteratorByRefExt {
    /// Prints items separated by the specified separator.
    ///
    /// # Examples
    /// ```
    /// use fmtex::prelude::*;
    ///
    /// let s = [1, 2, 3].joined(", ").to_string();
    /// assert_eq!(s, "1, 2, 3");
    /// ```
    #[inline]
    fn joined<S>(&self, sepr: S) -> Joined<&Self, S>
    where
        S: fmt::Display,
    {
        Joined { iter: self, sepr }
    }
}

impl<I> IntoIteratorByRefExt for I
where
    I: ?Sized,
    for<'it> &'it I: IntoIterator<Item: fmt::Display>,
{
}

/// Extension trait for [`IntoIterator`](::core::iter::IntoIterator).
pub trait IntoIteratorExt: IntoIterator {
    /// Creates a consumed iterator, allowing it
    /// to be used by a shared reference.
    ///
    /// # Examples
    /// ```
    /// use fmtex::prelude::*;
    ///
    /// let s = (1..4).consumed().joined(", ").to_string();
    /// assert_eq!(s, "1, 2, 3");
    /// ```
    ///
    /// # Note
    /// Be careful, the iterator is exhausted each time it is used:
    /// ```
    /// use fmtex::prelude::*;
    ///
    /// let it = (1..4).consumed();
    ///
    /// let s = it.joined(", ").to_string();
    /// assert_eq!(s, "1, 2, 3");
    ///
    /// // now it's exhausted
    /// let s = it.joined(", ").to_string();
    /// assert_eq!(s, "");
    /// ```
    #[inline]
    fn consumed(self) -> Consumed<Self::IntoIter>
    where
        Self: Sized,
    {
        Consumed::new(self.into_iter())
    }
}

impl<I> IntoIteratorExt for I where I: IntoIterator {}

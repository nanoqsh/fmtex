use core::{cell::Cell, fmt, iter::FusedIterator};

/// Extension trait for references that are
/// [`IntoIterator`](core::iter::IntoIterator).
///
/// This trait is implemented for any type from a reference
/// to which an iterator can be created. Also the implementation
/// requires item to be a [`Display`](core::fmt::Display).
/// If a type doesn't satisfy this requirement, consider to create
/// a [`consumed`](IntoIteratorExt::consumed) iterator
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

/// Display implementer for the
/// [`joined`](IntoIteratorByRefExt::joined) method.
#[derive(Clone, Copy, Debug)]
pub struct Joined<I, S> {
    iter: I,
    sepr: S,
}

impl<'it, I, S> fmt::Display for Joined<&'it I, S>
where
    I: ?Sized,
    &'it I: IntoIterator<Item: fmt::Display>,
    S: fmt::Display,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iter = self.iter.into_iter();
        let sepr = &self.sepr;

        let Some(first) = iter.next() else {
            return Ok(());
        };

        write!(f, "{first}")?;
        for item in iter {
            write!(f, "{sepr}{item}")?;
        }

        Ok(())
    }
}

/// Extension trait for [`IntoIterator`](core::iter::IntoIterator).
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

/// Reference iterator for the
/// [`consumed`](IntoIteratorExt::consumed) method.
pub struct Consumed<I> {
    state: Cell<Option<I>>,
}

impl<I> Consumed<I> {
    #[inline]
    pub(crate) fn new(iter: I) -> Self {
        Self {
            state: Cell::new(Some(iter)),
        }
    }
}

impl<I> Default for Consumed<I> {
    #[inline]
    fn default() -> Self {
        Self {
            state: Cell::default(),
        }
    }
}

impl<I> fmt::Debug for Consumed<I> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Consume").field("state", &"..").finish()
    }
}

impl<I> Iterator for &Consumed<I>
where
    I: Iterator,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.state.take().and_then(|mut iter| {
            let item = iter.next();
            self.state.set(item.is_some().then_some(iter));
            item
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let state = self.state.take();
        let hint = state
            .as_ref()
            .map(Iterator::size_hint)
            .unwrap_or((0, Some(0)));

        self.state.set(state);
        hint
    }
}

impl<I> ExactSizeIterator for &Consumed<I> where I: ExactSizeIterator {}

impl<I> FusedIterator for &Consumed<I> where I: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joined_array() {
        let s = [1, 2, 3].joined(", ").to_string();
        assert_eq!(s, "1, 2, 3");
    }

    #[test]
    fn joined_slice() {
        let s = [1, 2, 3][..].joined(", ").to_string();
        assert_eq!(s, "1, 2, 3");
    }

    #[test]
    fn joined_vec() {
        let s = vec![1, 2, 3].joined(", ").to_string();
        assert_eq!(s, "1, 2, 3");
    }

    #[test]
    fn consume_range() {
        let range = &(1..4).consumed();
        assert_eq!(range.len(), 3);

        let s = range.joined(", ").to_string();
        assert_eq!(s, "1, 2, 3");

        // on second `to_string` call the range iterator is exhausted
        assert_eq!(range.len(), 0);

        let s = range.joined(", ").to_string();
        assert!(s.is_empty());
    }

    #[test]
    fn always_fused() {
        use core::iter;

        let mut non_fused_iter = {
            let items = [None, Some(1), Some(2), None, Some(3)];
            let mut count = 0;
            iter::from_fn(move || {
                let item = items[count];
                count += 1;
                item
            })
        };

        // check the iterator isn't fused
        assert_eq!(non_fused_iter.next(), None);
        assert_eq!(non_fused_iter.next(), Some(1));

        // now it's fused
        let mut consumed = &non_fused_iter.consumed();
        assert_eq!(consumed.next(), Some(2));
        assert_eq!(consumed.next(), None);
        assert_eq!(consumed.next(), None);
    }
}

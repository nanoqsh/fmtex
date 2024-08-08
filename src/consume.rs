use core::{cell::Cell, fmt, iter::FusedIterator};

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
    use crate::ext::{IntoIteratorByRefExt, IntoIteratorExt};

    #[test]
    fn consume_range() {
        let range = &(1..4).consumed();
        assert_eq!(range.len(), 3);

        let s = range.joined(", ").to_string();
        assert_eq!(s, "1, 2, 3");

        // on second `to_string` call the range iterator is drained
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

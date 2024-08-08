use core::{cell::Cell, fmt};

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
}

impl<I> fmt::Debug for Consumed<I> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Consume").field("state", &"..").finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::ext::{IntoIteratorByRefExt, IntoIteratorExt};

    #[test]
    fn consume_range() {
        let range = (1..4).consumed();
        let s = range.joined(", ").to_string();
        assert_eq!(s, "1, 2, 3");

        // on second `to_string` call the range iterator is consumed
        let s = range.joined(", ").to_string();
        assert!(s.is_empty());
    }
}

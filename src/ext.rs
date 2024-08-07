use {
    crate::{consume::Consume, joined::Joined},
    core::fmt,
};

pub trait FormatRefIterator {
    #[inline]
    fn joined<S>(&self, sepr: S) -> Joined<&Self, S> {
        Joined { iter: self, sepr }
    }
}

impl<I> FormatRefIterator for I
where
    I: ?Sized,
    for<'it> &'it I: IntoIterator<Item: fmt::Display>,
{
}

pub trait IntoIteratorExt: IntoIterator {
    #[inline]
    fn consume(self) -> Consume<Self::IntoIter>
    where
        Self: Sized,
    {
        Consume::new(self.into_iter())
    }
}

impl<I> IntoIteratorExt for I where I: IntoIterator {}

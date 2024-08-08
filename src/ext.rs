use {
    crate::{consume::Consumed, joined::Joined},
    core::fmt,
};

pub trait IntoIteratorByRefExt {
    #[inline]
    fn joined<S>(&self, sepr: S) -> Joined<&Self, S> {
        Joined { iter: self, sepr }
    }
}

impl<I> IntoIteratorByRefExt for I
where
    I: ?Sized,
    for<'it> &'it I: IntoIterator<Item: fmt::Display>,
{
}

pub trait IntoIteratorExt: IntoIterator {
    #[inline]
    fn consumed(self) -> Consumed<Self::IntoIter>
    where
        Self: Sized,
    {
        Consumed::new(self.into_iter())
    }
}

impl<I> IntoIteratorExt for I where I: IntoIterator {}

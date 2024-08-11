use core::fmt;

/// Display implementer for the
/// [`joined`](crate::IntoIteratorByRefExt::joined) method.
#[derive(Clone, Copy, Debug)]
pub struct Joined<I, S> {
    pub(crate) iter: I,
    pub(crate) sepr: S,
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

#[cfg(test)]
mod tests {
    use crate::ext::IntoIteratorByRefExt;

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
}

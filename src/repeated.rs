use core::fmt;

/// Display implementer for the
/// [`repeated`](crate::DisplayExt::repeated) method.
#[derive(Clone, Copy, Debug)]
pub struct Repeated<'ds, D>
where
    D: ?Sized,
{
    pub(crate) display: &'ds D,
    pub(crate) n: usize,
}

impl<D> fmt::Display for Repeated<'_, D>
where
    D: fmt::Display + ?Sized,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = &self.display;
        for _ in 0..self.n {
            write!(f, "{display}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ext::DisplayExt;

    #[test]
    fn repeated_empty() {
        let s = "a".repeated(0).to_string();
        assert_eq!(s, "");
    }

    #[test]
    fn repeated_once() {
        let s = "a".repeated(1).to_string();
        assert_eq!(s, "a");
    }

    #[test]
    fn repeated_many() {
        let s = "a".repeated(5).to_string();
        assert_eq!(s, "aaaaa");
    }
}

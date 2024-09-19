use core::fmt;

/// Extension trait for [`Display`](core::fmt::Display).
pub trait DisplayExt: fmt::Display {
    /// Prints the value multiple times.
    ///
    /// # Examples
    /// ```
    /// use fmtex::prelude::*;
    ///
    /// let s = "*".repeated(3).to_string();
    /// assert_eq!(s, "***");
    /// ```
    #[inline]
    fn repeated(&self, n: usize) -> Repeated<'_, Self> {
        Repeated { display: self, n }
    }
}

impl<T> DisplayExt for T where T: fmt::Display {}

/// Display implementer for the
/// [`repeated`](DisplayExt::repeated) method.
#[derive(Clone, Copy, Debug)]
pub struct Repeated<'ds, D>
where
    D: ?Sized,
{
    display: &'ds D,
    n: usize,
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
    use super::*;

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

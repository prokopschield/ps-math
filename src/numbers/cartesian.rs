use crate::Number;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cartesian<R, I>
where
    R: Number,
    I: Number,
{
    pub real: R,
    pub imag: I,
}

impl<R: Number, I: Number> Number for Cartesian<R, I> {
    #[inline]
    fn real(&self) -> f64 {
        self.real.real()
    }

    #[inline]
    fn imag(&self) -> f64 {
        self.imag.real()
    }
}

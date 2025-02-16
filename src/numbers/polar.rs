use crate::Number;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Polar<M = f64, P = M>
where
    M: Number,
    P: Number,
{
    pub magnitude: M,
    pub phase: P,
}

impl<M: Number, P: Number> Number for Polar<M, P> {
    #[inline]
    fn magnitude(&self) -> f64 {
        self.magnitude.real()
    }

    #[inline]
    fn phase(&self) -> f64 {
        self.phase.real()
    }
}

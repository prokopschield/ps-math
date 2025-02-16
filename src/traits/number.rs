pub trait Number {
    /// Returns the real part of this number.
    fn real(&self) -> f64 {
        self.magnitude() * self.phase().cos()
    }

    /// Returns the imaginary part of this number.
    fn imag(&self) -> f64 {
        self.magnitude() * self.phase().sin()
    }

    /// Computes the magnitude (absolute value) of this number.
    #[inline]
    fn magnitude(&self) -> f64 {
        f64::sqrt(self.real().powi(2) + self.imag().powi(2))
    }

    /// Computes the phase (angle) of the number in radians.
    #[inline]
    fn phase(&self) -> f64 {
        f64::atan2(self.imag(), self.real())
    }
}

macro_rules! impl_number_for {
    ($($t:ty),*) => {
        $(
            impl Number for $t {
                #[inline(always)]
                fn real(&self) -> f64 {
                    *self as f64
                }

                #[inline(always)]
                fn imag(&self) -> f64 {
                    0f64
                }
            }
        )*
    };
}

impl_number_for!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, usize, isize);

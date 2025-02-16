use crate::{Cartesian, Polar};

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

    /// Computes the sum of two numbers.
    #[inline]
    fn add(&self, rhs: impl Number) -> Cartesian<f64, f64> {
        Cartesian {
            real: self.real() + rhs.real(),
            imag: self.imag() + rhs.imag(),
        }
    }

    /// Computes the difference of two numbers.
    #[inline]
    fn sub(&self, rhs: impl Number) -> Cartesian<f64, f64> {
        Cartesian {
            real: self.real() - rhs.real(),
            imag: self.imag() - rhs.imag(),
        }
    }

    /// Computes the product of two numbers in cartesian form.
    #[inline]
    fn cartesian_mul(&self, rhs: impl Number) -> Cartesian<f64, f64> {
        Cartesian {
            real: self.real() * rhs.real() - self.imag() * rhs.imag(),
            imag: self.real() * rhs.imag() + self.imag() * rhs.real(),
        }
    }

    /// Computes the product of two numbers in polar form.
    #[inline]
    fn polar_mul(&self, rhs: impl Number) -> Polar<f64, f64> {
        Polar {
            magnitude: self.magnitude() * rhs.magnitude(),
            phase: self.phase() + rhs.phase(),
        }
    }

    /// Computes the product of two numbers.
    #[inline]
    fn mul(&self, rhs: impl Number) -> Polar<f64, f64> {
        self.polar_mul(rhs)
    }

    /// Computes the quotient of two numbers.
    #[inline]
    fn div(&self, rhs: impl Number) -> Polar<f64, f64> {
        Polar {
            magnitude: self.magnitude() / rhs.magnitude(),
            phase: self.phase() - rhs.phase(),
        }
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

impl<T: Number> Number for &T {
    /// Returns the real part of this number.
    #[inline]
    fn real(&self) -> f64 {
        (**self).real()
    }

    /// Returns the imaginary part of this number.
    #[inline]
    fn imag(&self) -> f64 {
        (**self).imag()
    }

    /// Computes the magnitude (absolute value) of this number.
    #[inline]
    fn magnitude(&self) -> f64 {
        (**self).magnitude()
    }

    /// Computes the phase (angle) of the number in radians.
    #[inline]
    fn phase(&self) -> f64 {
        (**self).phase()
    }

    /// Computes the sum of two numbers.
    #[inline]
    fn add(&self, rhs: impl Number) -> Cartesian<f64, f64> {
        (**self).add(rhs)
    }

    /// Computes the difference of two numbers.
    #[inline]
    fn sub(&self, rhs: impl Number) -> Cartesian<f64, f64> {
        (**self).sub(rhs)
    }

    /// Computes the product of two numbers in cartesian form.
    #[inline]
    fn cartesian_mul(&self, rhs: impl Number) -> Cartesian<f64, f64> {
        (**self).cartesian_mul(rhs)
    }

    /// Computes the product of two numbers in polar form.
    #[inline]
    fn polar_mul(&self, rhs: impl Number) -> Polar<f64, f64> {
        (**self).polar_mul(rhs)
    }

    /// Computes the product of two numbers.
    #[inline]
    fn mul(&self, rhs: impl Number) -> Polar<f64, f64> {
        (**self).mul(rhs)
    }

    /// Computes the quotient of two numbers.
    #[inline]
    fn div(&self, rhs: impl Number) -> Polar<f64, f64> {
        (**self).div(rhs)
    }
}

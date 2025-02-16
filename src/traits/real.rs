use std::f64::consts::PI;

use crate::Cartesian;

use super::Number;

pub trait Real {
    /// Returns the real part of this number.
    fn real(&self) -> f64;

    /// Returns the imaginary part of this number.
    #[inline]
    fn imag(&self) -> f64 {
        0f64
    }

    /// Computes the magnitude (absolute value) of this number.
    #[inline]
    fn magnitude(&self) -> f64 {
        self.real().abs()
    }

    /// Computes the phase (angle) of the number in radians.
    #[inline]
    fn phase(&self) -> f64 {
        if self.real().is_sign_negative() {
            PI
        } else {
            0f64
        }
    }

    /// Computes the sum of two real numbers.
    #[inline]
    fn add_real(&self, rhs: impl Real) -> f64 {
        self.real() + rhs.real()
    }

    /// Computes the difference of two real numbers.
    #[inline]
    fn sub_real(&self, rhs: impl Real) -> f64 {
        self.real() - rhs.real()
    }

    /// Computes the product of a real number and a complex number in cartesian form.
    #[inline]
    fn cartesian_mul(&self, rhs: impl Number) -> Cartesian {
        Cartesian {
            real: self.real() * rhs.real(),
            imag: self.real() * rhs.imag(),
        }
    }

    /// Computes the product of two real numbers.
    #[inline]
    fn mul_real(&self, rhs: impl Real) -> f64 {
        self.real() * rhs.real()
    }

    /// Computes the quotient of two real numbers.
    #[inline]
    fn div_real(&self, rhs: impl Real) -> f64 {
        self.real() / rhs.real()
    }
}

macro_rules! impl_number_for {
    ($($t:ty),*) => {
        $(
            impl Real for $t {
                #[inline(always)]
                fn real(&self) -> f64 {
                    *self as f64
                }
            }
        )*
    };
}

impl_number_for!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, usize, isize);

impl<T: Real> Real for &T {
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

    /// Computes the sum of two real numbers.
    #[inline]
    fn add_real(&self, rhs: impl Real) -> f64 {
        (**self).add_real(rhs)
    }

    /// Computes the difference of two real numbers.
    #[inline]
    fn sub_real(&self, rhs: impl Real) -> f64 {
        (**self).sub_real(rhs)
    }

    /// Computes the product of two numbers in cartesian form.
    #[inline]
    fn cartesian_mul(&self, rhs: impl Number) -> Cartesian {
        (**self).cartesian_mul(rhs)
    }

    /// Computes the product of two real numbers.
    #[inline]
    fn mul_real(&self, rhs: impl Real) -> f64 {
        (**self).mul_real(rhs)
    }

    /// Computes the quotient of two real numbers.
    #[inline]
    fn div_real(&self, rhs: impl Real) -> f64 {
        (**self).div_real(rhs)
    }
}

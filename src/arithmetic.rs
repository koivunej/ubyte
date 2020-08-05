use core::cmp::Ordering;
use core::ops::{Mul, Add, Shl, Sub, Div, Shr, Rem};

use crate::ByteUnit;

impl<T: Into<ByteUnit>> Mul<T> for ByteUnit {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: T) -> Self::Output {
        ByteUnit(self.0.saturating_mul(rhs.into().0))
    }
}

impl<T: Into<ByteUnit>> Div<T> for ByteUnit {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: T) -> Self::Output {
        let value = rhs.into().0;
        if value == 0 {
            ByteUnit::max_value()
        } else {
            ByteUnit(self.0 / value)
        }
    }
}

impl<T: Into<ByteUnit>> Rem<T> for ByteUnit {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: T) -> Self::Output {
        ByteUnit(self.0 % rhs.into().0)
    }
}

impl<T: Into<ByteUnit>> Add<T> for ByteUnit {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: T) -> Self::Output {
        ByteUnit(self.0.saturating_add(rhs.into().0))
    }
}

impl<T: Into<ByteUnit>> Sub<T> for ByteUnit {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: T) -> Self::Output {
        ByteUnit(self.0.saturating_sub(rhs.into().0))
    }
}

impl<T: Into<ByteUnit>> Shl<T> for ByteUnit {
    type Output = Self;
    fn shl(self, rhs: T) -> Self::Output {
        let wanted = rhs.into().0;
        let available = self.0.leading_zeros() as u64;
        if wanted > available {
            ByteUnit::max_value()
        } else {
            ByteUnit(self.0 << wanted)
        }
    }
}

impl<T: Into<ByteUnit>> Shr<T> for ByteUnit {
    type Output = Self;
    fn shr(self, rhs: T) -> Self::Output {
        ByteUnit(self.0 >> rhs.into().0)
    }
}

impl<T: Into<ByteUnit> + Copy> PartialEq<T> for ByteUnit {
    fn eq(&self, other: &T) -> bool {
        self.0 == (*other).into().0
    }
}

impl<T: Into<ByteUnit> + Copy> PartialOrd<T> for ByteUnit {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.0.partial_cmp(&(*other).into().0)
    }
}

macro_rules! impl_arith_ops_on_core {
    ($T:ty) => (
        impl Mul<ByteUnit> for $T {
            type Output = ByteUnit;

            #[inline(always)]
            fn mul(self, rhs: ByteUnit) -> Self::Output {
                rhs * self
            }
        }

        impl Div<ByteUnit> for $T {
            type Output = ByteUnit;

            #[inline(always)]
            fn div(self, rhs: ByteUnit) -> Self::Output {
                rhs / self
            }
        }

        impl Rem<ByteUnit> for $T {
            type Output = ByteUnit;

            #[inline(always)]
            fn rem(self, rhs: ByteUnit) -> Self::Output {
                rhs % self
            }
        }

        impl Add<ByteUnit> for $T {
            type Output = ByteUnit;

            #[inline(always)]
            fn add(self, rhs: ByteUnit) -> Self::Output {
                rhs + self
            }
        }

        impl Sub<ByteUnit> for $T {
            type Output = ByteUnit;

            #[inline(always)]
            fn sub(self, rhs: ByteUnit) -> Self::Output {
                rhs - self
            }
        }

        impl Shr<ByteUnit> for $T {
            type Output = ByteUnit;

            #[inline(always)]
            fn shr(self, rhs: ByteUnit) -> Self::Output {
                rhs >> self
            }
        }

        impl Shl<ByteUnit> for $T {
            type Output = ByteUnit;

            #[inline(always)]
            fn shl(self, rhs: ByteUnit) -> Self::Output {
                rhs << self
            }
        }

        impl PartialEq<ByteUnit> for $T {
            #[inline(always)]
            fn eq(&self, other: &ByteUnit) -> bool {
                other == self
            }
        }

        impl PartialOrd<ByteUnit> for $T {
            #[inline(always)]
            fn partial_cmp(&self, other: &ByteUnit) -> Option<Ordering> {
                other.partial_cmp(self)
            }
        }
    )
}

impl_arith_ops_on_core!(usize);
impl_arith_ops_on_core!(u8);
impl_arith_ops_on_core!(u16);
impl_arith_ops_on_core!(u32);
impl_arith_ops_on_core!(u64);
impl_arith_ops_on_core!(u128);

impl_arith_ops_on_core!(isize);
impl_arith_ops_on_core!(i8);
impl_arith_ops_on_core!(i16);
impl_arith_ops_on_core!(i32);
impl_arith_ops_on_core!(i64);
impl_arith_ops_on_core!(i128);

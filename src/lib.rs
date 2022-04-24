use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};
use std::{fmt, ops::*, str::FromStr};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Clone, Copy)]
#[serde(transparent)]
pub struct NormDecimal(Decimal);

impl FromStr for NormDecimal {
    type Err = <Decimal as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Decimal::from_str(s).map(Into::into)
    }
}

impl From<Decimal> for NormDecimal {
    fn from(value: Decimal) -> Self {
        Self(value.normalize())
    }
}

impl<'a> From<&'a Decimal> for NormDecimal {
    fn from(value: &'a Decimal) -> Self {
        Self(value.normalize())
    }
}

impl Deref for NormDecimal {
    type Target = Decimal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for NormDecimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'de> Deserialize<'de> for NormDecimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        <Decimal as Deserialize>::deserialize(deserializer).map(Into::into)
    }
}

macro_rules! forward_from_impl {
    ($($typ:ident),+) => {
        $(impl From<$typ> for NormDecimal {
            fn from(value: $typ) -> Self {
                Self::from(Decimal::from(value))
            }
        })+
    }
}

forward_from_impl!(u8, i8, u16, i16, u32, i32, u64, i64);

impl<T> Add<T> for NormDecimal
where
    T: Into<NormDecimal>,
{
    type Output = NormDecimal;

    fn add(self, rhs: T) -> Self::Output {
        Self::from(self.0 + rhs.into().0)
    }
}

impl<T> Sub<T> for NormDecimal
where
    T: Into<NormDecimal>,
{
    type Output = NormDecimal;

    fn sub(self, rhs: T) -> Self::Output {
        Self::from(self.0 - rhs.into().0)
    }
}

impl<T> Div<T> for NormDecimal
where
    T: Into<NormDecimal>,
{
    type Output = NormDecimal;

    fn div(self, rhs: T) -> Self::Output {
        Self::from(self.0 / rhs.into().0)
    }
}

impl<T> Rem<T> for NormDecimal
where
    T: Into<NormDecimal>,
{
    type Output = NormDecimal;

    fn rem(self, rhs: T) -> Self::Output {
        Self::from(self.0 % rhs.into().0)
    }
}

impl<T> Mul<T> for NormDecimal
where
    T: Into<NormDecimal>,
{
    type Output = NormDecimal;

    fn mul(self, rhs: T) -> Self::Output {
        Self::from(self.0 * rhs.into().0)
    }
}

impl Neg for NormDecimal {
    type Output = NormDecimal;

    fn neg(self) -> Self::Output {
        Self::from(-self.0)
    }
}

impl<T> AddAssign<T> for NormDecimal
where
    T: Into<NormDecimal>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T> SubAssign<T> for NormDecimal
where
    T: Into<NormDecimal>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl<T> MulAssign<T> for NormDecimal
where
    T: Into<NormDecimal>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T> DivAssign<T> for NormDecimal
where
    T: Into<NormDecimal>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<T> RemAssign<T> for NormDecimal
where
    T: Into<NormDecimal>,
{
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs;
    }
}

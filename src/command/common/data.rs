use core::{cmp::Ord, fmt::Debug};

use crate::error::{Error, Result};
use atat::{atat_derive::AtatEnum, AtatLen};
use serde::{Deserialize, Serialize};

pub type Integer = u32;

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct BoundedInteger<T, const MIN: usize, const MAX: usize>(T);

impl<T, const MIN: usize, const MAX: usize> BoundedInteger<T, MIN, MAX>
where
    T: TryFrom<usize> + Ord,
{
    pub const RANGE_LEN: usize = (MAX - MIN) + 1;

    pub fn new(value: T) -> Result<Self>
    where
        <T as TryFrom<usize>>::Error: Debug,
    {
        if value < MIN.try_into().unwrap() || value > MAX.try_into().unwrap() {
            return Err(Error::IntegerRange);
        }

        Ok(Self(value))
    }
}

impl<T, const MIN: usize, const MAX: usize> AtatLen for BoundedInteger<T, MIN, MAX> {
    // this is a lower upper bound than that provided by the implementation for u32
    const LEN: usize = MAX.ilog10() as usize + 1;
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Enabled {
    Disable = 0,
    Enable = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_integer_restrict() {
        assert_eq!(BoundedInteger::<u8, 2, 8>::new(1), Err(Error::IntegerRange));
        assert_eq!(BoundedInteger::<u8, 2, 8>::new(2), Ok(BoundedInteger(2)));
        assert_eq!(BoundedInteger::<u8, 2, 8>::new(5), Ok(BoundedInteger(5)));
        assert_eq!(BoundedInteger::<u8, 2, 8>::new(8), Ok(BoundedInteger(8)));
        assert_eq!(BoundedInteger::<u8, 2, 8>::new(9), Err(Error::IntegerRange));
    }

    #[test]
    fn bounded_integer_len() {
        type Short = BoundedInteger<u8, 0, 1>;
        type Medium = BoundedInteger<u8, 2, 8>;
        type Long = BoundedInteger<usize, { usize::MIN }, { usize::MAX - 1 }>;
        assert_eq!(Short::RANGE_LEN, 2);
        assert_eq!(Medium::RANGE_LEN, 7);
        assert_eq!(Long::RANGE_LEN, usize::MAX);
    }

    #[test]
    fn bounded_integer_atatlen() {
        type Lower1 = BoundedInteger<u16, 0, 1>;
        type Upper1 = BoundedInteger<u16, 0, 9>;
        type Lower2 = BoundedInteger<u16, 0, 10>;
        type Upper2 = BoundedInteger<u16, 0, 99>;
        type Lower3 = BoundedInteger<u16, 0, 100>;
        type Upper3 = BoundedInteger<u16, 0, 999>;
        assert_eq!(Lower1::LEN, 1);
        assert_eq!(Upper1::LEN, 1);
        assert_eq!(Lower2::LEN, 2);
        assert_eq!(Upper2::LEN, 2);
        assert_eq!(Lower3::LEN, 3);
        assert_eq!(Upper3::LEN, 3);
    }
}

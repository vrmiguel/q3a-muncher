use num_traits::{CheckedAdd, CheckedSub, Num, One};

use crate::{Error, Result};

pub trait ExtraCheckedOps {
    fn checked_increment(&mut self) -> Result<()>;

    fn checked_decrement(&mut self) -> Result<()>;
}

impl<N: Num + CheckedAdd + CheckedSub + One + Copy>
    ExtraCheckedOps for N
{
    fn checked_increment(&mut self) -> Result<()> {
        let current = *self;
        let one = N::one();

        *self =
            current.checked_add(&one).ok_or(Error::Overflow)?;

        Ok(())
    }

    fn checked_decrement(&mut self) -> Result<()> {
        let current = *self;
        let one = N::one();

        *self =
            current.checked_sub(&one).ok_or(Error::Underflow)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::extra_checked_ops::ExtraCheckedOps;

    #[test]
    fn extra_checked_ops_works() {
        let mut zero = 0_u8;
        let mut limit = 255_u8;

        zero.checked_increment().unwrap();
        zero.checked_increment().unwrap();
        assert_eq!(zero, 2);
        zero.checked_decrement().unwrap();
        assert_eq!(zero, 1);

        assert!(limit.checked_increment().is_err());
    }
}

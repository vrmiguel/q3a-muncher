use std::marker::PhantomData;

use crate::{
    ensure, extra_checked_ops::ExtraCheckedOps, Result,
};

pub struct InstanceCounter<T: Into<u8>, const N: usize> {
    counter: [u8; N],
    marker: PhantomData<T>,
}

impl<T: Into<u8>, const N: usize> InstanceCounter<T, N> {
    pub const fn new() -> Self {
        Self {
            counter: [0; N],
            marker: PhantomData,
        }
    }

    pub fn add(&mut self, element: T) -> Result<()> {
        let position = Self::element_to_usize(element);

        ensure!(
            position < N,
            "Element would not fit in the counter buffer"
        );

        self.counter[position].checked_increment()?;

        Ok(())
    }

    pub fn get(&self, element: T) -> Option<u8> {
        let index = Self::element_to_usize(element);

        self.counter.get(index).copied()
    }

    #[inline(always)]
    fn element_to_usize(element: T) -> usize {
        let byte: u8 = element.into();
        byte as usize
    }
}

#[cfg(test)]
mod tests {
    use super::InstanceCounter;
    use crate::{CauseOfDeath, CAUSES_OF_DEATH};

    #[test]
    fn instance_counter_adds_correctly() {
        let mut counter: InstanceCounter<
            CauseOfDeath,
            CAUSES_OF_DEATH,
        > = InstanceCounter::new();

        counter.add(CauseOfDeath::Bfg).unwrap();
        counter.add(CauseOfDeath::Bfg).unwrap();

        assert_eq!(counter.get(CauseOfDeath::Bfg).unwrap(), 2);
        assert_eq!(
            counter.get(CauseOfDeath::Rocket).unwrap(),
            0
        );
    }
}

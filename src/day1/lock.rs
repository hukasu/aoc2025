use std::ops::{Add, Deref, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lock(i32);

impl Lock {
    pub fn add_with_turns(self, rhs: i32) -> (Self, u32) {
        let after = self.0 + rhs;
        let Ok(q) = u32::try_from(after / 100) else {
            unreachable!("Will never be negative and will never overflow.");
        };
        let m = after % 100;
        (Self(m), q)
    }

    pub fn sub_with_turns(self, rhs: i32) -> (Self, u32) {
        let Ok(full_turns) = u32::try_from(rhs / 100) else {
            unreachable!("Will never be negative and will never overflow.");
        };
        let remaining_moves = rhs % 100;
        let after = self.0 - remaining_moves;
        let q = if self.0 != 0 && after <= 0 { 1 } else { 0 };
        let m = after.rem_euclid(100);
        (Self(m), full_turns + q)
    }
}

impl Default for Lock {
    fn default() -> Self {
        Lock(50)
    }
}

impl Add<i32> for Lock {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self((self.0 + rhs) % 100)
    }
}

impl Sub<i32> for Lock {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self((self.0 - rhs).rem_euclid(100))
    }
}

impl Deref for Lock {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lock() {
        assert_eq!(Lock(11) + 8, Lock(19));
        assert_eq!(Lock(19) - 19, Lock(0));
        assert_eq!(Lock(0) - 1, Lock(99));
        assert_eq!(Lock(99) + 1, Lock(0));
        assert_eq!(Lock(5) - 10, Lock(95));
        assert_eq!(Lock(95) + 5, Lock(0));
        assert_eq!(Lock(50) - 68, Lock(82));
        assert_eq!(Lock(82) - 30, Lock(52));
        assert_eq!(Lock(52) + 48, Lock(0));
        assert_eq!(Lock(0) - 5, Lock(95));
        assert_eq!(Lock(95) + 60, Lock(55));
        assert_eq!(Lock(55) - 55, Lock(0));
        assert_eq!(Lock(0) - 1, Lock(99));
        assert_eq!(Lock(99) - 99, Lock(0));
        assert_eq!(Lock(0) + 14, Lock(14));
        assert_eq!(Lock(14) - 82, Lock(32));

        assert_eq!(Lock(50).sub_with_turns(68), (Lock(82), 1));
        assert_eq!(Lock(95).add_with_turns(60), (Lock(55), 1));
        assert_eq!(Lock(14).sub_with_turns(82), (Lock(32), 1));

        assert_eq!(Lock(50).add_with_turns(150), (Lock(0), 2));
        assert_eq!(Lock(50).sub_with_turns(150), (Lock(0), 2));
    }
}

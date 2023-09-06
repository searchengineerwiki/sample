use core::convert::From;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Sub, SubAssign};

// Simple bitset, limited to values between 0 and 127
struct Bitset {
    data: u128,
}

impl Bitset {
    // Creates a new Bitset
    fn new() -> Self {
        Self { data: 0 }
    }

    // Resets the bitmap back to an empty state
    fn clear(&mut self) {
        self.data = 0;
    }

    // Returns `true` if this set contains the specified value
    fn contains(&self, value: u128) -> bool {
        let mask = 1 << value;
        self.data & mask != 0
    }

    // Adds the value to the set
    // Returns whether the value was absent from the set.
    fn insert(&mut self, value: u128) -> Result<bool, ()> {
        if value > 127 {
            return Err(());
        }

        let mask = 1 << value;
        let exists = self.data & mask != 0;
        self.data |= mask;
        Ok(exists)
    }

    // Returns `true` if the bitset is empty, otherwise `false`
    fn is_empty(&self) -> bool {
        self.data == 0
    }
}

impl From<u128> for Bitset {
    fn from(value: u128) -> Self {
        Self { data: value }
    }
}

impl BitAnd for Bitset {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let raw = self.data & rhs.data;
        raw.into()
    }
}

impl BitAndAssign for Bitset {
    fn bitand_assign(&mut self, rhs: Self) {
        let raw = self.data & rhs.data;
        *self = raw.into();
    }
}

impl BitOr for Bitset {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let raw = self.data | rhs.data;
        raw.into()
    }
}

impl BitOrAssign for Bitset {
    fn bitor_assign(&mut self, rhs: Self) {
        let raw = self.data | rhs.data;
        *self = raw.into()
    }
}

impl Sub for Bitset {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let raw = self.data & !rhs.data;
        raw.into()
    }
}

impl SubAssign for Bitset {
    fn sub_assign(&mut self, rhs: Self) {
        let raw = self.data & !rhs.data;
        *self = raw.into()
    }
}

impl Not for Bitset {
    type Output = Self;

    fn not(self) -> Self::Output {
        let raw = !self.data;
        raw.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SET_A: Bitset = Bitset { data: 30 }; // Binary 11110, 1, 2, 3, and 4 are set
    const SET_B: Bitset = Bitset { data: 6 }; //  Binary 00110, 1 and 2 are set

    #[test]
    fn test_clear() {
        let mut bitset = Bitset::new();
        let _ = bitset.insert(100);
        assert_eq!(true, bitset.contains(100));
        bitset.clear();
        assert_eq!(false, bitset.contains(100));
    }

    #[test]
    fn test_from_u128() {
        let raw: u128 = 10; // 1010 in binary
        let bitset: Bitset = raw.into();

        assert!(bitset.contains(1));
        assert!(bitset.contains(3));
    }

    #[test]
    fn test_insert_and_contains() -> Result<(), ()> {
        // Initial bitset should not contains any values
        let mut bitset = Bitset::new();
        assert_eq!(false, bitset.contains(42));

        // Add a value, noting that the set didn't already contain the value
        // and verify the set contains the item
        assert_eq!(false, bitset.insert(42)?);
        assert_eq!(true, bitset.contains(42));

        // Add a value, noting that the set did already contain the value
        // and verify the set still contains the item
        assert_eq!(true, bitset.insert(42)?);
        assert_eq!(true, bitset.contains(42));

        Ok(())
    }

    #[test]
    fn test_insert_large_value_fails() {
        let mut bitset = Bitset::new();
        let result = bitset.insert(128);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_empty() {
        let mut bitset = Bitset::new();
        assert!(bitset.is_empty());
        let _ = bitset.insert(10);
        assert_eq!(false, bitset.is_empty());
    }

    #[test]
    fn test_set_difference() {
        let result = SET_A - SET_B;
        assert!(!result.contains(1));
        assert!(!result.contains(2));
        assert!(result.contains(3));
        assert!(result.contains(4));
    }

    #[test]
    fn test_set_difference_assign() {
        // Need a mutable set b to do assignment
        let mut set_a = Bitset::new();
        let _ = set_a.insert(1);
        let _ = set_a.insert(2);
        let _ = set_a.insert(3);
        let _ = set_a.insert(4);

        set_a -= SET_B;
        assert!(!set_a.contains(1));
        assert!(!set_a.contains(2));
        assert!(set_a.contains(3));
        assert!(set_a.contains(4));
    }

    #[test]
    fn test_set_intersection() {
        let result = SET_A & SET_B;
        assert!(result.contains(1));
        assert!(result.contains(2));
        assert!(!result.contains(3));
        assert!(!result.contains(4));
    }

    #[test]
    fn test_set_intersection_assign() {
        // Need a mutable set b to do assignment
        let mut set_a = Bitset::new();
        let _ = set_a.insert(1);
        let _ = set_a.insert(2);
        let _ = set_a.insert(3);
        let _ = set_a.insert(4);

        set_a &= SET_B;
        assert!(set_a.contains(1));
        assert!(set_a.contains(2));
        assert!(!set_a.contains(3));
        assert!(!set_a.contains(4));
    }

    #[test]
    fn test_set_negation() {
        let result = !SET_A;

        assert!(!result.contains(1));
        assert!(!result.contains(2));
        assert!(!result.contains(3));
        assert!(!result.contains(4));

        assert!(result.contains(5));
        assert!(result.contains(28));
        assert!(result.contains(56));
        assert!(result.contains(100));
    }

    #[test]
    fn test_set_union() {
        let result = SET_A | SET_B;
        assert!(result.contains(1));
        assert!(result.contains(2));
        assert!(result.contains(3));
        assert!(result.contains(4));
    }

    #[test]
    fn test_set_union_assign() {
        let mut set_a = Bitset::new();
        let _ = set_a.insert(1);
        let _ = set_a.insert(2);
        let _ = set_a.insert(3);
        let _ = set_a.insert(4);

        set_a |= SET_B;
        assert!(set_a.contains(1));
        assert!(set_a.contains(2));
        assert!(set_a.contains(3));
        assert!(set_a.contains(4));
    }
}

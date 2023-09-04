// Simple bitset, limited to values between 0 and 127
struct Bitset {
    data: u128,
}

impl Bitset {
    // Creates a new Bitset
    fn new() -> Self {
        Self { data: 0 }
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

    fn clear(&mut self) {
        self.data = 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_clear() {
        let mut bitset = Bitset::new();
        let _ = bitset.insert(100);
        assert_eq!(true, bitset.contains(100));
        bitset.clear();
        assert_eq!(false, bitset.contains(100));
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
}

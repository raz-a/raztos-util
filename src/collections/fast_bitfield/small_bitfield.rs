use super::{find_highest_set_bit, find_lowest_set_bit, FastBitField};

/// Defines the maximum number of bits in a small bitfield.
const SMALL_BIT_FIELD_BIT_SIZE: usize = core::mem::size_of::<usize>() * 8;

/// Defines the structure and fast_bitfield interface for Small Bitfieds.
/// A Small Bitfield is a wrapper type that holds a `usize` bitfield.
pub struct SmallBitField {
    /// Holds the bitfield state.
    bitfield: usize,
}

/// Defines functionality unique to SmallBitField.
impl SmallBitField {
    /// Sets bits in the bit field.
    ///
    /// # Arguments
    /// field - Provides the bits to be set.
    pub fn set_field(&mut self, field: usize) {
        self.bitfield |= field;
    }

    /// Clears bits in the bit field.
    ///
    /// # Arguments
    /// field - Provides the bits to be cleared.
    pub fn clear_field(&mut self, field: usize) {
        self.bitfield &= !field;
    }
}

/// Defines the FastBitField interface for SmallBitField.
impl FastBitField for SmallBitField {
    /// Creates a new, empty SmallBitField
    ///
    /// # Returns
    /// A SmallBitField.
    fn new() -> Self {
        SmallBitField { bitfield: 0 }
    }

    /// Gets the number of bits available in the bitfield type.
    ///
    /// # Returns
    /// The number of bits available.
    ///
    /// # Examples
    /// ```
    /// use raztos_util::collections::fast_bitfield::{FastBitField, SmallBitField};
    ///
    /// assert_eq!(SmallBitField::get_number_of_bits(), core::mem::size_of::<usize>() * 8);
    /// ```
    fn get_number_of_bits() -> usize {
        SMALL_BIT_FIELD_BIT_SIZE
    }

    /// Sets a bit in the bit field
    ///
    /// # Arguments
    /// index - Provides the bit to set.
    fn set_bit(&mut self, index: usize) {
        if index < SMALL_BIT_FIELD_BIT_SIZE {
            self.bitfield |= 1 << index;
        }
    }

    /// Clears a bit in the bit field
    ///
    /// # Arguments
    /// index - Provides the bit to clear.
    fn clear_bit(&mut self, index: usize) {
        if index < SMALL_BIT_FIELD_BIT_SIZE {
            self.bitfield &= !(1 << index);
        }
    }

    /// Gets the lowest set bit.
    ///
    /// # Returns
    /// The lowest set bit index or `None` if no bits are set.
    ///
    /// # Examples
    /// ```
    /// use raztos_util::collections::fast_bitfield::{FastBitField, SmallBitField};
    ///
    /// let mut small = SmallBitField::new();
    /// small.clear_field(core::usize::MAX);
    ///
    /// assert_eq!(small.get_lowest_set_bit(), None);
    ///
    /// small.set_bit(0);
    /// assert_eq!(small.get_lowest_set_bit(), Some(0));
    ///
    /// small.set_bit(1);
    /// assert_eq!(small.get_lowest_set_bit(), Some(0));
    /// ```
    fn get_lowest_set_bit(&self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        Some(self.get_lowest_set_bit_unchecked())
    }

    /// Gets the highest set bit.
    ///
    /// # Returns
    /// The highest set bit index or `None` if no bits are set.
    ///
    /// # Examples
    /// ```
    /// use raztos_util::collections::fast_bitfield::{FastBitField, SmallBitField};
    ///
    /// let mut small = SmallBitField::new();
    /// small.clear_field(core::usize::MAX);
    ///
    /// assert_eq!(small.get_highest_set_bit(), None);
    ///
    /// small.set_bit(0);
    /// assert_eq!(small.get_highest_set_bit(), Some(0));
    ///
    /// small.set_bit(1);
    /// assert_eq!(small.get_highest_set_bit(), Some(1));
    /// ```
    fn get_highest_set_bit(&self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        Some(self.get_highest_set_bit_unchecked())
    }

    /// Gets the value of a specific bit in the bit field.
    ///
    /// # Arguments
    /// index - Provides the bit to test.
    ///
    /// # Returns
    /// `Some(true)` if bit is set.
    /// `Some(false)` if bit is cleared.
    /// `None` if index is invalid.
    ///
    /// # Examples
    /// ```
    /// use raztos_util::collections::fast_bitfield::{FastBitField, SmallBitField};
    ///
    /// let mut small = SmallBitField::new();
    /// small.clear_field(core::usize::MAX);
    ///
    /// assert_eq!(small.test_bit(1000), None);
    /// assert_eq!(small.test_bit(5), Some(false));
    ///
    /// small.set_bit(5);
    /// assert_eq!(small.test_bit(5), Some(true));
    /// ```
    fn test_bit(&self, index: usize) -> Option<bool> {
        if index < SMALL_BIT_FIELD_BIT_SIZE {
            //
            // UNSAFE: The index check that makes the unsafe variant unsafe is performed before
            // calling it.
            //

            unsafe {
                return Some(self.test_bit_unchecked(index));
            }
        }

        None
    }

    /// Determines whether or not the bitfield is empty.
    ///
    /// # Returns
    /// `true` if empty, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use raztos_util::collections::fast_bitfield::{FastBitField, SmallBitField};
    ///
    /// let mut small = SmallBitField::new();
    /// small.clear_field(core::usize::MAX);
    /// assert!(small.is_empty());
    ///
    /// small.set_bit(0);
    /// assert!(!small.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        self.bitfield == 0
    }

    /// Gets the lowest set bit, guaranteed to have no branches and be in constant time, completely
    /// invariant of the state of the bit field. If no bits are set, the result is undefined.
    ///
    /// This function should only be used if the caller can guarantee the bitfield will always
    /// have at least one bit set.
    ///
    /// # Returns
    /// The lowest set bit index or `UNDEFINED` if no bits are set.
    ///
    /// # Examples
    /// ```
    /// use raztos_util::collections::fast_bitfield::{FastBitField, SmallBitField};
    ///
    /// let mut small = SmallBitField::new();
    /// small.clear_field(core::usize::MAX);
    ///
    /// small.set_bit(0);
    /// assert_eq!(small.get_lowest_set_bit_unchecked(), 0);
    ///
    /// small.set_bit(1);
    /// assert_eq!(small.get_lowest_set_bit_unchecked(), 0);
    /// ```
    fn get_lowest_set_bit_unchecked(&self) -> usize {
        find_lowest_set_bit(self.bitfield)
    }

    /// Gets the highest set bit, guaranteed to have no branches and be in constant time, completely
    /// invariant of the state of the bit field. If no bits are set, the result is undefined.
    ///
    /// This function should only be used if the caller can guarantee the bitfield will always
    /// have at least one bit set.
    ///
    /// # Returns
    /// The highest set bit index or `UNDEFINED` if no bits are set.
    ///
    /// # Examples
    /// ```
    /// use raztos_util::collections::fast_bitfield::{FastBitField, SmallBitField};
    ///
    /// let mut small = SmallBitField::new();
    /// small.clear_field(core::usize::MAX);
    ///
    /// small.set_bit(0);
    /// assert_eq!(small.get_highest_set_bit_unchecked(), 0);
    ///
    /// small.set_bit(1);
    /// assert_eq!(small.get_highest_set_bit_unchecked(), 1);
    /// ```
    fn get_highest_set_bit_unchecked(&self) -> usize {
        find_highest_set_bit(self.bitfield)
    }

    /// Sets a bit in the bit field.
    ///
    /// # Arguments
    /// index - Provides the bit to set.
    ///
    /// # Unsafe
    /// This unsafe variant does not check if the index is valid for the size of
    /// the bit field. The caller must guarantee that the index is less than `get_number_of_bits()`.
    unsafe fn set_bit_unchecked(&mut self, index: usize) {
        self.bitfield |= 1 << index;
    }

    /// Clears a bit in the bit field
    ///
    /// # Arguments
    /// index - Provides the bit to clear.
    ///
    /// # Unsafe
    /// This unsafe variant does not check if the index is valid for the size of
    /// the bit field. The caller must guarantee that the index is less than `get_number_of_bits()`.
    unsafe fn clear_bit_unchecked(&mut self, index: usize) {
        self.bitfield &= !(1 << index);
    }

    /// Gets the value of a specific bit in the bit field.
    ///
    /// # Arguments
    /// index - Provides the bit to test.
    ///
    /// # Returns
    /// `true` if bit is set.
    /// `false` if bit is cleared.
    ///
    /// # Unsafe
    /// This unsafe variant does not check if the index is valid for the size of
    /// the bit field. The caller must guarantee that the index is less than `get_number_of_bits()`.
    ///
    /// # Examples
    /// ```
    /// use raztos_util::collections::fast_bitfield::{FastBitField, SmallBitField};
    ///
    /// let mut small = SmallBitField::new();
    /// small.clear_field(core::usize::MAX);
    ///
    /// unsafe {
    ///     assert_eq!(small.test_bit_unchecked(7), false);
    ///
    ///     small.set_bit_unchecked(7);
    ///     assert_eq!(small.test_bit_unchecked(7), true);
    /// }
    /// ```
    unsafe fn test_bit_unchecked(&self, index: usize) -> bool {
        (self.bitfield & (1 << index)) != 0
    }
}

//
// Unit Tests
//

#[cfg(test)]
mod tests {
    use super::*;

    //
    // Constructor Test
    //

    #[test]
    fn create_defaults_to_empty() {
        let small = SmallBitField::new();
        assert_eq!(small.bitfield, 0);
        assert!(small.is_empty());
    }

    //
    // Trait Tests
    //

    #[test]
    fn number_of_bits() {
        assert_eq!(
            SmallBitField::get_number_of_bits(),
            SMALL_BIT_FIELD_BIT_SIZE
        );
    }

    #[test]
    fn validate_set_bit() {
        let mut small = SmallBitField::new();
        let mut small_unsafe = SmallBitField::new();
        let mut expected_value: usize = 0;

        for i in 0..SMALL_BIT_FIELD_BIT_SIZE {
            //
            // Out of bounds set should do nothing.
            //

            small.set_bit(SMALL_BIT_FIELD_BIT_SIZE);
            assert_eq!(small.bitfield, expected_value);

            expected_value |= 1 << i;
            small.set_bit(i);
            assert_eq!(small.bitfield, expected_value);

            //
            // Calling set for an already set bit should result in no change.
            //

            small.set_bit(i);
            assert_eq!(small.bitfield, expected_value);

            unsafe {
                small_unsafe.set_bit_unchecked(i);
                assert_eq!(small_unsafe.bitfield, expected_value);

                //
                // Calling set for an already set bit should result in no change.
                //

                small_unsafe.set_bit_unchecked(i);
                assert_eq!(small_unsafe.bitfield, expected_value);
            }
        }
    }

    #[test]
    fn validate_clear_bit() {
        let mut small = SmallBitField::new();
        let mut small_unsafe = SmallBitField::new();
        let mut expected_value = core::usize::MAX;

        small.bitfield = core::usize::MAX;
        small_unsafe.bitfield = core::usize::MAX;

        for i in 0..SMALL_BIT_FIELD_BIT_SIZE {
            //
            // Out of bounds clear should do nothing.
            //

            small.clear_bit(SMALL_BIT_FIELD_BIT_SIZE);
            assert_eq!(small.bitfield, expected_value);

            expected_value &= !(1 << i);
            small.clear_bit(i);
            assert_eq!(small.bitfield, expected_value);

            //
            // Calling clear for an already cleared bit should result in no change.
            //

            small.clear_bit(i);
            assert_eq!(small.bitfield, expected_value);

            unsafe {
                small_unsafe.clear_bit_unchecked(i);
                assert_eq!(small_unsafe.bitfield, expected_value);

                //
                // Calling clear for an already cleared bit should result in no change.
                //

                small_unsafe.clear_bit_unchecked(i);
                assert_eq!(small_unsafe.bitfield, expected_value);
            }
        }
    }

    #[test]
    fn validate_get_lowest_set_bit() {
        let mut small = SmallBitField::new();

        //
        // Empty should result in None for checked variant
        //

        assert_eq!(small.get_lowest_set_bit(), None);

        for i in 0..SMALL_BIT_FIELD_BIT_SIZE {
            small.set_bit(i);
            assert_eq!(small.get_lowest_set_bit(), Some(0));
            assert_eq!(small.get_lowest_set_bit_unchecked(), 0);
        }

        for i in 0..SMALL_BIT_FIELD_BIT_SIZE {
            assert_eq!(small.get_lowest_set_bit(), Some(i));
            assert_eq!(small.get_lowest_set_bit_unchecked(), i);
            small.clear_bit(i);
        }
    }

    #[test]
    fn validate_get_highest_set_bit() {
        let mut small = SmallBitField::new();

        //
        // Empty should result in None for checked variant
        //

        assert_eq!(small.get_highest_set_bit(), None);

        for i in 0..SMALL_BIT_FIELD_BIT_SIZE {
            small.set_bit(i);
            assert_eq!(small.get_highest_set_bit(), Some(i));
            assert_eq!(small.get_highest_set_bit_unchecked(), i);
        }

        for i in 0..SMALL_BIT_FIELD_BIT_SIZE {
            assert_eq!(
                small.get_highest_set_bit(),
                Some(SMALL_BIT_FIELD_BIT_SIZE - 1)
            );
            assert_eq!(
                small.get_highest_set_bit_unchecked(),
                SMALL_BIT_FIELD_BIT_SIZE - 1
            );
            small.clear_bit(i);
        }
    }

    #[test]
    fn validate_test_bit() {
        let mut small = SmallBitField::new();

        //
        // Out of bounds should return None for checked variant
        //

        assert_eq!(small.test_bit(SMALL_BIT_FIELD_BIT_SIZE), None);

        //
        // Set causes test to return true.
        //

        small.set_bit(0);
        assert_eq!(small.test_bit(0), Some(true));
        unsafe {
            assert_eq!(small.test_bit_unchecked(0), true);
        }

        //
        // Clear causes test to return false.
        //

        small.clear_bit(0);
        assert_eq!(small.test_bit(0), Some(false));
        unsafe {
            assert_eq!(small.test_bit_unchecked(0), false);
        }

        //
        // Changing another bit has no affect on the bit being tested.
        //

        small.set_bit(1);
        assert_eq!(small.test_bit(0), Some(false));
        unsafe {
            assert_eq!(small.test_bit_unchecked(0), false);
        }

        //
        // Clear causes test to return false.
        //

        small.set_bit(0);
        small.clear_bit(1);
        assert_eq!(small.test_bit(0), Some(true));
        unsafe {
            assert_eq!(small.test_bit_unchecked(0), true);
        }
    }

    //
    // Method Tests
    //

    #[test]
    fn validate_set_and_clear_field() {
        let mut small = SmallBitField::new();
        let mut expected_value: usize = 0;
        let zeros: usize = 0;
        let fives = (0x55555555_55555555 & core::usize::MAX) as usize;
        let a_s = (0xAAAAAAAA_AAAAAAAA & core::usize::MAX) as usize;
        let f_s = (0xFFFFFFFF_FFFFFFFF & core::usize::MAX) as usize;

        //
        // Calling set with 0 results in no change.
        //

        assert_eq!(small.bitfield, zeros);
        small.set_field(0);
        assert_eq!(small.bitfield, zeros);

        //
        // Setting only sets bits expected bits.
        //

        expected_value |= 1 << 1;
        small.set_bit(1);

        expected_value |= fives;
        small.set_field(fives);
        assert_eq!(small.bitfield, expected_value);

        //
        // Settings already set values should result in no change.
        //

        small.set_field(fives);
        assert_eq!(small.bitfield, expected_value);

        small.set_field(a_s);
        assert_eq!(small.bitfield, f_s);

        //
        // Clearing only clears expected bits.
        //

        small.clear_field(fives);
        assert_eq!(small.bitfield, a_s);

        //
        // Clearing already cleared values should result in no change.
        //

        small.clear_field(fives);
        assert_eq!(small.bitfield, a_s);

        //
        // Calling clear with 0 results in no change.
        //

        small.clear_field(0);
        assert_eq!(small.bitfield, a_s);
    }
}

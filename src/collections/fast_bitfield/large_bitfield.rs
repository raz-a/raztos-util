use super::{find_highest_set_bit, find_lowest_set_bit, FastBitField};

/// Defines the number of bitfield groups in a large bitfield
const LARGE_BIT_FIELD_GROUP_COUNT: usize = core::mem::size_of::<usize>() * 8;

/// Defines the maximum number of bits in a large bitfield.
const LARGE_BIT_FIELD_BIT_SIZE: usize = LARGE_BIT_FIELD_GROUP_COUNT * LARGE_BIT_FIELD_GROUP_COUNT;

/// Defines the structure and fast_bitfield interface for Large Bitfieds.
/// A Large Bitfield is a strcture that holds an array of `sizeof(usize) * 8` `usize` values as well
/// as a "layer_cache" `usize` field to quickly determine highest and lowest set bits.
pub struct LargeBitField {
    /// Holds a bitfield describing which sub bitfields currently have any set bits.
    layer_cache: usize,

    /// Holds the bitfield state.
    bitfield: [usize; LARGE_BIT_FIELD_GROUP_COUNT],
}

/// Defines the FastBitField interface for LargeBitField.
impl LargeBitField {
    /// Gets whether or not a specific group in the bit field has any bits set.
    ///
    /// # Arguments
    /// group_index - Provides the group to test.
    ///
    /// # Returns
    /// `Some(true)` if the group has any bits set.
    /// `Some(false)` if the group as no bits set.
    /// `None` if group_index is invalid.
    ///
    /// # Examples
    /// ```
    /// use raztos_util::collections::fast_bitfield::{FastBitField, LargeBitField};
    /// const BITS_OF: usize = core::mem::size_of::<usize>() * 8;
    ///
    /// let mut large = LargeBitField::new();
    /// let clear_value = [core::usize::MAX; BITS_OF];
    /// large.clear_field(&clear_value);
    ///
    /// assert_eq!(large.test_group(core::usize::MAX), None);
    /// assert_eq!(large.test_group(0), Some(false));
    ///
    /// large.set_bit(2);
    /// assert_eq!(large.test_group(0), Some(true));
    /// ```
    pub fn test_group(&self, group_index: usize) -> Option<bool> {
        if group_index < LARGE_BIT_FIELD_GROUP_COUNT {
            //
            // UNSAFE: The index check that makes the unsafe variant unsafe is performed before
            // calling it.
            //

            unsafe {
                return Some(self.test_group_unchecked(group_index));
            }
        }

        None
    }

    /// Sets bits in a specific group in the bit field.
    ///
    /// # Arguments
    /// group_index - Provides the group within the bit field to set.
    /// group_field - Provides the bits to set within the group.
    ///
    /// # Note
    /// If the group_index provided is larger than the number of groups in the bit field. The field
    /// will remain unchanged.
    pub fn set_group(&mut self, group_index: usize, group_field: usize) {
        if group_index < LARGE_BIT_FIELD_GROUP_COUNT {
            //
            // UNSAFE: The group_index check that makes the unsafe variant unsafe is performed before
            // calling it.
            //

            unsafe {
                self.set_group_unchecked(group_index, group_field);
            }
        }
    }

    /// Clears bits in a specific group in the bit field.
    ///
    /// # Arguments
    /// group_index - Provides the group within the bit field to clear.
    /// group_field - Provides the bits to clear within the group.
    ///
    /// # Note
    /// If the group_index provided is larger than the number of groups in the bit field. The field
    /// will remain unchanged.
    pub fn clear_group(&mut self, group_index: usize, group_field: usize) {
        if group_index < LARGE_BIT_FIELD_GROUP_COUNT {
            //
            // UNSAFE: The group_index check that makes the unsafe variant unsafe is performed before
            // calling it.
            //

            unsafe {
                self.clear_group_unchecked(group_index, group_field);
            }
        }
    }

    /// Sets bits in the bitfield
    ///
    /// # Arguments
    /// values - Provides the bits to be set in the bitfield.
    pub fn set_field(&mut self, values: &[usize; LARGE_BIT_FIELD_GROUP_COUNT]) {
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            //
            // UNSAFE: index is guaranteed to be less than the number of groups in the bitfield.
            //

            unsafe {
                self.set_group_unchecked(index, values[index]);
            }
        }
    }

    /// Clears bits in the bitfield
    ///
    /// # Arguments
    /// values - Provides the bits to be cleared in the bitfield.
    pub fn clear_field(&mut self, values: &[usize; LARGE_BIT_FIELD_GROUP_COUNT]) {
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            //
            // UNSAFE: index is guaranteed to be less than the number of groups in the bitfield.
            //

            unsafe {
                self.clear_group_unchecked(index, values[index]);
            }
        }
    }

    /// Gets whether or not a specific group in the bit field has any bits set.
    ///
    /// # Arguments
    /// group_index - Provides the group to test.
    ///
    /// # Returns
    /// `true` if the group has any bits set.
    /// `false` if the group as no bits set.
    ///
    /// # Unsafe
    /// This unsafe variant does not check if the group_index is valid for the size of
    /// the bit field. The caller must guarantee that group_index is within the number of
    /// groups in the bit field.
    ///
    /// # Examples
    /// ```
    /// use raztos_util::collections::fast_bitfield::{FastBitField, LargeBitField};
    /// const BITS_OF: usize = core::mem::size_of::<usize>() * 8;
    ///
    /// let mut large = LargeBitField::new();
    /// let clear_value = [core::usize::MAX; BITS_OF];
    /// large.clear_field(&clear_value);
    ///
    /// unsafe {
    ///     assert_eq!(large.test_group_unchecked(0), false);
    ///
    ///     large.set_bit_unchecked(2);
    ///     assert_eq!(large.test_group_unchecked(0), true);
    /// }
    /// ```
    pub unsafe fn test_group_unchecked(&self, group_index: usize) -> bool {
        (self.layer_cache & (1 << group_index)) != 0
    }

    /// Sets bits in a specific group in the bit field.
    ///
    /// # Arguments
    /// group_index - Provides the group within the bit field to set.
    /// group_field - Provides the bits to set within the group.
    ///
    /// # Unsafe
    /// This unsafe variant does not check if the group_index is valid for the size of
    /// the bit field. The caller must guarantee that group_index is within the number of
    /// groups in the bit field.
    pub unsafe fn set_group_unchecked(&mut self, group_index: usize, group_field: usize) {
        //
        // Turn boolean into a usize to avoid branching.
        //

        let field_has_values = (group_field != 0) as usize;
        let layer_cache_update = (1 << group_index) * field_has_values;

        let subfield = self.bitfield.get_unchecked_mut(group_index);
        *subfield |= group_field;

        self.layer_cache |= layer_cache_update;
    }

    /// Clears bits in a specific group in the bit field.
    ///
    /// # Arguments
    /// group_index - Provides the group within the bit field to clear.
    /// group_field - Provides the bits to clear within the group.
    ///
    /// # Unsafe
    /// This unsafe variant does not check if the group_index is valid for the size of
    /// the bit field. The caller must guarantee that group_index is within the number of
    /// groups in the bit field.
    pub unsafe fn clear_group_unchecked(&mut self, group_index: usize, group_field: usize) {
        let subfield = self.bitfield.get_unchecked_mut(group_index);
        *subfield &= !group_field;

        //
        // Turn boolean into a usize to avoid branching.
        //

        let is_clear = (*subfield == 0) as usize;
        let layer_cache_update = (1 << group_index) * is_clear;
        self.layer_cache &= !layer_cache_update;
    }
}

/// Defines the FastBitField interface for LargeBitField.
impl FastBitField for LargeBitField {
    /// Creates a new, empty LargeBitField
    ///
    /// # Returns
    /// A LargeBitField.
    fn new() -> Self {
        LargeBitField {
            layer_cache: 0,
            bitfield: [0; LARGE_BIT_FIELD_GROUP_COUNT],
        }
    }

    /// Gets the number of bits available in the bitfield type.
    ///
    /// # Returns
    /// The number of bits available.
    ///
    /// # Examples
    /// ```
    /// use raztos_util::collections::fast_bitfield::{FastBitField, LargeBitField};
    ///
    /// let bits_of = core::mem::size_of::<usize>() * 8;
    /// assert_eq!(LargeBitField::get_number_of_bits(), bits_of * bits_of);
    /// ```
    fn get_number_of_bits() -> usize {
        LARGE_BIT_FIELD_BIT_SIZE
    }

    /// Sets a bit in the bit field
    ///
    /// # Arguments
    /// index - Provides the bit to set.
    fn set_bit(&mut self, index: usize) {
        let top_layer = index / LARGE_BIT_FIELD_GROUP_COUNT;
        let bottom_layer = index % LARGE_BIT_FIELD_GROUP_COUNT;

        let sub_field = self.bitfield.get_mut(top_layer);
        let sub_field = match sub_field {
            Some(s) => s,
            None => return,
        };

        self.layer_cache |= 1 << top_layer;
        *sub_field |= 1 << bottom_layer;
    }

    /// Clears a bit in the bit field
    ///
    /// # Arguments
    /// index - Provides the bit to clear.
    fn clear_bit(&mut self, index: usize) {
        let top_layer = index / LARGE_BIT_FIELD_GROUP_COUNT;
        let bottom_layer = index % LARGE_BIT_FIELD_GROUP_COUNT;

        let sub_field = self.bitfield.get_mut(top_layer);
        let sub_field = match sub_field {
            Some(s) => s,
            None => return,
        };

        *sub_field &= !(1 << bottom_layer);
        if *sub_field == 0 {
            self.layer_cache &= !(1 << top_layer);
        }
    }

    /// Gets the lowest set bit.
    ///
    /// # Returns
    /// The lowest set bit index or `None` if no bits are set.
    ///
    /// # Examples
    /// ```
    /// use raztos_util::collections::fast_bitfield::{FastBitField, LargeBitField};
    /// const BITS_OF: usize = core::mem::size_of::<usize>() * 8;
    ///
    /// let mut large = LargeBitField::new();
    /// let clear_value = [core::usize::MAX; BITS_OF];
    /// large.clear_field(&clear_value);
    ///
    /// assert_eq!(large.get_lowest_set_bit(), None);
    ///
    /// large.set_bit(7);
    /// assert_eq!(large.get_lowest_set_bit(), Some(7));
    ///
    /// large.set_bit(9);
    /// assert_eq!(large.get_lowest_set_bit(), Some(7));
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
    /// use raztos_util::collections::fast_bitfield::{FastBitField, LargeBitField};
    /// const BITS_OF: usize = core::mem::size_of::<usize>() * 8;
    ///
    /// let mut large = LargeBitField::new();
    /// let clear_value = [core::usize::MAX; BITS_OF];
    /// large.clear_field(&clear_value);
    ///
    /// assert_eq!(large.get_highest_set_bit(), None);
    ///
    /// large.set_bit(7);
    /// assert_eq!(large.get_highest_set_bit(), Some(7));
    ///
    /// large.set_bit(9);
    /// assert_eq!(large.get_highest_set_bit(), Some(9));
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
    /// use raztos_util::collections::fast_bitfield::{FastBitField, LargeBitField};
    /// const BITS_OF: usize = core::mem::size_of::<usize>() * 8;
    ///
    /// let mut large = LargeBitField::new();
    /// let clear_value = [core::usize::MAX; BITS_OF];
    /// large.clear_field(&clear_value);
    ///
    /// assert_eq!(large.test_bit(core::usize::MAX), None);
    /// assert_eq!(large.test_bit(10), Some(false));
    ///
    /// large.set_bit(10);
    /// assert_eq!(large.test_bit(10), Some(true));
    /// ```
    fn test_bit(&self, index: usize) -> Option<bool> {
        if index < LARGE_BIT_FIELD_BIT_SIZE {
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
    /// use raztos_util::collections::fast_bitfield::{FastBitField, LargeBitField};
    ///
    /// const BITS_OF: usize = core::mem::size_of::<usize>() * 8;
    ///
    /// let mut large = LargeBitField::new();
    ///
    /// let clear_value = [core::usize::MAX; BITS_OF];
    ///
    /// large.clear_field(&clear_value);
    /// assert!(large.is_empty());
    ///
    /// large.set_bit(0);
    /// assert!(!large.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        self.layer_cache == 0
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
    /// use raztos_util::collections::fast_bitfield::{FastBitField, LargeBitField};
    /// const BITS_OF: usize = core::mem::size_of::<usize>() * 8;
    ///
    /// let mut large = LargeBitField::new();
    /// let clear_value = [core::usize::MAX; BITS_OF];
    /// large.clear_field(&clear_value);
    ///
    /// large.set_bit(7);
    /// assert_eq!(large.get_lowest_set_bit_unchecked(), 7);
    ///
    /// large.set_bit(9);
    /// assert_eq!(large.get_lowest_set_bit_unchecked(), 7);
    /// ```
    fn get_lowest_set_bit_unchecked(&self) -> usize {
        let level = find_lowest_set_bit(self.layer_cache);

        //
        // UNSAFE: level is guaranteed to be between 0 and SMALL_BIT_FIELD_SIZE - 1 by the
        // the definition of find_lowest_set_bit. No need to perform bounds checking on the array.
        //

        unsafe {
            let sub_field = self.bitfield.get_unchecked(level);
            return (level * LARGE_BIT_FIELD_GROUP_COUNT) + find_lowest_set_bit(*sub_field);
        }
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
    /// use raztos_util::collections::fast_bitfield::{FastBitField, LargeBitField};
    /// const BITS_OF: usize = core::mem::size_of::<usize>() * 8;
    ///
    /// let mut large = LargeBitField::new();
    /// let clear_value = [core::usize::MAX; BITS_OF];
    /// large.clear_field(&clear_value);
    ///
    /// large.set_bit(7);
    /// assert_eq!(large.get_highest_set_bit_unchecked(), 7);
    ///
    /// large.set_bit(9);
    /// assert_eq!(large.get_highest_set_bit_unchecked(), 9);
    /// ```
    fn get_highest_set_bit_unchecked(&self) -> usize {
        let level = find_highest_set_bit(self.layer_cache);

        //
        // UNSAFE: level is guaranteed to be between 0 and SMALL_BIT_FIELD_SIZE - 1 by the
        // the definition of find_highest_set_bit. No need to perform bounds checking on the array.
        //

        unsafe {
            let sub_field = self.bitfield.get_unchecked(level);
            return (level * LARGE_BIT_FIELD_GROUP_COUNT) + find_highest_set_bit(*sub_field);
        }
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
        let top_layer = index / LARGE_BIT_FIELD_GROUP_COUNT;
        let bottom_layer = index % LARGE_BIT_FIELD_GROUP_COUNT;

        self.layer_cache |= 1 << top_layer;
        let sub_field = self.bitfield.get_unchecked_mut(top_layer);
        *sub_field |= 1 << bottom_layer;
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
        let top_layer = index / LARGE_BIT_FIELD_GROUP_COUNT;
        let bottom_layer = index % LARGE_BIT_FIELD_GROUP_COUNT;

        let sub_field = self.bitfield.get_unchecked_mut(top_layer);
        *sub_field &= !(1 << bottom_layer);

        //
        // Turn boolean into a usize to avoid branching.
        //

        let is_clear = (*sub_field == 0) as usize;
        let layer_cache_update = (1 << top_layer) * is_clear;
        self.layer_cache &= !layer_cache_update
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
    /// use raztos_util::collections::fast_bitfield::{FastBitField, LargeBitField};
    /// const BITS_OF: usize = core::mem::size_of::<usize>() * 8;
    ///
    /// let mut large = LargeBitField::new();
    /// let clear_value = [core::usize::MAX; BITS_OF];
    /// large.clear_field(&clear_value);
    ///
    /// unsafe {
    ///     assert_eq!(large.test_bit_unchecked(10), false);
    ///
    ///     large.set_bit_unchecked(10);
    ///     assert_eq!(large.test_bit_unchecked(10), true);
    /// }
    /// ```
    unsafe fn test_bit_unchecked(&self, index: usize) -> bool {
        let top_layer = index / LARGE_BIT_FIELD_GROUP_COUNT;
        let bottom_mask = 1 << (index % LARGE_BIT_FIELD_GROUP_COUNT);

        let sub_field = self.bitfield.get_unchecked(top_layer);
        (*sub_field & bottom_mask) != 0
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
        let large = LargeBitField::new();

        assert_eq!(large.layer_cache, 0);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large.bitfield[index], 0);
        }

        assert!(large.is_empty());
    }

    //
    // Trait Tests
    //

    #[test]
    fn number_of_bits() {
        assert_eq!(
            LargeBitField::get_number_of_bits(),
            LARGE_BIT_FIELD_BIT_SIZE
        );
    }

    #[test]
    fn validate_set_bit() {
        let mut large = LargeBitField::new();
        let mut large_unsafe = LargeBitField::new();
        let mut expected_toplayer = 0 as usize;
        let mut expected_bitfield = [0 as usize; LARGE_BIT_FIELD_GROUP_COUNT];

        for i in 0..LARGE_BIT_FIELD_BIT_SIZE {
            //
            // Out of bounds set should do nothing.
            //

            large.set_bit(LARGE_BIT_FIELD_BIT_SIZE);
            assert_eq!(large.layer_cache, expected_toplayer);
            for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
                assert_eq!(large.bitfield[index], expected_bitfield[index]);
            }

            let active_group = i / LARGE_BIT_FIELD_GROUP_COUNT;
            expected_toplayer |= 1 << active_group;
            expected_bitfield[active_group] |= 1 << (i % LARGE_BIT_FIELD_GROUP_COUNT);

            large.set_bit(i);
            assert_eq!(large.layer_cache, expected_toplayer);
            for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
                assert_eq!(large.bitfield[index], expected_bitfield[index]);
            }

            //
            // Calling set for an already set bit should result in no change.
            //

            large.set_bit(i);
            assert_eq!(large.layer_cache, expected_toplayer);
            for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
                assert_eq!(large.bitfield[index], expected_bitfield[index]);
            }

            unsafe {
                large_unsafe.set_bit_unchecked(i);
                assert_eq!(large.layer_cache, expected_toplayer);
                for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
                    assert_eq!(large.bitfield[index], expected_bitfield[index]);
                }

                //
                // Calling set for an already set bit should result in no change.
                //

                large_unsafe.set_bit_unchecked(i);
                for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
                    assert_eq!(large.bitfield[index], expected_bitfield[index]);
                }
            }
        }
    }

    #[test]
    fn validate_clear_bit() {
        let mut large = LargeBitField::new();
        let mut large_unsafe = LargeBitField::new();
        let mut expected_toplayer = core::usize::MAX;
        let mut expected_bitfield = [core::usize::MAX; LARGE_BIT_FIELD_GROUP_COUNT];

        large.layer_cache = core::usize::MAX;
        large.bitfield = [core::usize::MAX; LARGE_BIT_FIELD_GROUP_COUNT];
        large_unsafe.layer_cache = core::usize::MAX;
        large_unsafe.bitfield = [core::usize::MAX; LARGE_BIT_FIELD_GROUP_COUNT];

        for i in 0..LARGE_BIT_FIELD_BIT_SIZE {
            //
            // Out of bounds set should do nothing.
            //

            large.clear_bit(LARGE_BIT_FIELD_BIT_SIZE);
            assert_eq!(large.layer_cache, expected_toplayer);
            for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
                assert_eq!(large.bitfield[index], expected_bitfield[index]);
            }

            let active_group = i / LARGE_BIT_FIELD_GROUP_COUNT;
            expected_bitfield[active_group] &= !(1 << (i % LARGE_BIT_FIELD_GROUP_COUNT));
            if expected_bitfield[active_group] == 0 {
                expected_toplayer &= !(1 << active_group);
            }

            large.clear_bit(i);
            assert_eq!(large.layer_cache, expected_toplayer);
            for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
                assert_eq!(large.bitfield[index], expected_bitfield[index]);
            }

            //
            // Calling clear for an already cleared bit should result in no change.
            //

            large.clear_bit(i);
            assert_eq!(large.layer_cache, expected_toplayer);
            for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
                assert_eq!(large.bitfield[index], expected_bitfield[index]);
            }

            unsafe {
                large_unsafe.clear_bit_unchecked(i);
                assert_eq!(large.layer_cache, expected_toplayer);
                for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
                    assert_eq!(large.bitfield[index], expected_bitfield[index]);
                }

                //
                // Calling clear for an already cleared bit should result in no change.
                //

                large_unsafe.clear_bit_unchecked(i);
                for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
                    assert_eq!(large.bitfield[index], expected_bitfield[index]);
                }
            }
        }
    }

    #[test]
    fn validate_get_lowest_set_bit() {
        let mut large = LargeBitField::new();

        //
        // Empty should result in None for checked variant
        //

        assert_eq!(large.get_lowest_set_bit(), None);

        for i in 0..LARGE_BIT_FIELD_BIT_SIZE {
            large.set_bit(i);
            assert_eq!(large.get_lowest_set_bit(), Some(0));
            assert_eq!(large.get_lowest_set_bit_unchecked(), 0);
        }

        for i in 0..LARGE_BIT_FIELD_BIT_SIZE {
            assert_eq!(large.get_lowest_set_bit(), Some(i));
            assert_eq!(large.get_lowest_set_bit_unchecked(), i);
            large.clear_bit(i);
        }
    }

    #[test]
    fn validate_get_highest_set_bit() {
        let mut large = LargeBitField::new();

        //
        // Empty should result in None for checked variant
        //

        assert_eq!(large.get_highest_set_bit(), None);

        for i in 0..LARGE_BIT_FIELD_BIT_SIZE {
            large.set_bit(i);
            assert_eq!(large.get_highest_set_bit(), Some(i));
            assert_eq!(large.get_highest_set_bit_unchecked(), i);
        }

        for i in 0..LARGE_BIT_FIELD_BIT_SIZE {
            assert_eq!(
                large.get_highest_set_bit(),
                Some(LARGE_BIT_FIELD_BIT_SIZE - 1)
            );
            assert_eq!(
                large.get_highest_set_bit_unchecked(),
                LARGE_BIT_FIELD_BIT_SIZE - 1
            );
            large.clear_bit(i);
        }
    }

    #[test]
    fn validate_test_bit() {
        let mut large = LargeBitField::new();

        //
        // Out of bounds should return None for checked variant
        //

        assert_eq!(large.test_bit(LARGE_BIT_FIELD_BIT_SIZE), None);

        //
        // Set causes test to return true.
        //

        large.set_bit(0);
        assert_eq!(large.test_bit(0), Some(true));
        unsafe {
            assert_eq!(large.test_bit_unchecked(0), true);
        }

        //
        // Clear causes test to return false.
        //s

        large.clear_bit(0);
        assert_eq!(large.test_bit(0), Some(false));
        unsafe {
            assert_eq!(large.test_bit_unchecked(0), false);
        }

        //
        // Changing another bit has no affect on the bit being tested.
        //

        large.set_bit(1);
        assert_eq!(large.test_bit(0), Some(false));
        unsafe {
            assert_eq!(large.test_bit_unchecked(0), false);
        }

        //
        // Clear causes test to return false.
        //

        large.set_bit(0);
        large.clear_bit(1);
        assert_eq!(large.test_bit(0), Some(true));
        unsafe {
            assert_eq!(large.test_bit_unchecked(0), true);
        }
    }

    //
    // Method Tests
    //

    #[test]
    fn validate_set_and_clear_field() {
        let mut large = LargeBitField::new();
        let mut expected_toplayer: usize = 0;
        let mut expected_bitfield = [0 as usize; LARGE_BIT_FIELD_GROUP_COUNT];

        let zeros = [0 as usize; LARGE_BIT_FIELD_GROUP_COUNT];
        let fives =
            [(0x55555555_55555555 & core::usize::MAX) as usize; LARGE_BIT_FIELD_GROUP_COUNT];

        let a_s = [(0xAAAAAAAA_AAAAAAAA & core::usize::MAX) as usize; LARGE_BIT_FIELD_GROUP_COUNT];
        let f_s = [(0xFFFFFFFF_FFFFFFFF & core::usize::MAX) as usize; LARGE_BIT_FIELD_GROUP_COUNT];

        //
        // Calling set with 0 results in no change.
        //

        assert_eq!(large.layer_cache, 0);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large.bitfield[index], zeros[index]);
        }

        large.set_field(&zeros);

        assert_eq!(large.layer_cache, 0);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large.bitfield[index], zeros[index]);
        }

        //
        // Setting only sets bits expected bits.
        //

        expected_bitfield[1 / LARGE_BIT_FIELD_GROUP_COUNT] |=
            1 << (1 % LARGE_BIT_FIELD_GROUP_COUNT);

        expected_toplayer |= 1 << (1 / LARGE_BIT_FIELD_GROUP_COUNT);

        large.set_bit(1);

        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            expected_bitfield[index] |= fives[index];
            if fives[index] != 0 {
                expected_toplayer |= 1 << index;
            }
        }

        large.set_field(&fives);

        assert_eq!(large.layer_cache, expected_toplayer);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large.bitfield[index], expected_bitfield[index]);
        }

        //
        // Settings already set values should result in no change.
        //

        large.set_field(&fives);

        assert_eq!(large.layer_cache, expected_toplayer);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large.bitfield[index], expected_bitfield[index]);
        }

        large.set_field(&a_s);
        assert_eq!(large.layer_cache, core::usize::MAX);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large.bitfield[index], f_s[index]);
        }

        //
        // Clearing only clears expected bits.
        //

        large.clear_field(&fives);
        assert_eq!(large.layer_cache, core::usize::MAX);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large.bitfield[index], a_s[index]);
        }

        //
        // Clearing already cleared values should result in no change.
        //

        large.clear_field(&fives);
        assert_eq!(large.layer_cache, core::usize::MAX);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large.bitfield[index], a_s[index]);
        }

        //
        // Calling clear with 0 results in no change.
        //

        large.clear_field(&zeros);
        assert_eq!(large.layer_cache, core::usize::MAX);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large.bitfield[index], a_s[index]);
        }
    }

    #[test]
    fn validate_set_and_clear_group() {
        let mut large = LargeBitField::new();
        let mut large_unsafe = LargeBitField::new();
        let mut expected_toplayer: usize = 0;
        let mut expected_bitfield = [0 as usize; LARGE_BIT_FIELD_GROUP_COUNT];
        let fives = (0x55555555_55555555 & core::usize::MAX) as usize;
        let first_group = 0;
        let second_group = 2;
        let third_group = 5;

        //
        // Verify Set Group
        //

        expected_toplayer |= 1 << first_group;
        expected_bitfield[first_group] |= fives;

        expected_toplayer |= 1 << second_group;
        expected_bitfield[second_group] |= fives;

        large.set_group(first_group, fives);
        large.set_group(second_group, fives);
        assert_eq!(large.layer_cache, expected_toplayer);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large.bitfield[index], expected_bitfield[index]);
        }

        unsafe {
            large_unsafe.set_group_unchecked(first_group, fives);
            large_unsafe.set_group_unchecked(second_group, fives);
        }

        assert_eq!(large_unsafe.layer_cache, expected_toplayer);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large_unsafe.bitfield[index], expected_bitfield[index]);
        }

        //
        // Calling set out of bounds results in no change
        //

        large.set_group(LARGE_BIT_FIELD_GROUP_COUNT, fives);
        assert_eq!(large.layer_cache, expected_toplayer);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large.bitfield[index], expected_bitfield[index]);
        }

        //
        // Calling set with 0, will result in no change
        //

        large.set_group(third_group, 0);
        assert_eq!(large.layer_cache, expected_toplayer);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large.bitfield[index], expected_bitfield[index]);
        }

        unsafe {
            large_unsafe.set_group_unchecked(third_group, 0);
        }

        assert_eq!(large_unsafe.layer_cache, expected_toplayer);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large_unsafe.bitfield[index], expected_bitfield[index]);
        }

        //
        // Verify Clear Group
        //

        expected_toplayer &= !(1 << first_group);
        expected_bitfield[first_group] &= !fives;

        large.clear_group(first_group, fives);
        assert_eq!(large.layer_cache, expected_toplayer);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large.bitfield[index], expected_bitfield[index]);
        }

        unsafe {
            large_unsafe.clear_group_unchecked(first_group, fives);
        }

        assert_eq!(large_unsafe.layer_cache, expected_toplayer);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large_unsafe.bitfield[index], expected_bitfield[index]);
        }

        //
        // Calling clear out of bounds results in no change
        //

        large.clear_group(LARGE_BIT_FIELD_GROUP_COUNT, fives);
        assert_eq!(large.layer_cache, expected_toplayer);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large.bitfield[index], expected_bitfield[index]);
        }

        //
        // Calling clear with 0, will result in no change
        //

        large.clear_group(second_group, 0);
        assert_eq!(large.layer_cache, expected_toplayer);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large.bitfield[index], expected_bitfield[index]);
        }

        unsafe {
            large_unsafe.clear_group_unchecked(second_group, 0);
        }

        assert_eq!(large_unsafe.layer_cache, expected_toplayer);
        for index in 0..LARGE_BIT_FIELD_GROUP_COUNT {
            assert_eq!(large_unsafe.bitfield[index], expected_bitfield[index]);
        }
    }

    #[test]
    fn validate_test_group() {
        let mut large = LargeBitField::new();
        let bit = 20;
        let different_group_bit = bit + LARGE_BIT_FIELD_GROUP_COUNT;

        //
        // Out of bounds should return None for checked variant
        //

        assert_eq!(large.test_group(LARGE_BIT_FIELD_GROUP_COUNT), None);

        //
        // Set causes test to return true.
        //

        large.set_bit(bit);
        assert_eq!(
            large.test_group(bit / LARGE_BIT_FIELD_GROUP_COUNT),
            Some(true)
        );
        unsafe {
            assert_eq!(
                large.test_group_unchecked(bit / LARGE_BIT_FIELD_GROUP_COUNT),
                true
            );
        }

        //
        // Clear causes test to return false.
        //

        large.clear_bit(bit);
        assert_eq!(
            large.test_group(bit / LARGE_BIT_FIELD_GROUP_COUNT),
            Some(false)
        );
        unsafe {
            assert_eq!(
                large.test_group_unchecked(bit / LARGE_BIT_FIELD_GROUP_COUNT),
                false
            );
        }

        //
        // Changing another group has no affect on the bit being tested.
        //

        large.set_bit(different_group_bit);
        assert_eq!(
            large.test_group(bit / LARGE_BIT_FIELD_GROUP_COUNT),
            Some(false)
        );
        unsafe {
            assert_eq!(
                large.test_group_unchecked(bit / LARGE_BIT_FIELD_GROUP_COUNT),
                false
            );
        }

        //
        // Clear causes test to return false.
        //

        large.set_bit(bit);
        large.clear_bit(different_group_bit);
        assert_eq!(
            large.test_group(bit / LARGE_BIT_FIELD_GROUP_COUNT),
            Some(true)
        );
        unsafe {
            assert_eq!(
                large.test_group_unchecked(bit / LARGE_BIT_FIELD_GROUP_COUNT),
                true
            );
        }
    }
}

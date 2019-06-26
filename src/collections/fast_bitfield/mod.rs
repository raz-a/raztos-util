//! # Fast Bitfield
//! `fast_bitfield` defines the interface as well as structures for fast bitfields.
//! Fast bitfields are bitfields that can evaluate the lowest and highest set bits, set and clear
//! bits, and check for empty quickly and in a constant time invariant (or nearly invariant) of the
//! contents of the bitfield.

use crate::cpu_features::opcodes;
use crate::algorithms::debruijin;

/// Defines the required functionality for fast bitfields
pub trait FastBitField {
    /// Creates a new, empty FastBitField
    ///
    /// # Returns
    /// A FastBitField.
    fn new() -> Self;

    /// Gets the number of bits available in the bitfield type.
    ///
    /// # Returns
    /// The number of bits available.
    fn get_number_of_bits() -> usize;

    /// Sets a bit in the bit field
    ///
    /// # Arguments
    /// index - Provides the bit to set.
    fn set_bit(&mut self, index: usize);

    /// Clears a bit in the bit field
    ///
    /// # Arguments
    /// index - Provides the bit to clear.
    fn clear_bit(&mut self, index: usize);

    /// Gets the lowest set bit.
    ///
    /// # Returns
    /// The lowest set bit index or `None` if no bits are set.
    fn get_lowest_set_bit(&self) -> Option<usize>;

    /// Gets the highest set bit.
    ///
    /// # Returns
    /// The highest set bit index or `None` if no bits are set.
    fn get_highest_set_bit(&self) -> Option<usize>;

    /// Gets the value of a specific bit in the bit field.
    ///
    /// # Arguments
    /// index - Provides the bit to test.
    ///
    /// # Returns
    /// `Some(true)` if bit is set.
    /// `Some(false)` if bit is cleared.
    /// `None` if index is invalid.
    fn test_bit(&self, index: usize) -> Option<bool>;

    /// Determines whether or not the bitfield is empty.
    ///
    /// # Returns
    /// `true` if empty, `false` otherwise.
    fn is_empty(&self) -> bool;

    /// Gets the lowest set bit, guaranteed to have no branches and be in constant time, completely
    /// invariant of the state of the bit field. If no bits are set, the result is undefined.
    ///
    /// This function should only be used if the caller can guarantee the bitfield will always
    /// have at least one bit set.
    ///
    /// # Returns
    /// The lowest set bit index or `UNDEFINED` if no bits are set.
    fn get_lowest_set_bit_unchecked(&self) -> usize;

    /// Gets the highest set bit, guaranteed to have no branches and be in constant time, completely
    /// invariant of the state of the bit field. If no bits are set, the result is undefined.
    ///
    /// This function should only be used if the caller can guarantee the bitfield will always
    /// have at least one bit set.
    ///
    /// # Returns
    /// The highest set bit index or `UNDEFINED` if no bits are set.
    fn get_highest_set_bit_unchecked(&self) -> usize;

    /// Sets a bit in the bit field.
    ///
    /// # Arguments
    /// index - Provides the bit to set.
    ///
    /// # Unsafe
    /// This unsafe variant does not check if the index is valid for the size of
    /// the bit field. The caller must guarantee that the index is less than `get_number_of_bits()`.
    unsafe fn set_bit_unchecked(&mut self, index: usize);

    /// Clears a bit in the bit field
    ///
    /// # Arguments
    /// index - Provides the bit to clear.
    ///
    /// # Unsafe
    /// This unsafe variant does not check if the index is valid for the size of
    /// the bit field. The caller must guarantee that the index is less than `get_number_of_bits()`.
    unsafe fn clear_bit_unchecked(&mut self, index: usize);

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
    unsafe fn test_bit_unchecked(&self, index: usize) -> bool;
}

/// Defines a fast bitfield that can hold `sizeof(usize) * 8` bits.
mod small_bitfield;
pub use small_bitfield::SmallBitField;

/// Defines a fast bitfield that can hold `sizeof(usize) * sizeof(usize) * 8` bits.
mod large_bitfield;
pub use large_bitfield::LargeBitField;

/// Gets the lowest set bit of a usize value.
///
/// # Arguments
/// value - The value to find the lowest set bit for.
///
/// # Returns
/// The lowest set bit index or `UNDEFINED` if no bits are set.
fn find_lowest_set_bit(value: usize) -> usize {
    if opcodes::count_leading_zeros_exists() {
        value.trailing_zeros() as usize
    } else {
        debruijin::get_lowest_set_bit(value)
    }
}

/// Gets the highest set bit of a usize value.
///
/// # Arguments
/// value - The value to find the highest set bit for.
///
/// # Returns
/// The highest set bit index or `UNDEFINED` if no bits are set.
fn find_highest_set_bit(value: usize) -> usize {
    if opcodes::count_leading_zeros_exists() {
        (core::mem::size_of::<usize>() * 8) - 1 - value.leading_zeros() as usize
    } else {
        debruijin::get_highest_set_bit(value)
    }
}

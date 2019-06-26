//! # De Bruijin Lowest and Highest Set Bit
//!
//! `debruijin` contains utilities to find the lowest and highest set bits in constant time using
//! De Bruijin Sequences
//!
//! # What is a De Bruijin Sequence?
//!
//! A size N cyclical sequence of bits, where N is a power of 2, in which every 0-1 sequence
//! of length log2(N) occurs exactly once as a contiguous substring.
//!
//! ### Example:
//!
//! 8-bits: `0x17` = `00010111`\
//! Segments of length 3:\
//! `000` (bit 7,6,5) \
//! `001` (bit 6,5,4) \
//! `010` (bit 5,4,3) \
//! `101` (bit 4,3,2) \
//! `011` (bit 3,2,1) \
//! `111` (bit 2,1,0) \
//! `110` (bit 1,0,7) \
//! `100` (bit 0,7,6)
//!
//! # How does a De Bruijin Sequence help us find the lowest and highest set bit?
//!
//! When rotated left or right by a value between 0 and N-1, a window of size log2(N) in the
//! De Bruijin sequence will be a unique number also between 0 and N-1.
//!
//! If we choose to use a De Bruijin Sequence that begins with log2(N) leading zeros, then a
//! left rotation of values 0 to log2(N) -1 is equivalent to a shift left (since the 0's being
//! shifted in would be equivalent to the 0's being rotated in). For the top log2(N) window, it
//! will be equivalent for left shifts between 0 and N-1.
//!
//! **Still, where does this leave us?**
//!
//! A shift left of x is equivalent to a multiply by 2^x so multiplying a De Bruijin sequence
//! with log2(N) leading zeros by a power of two between 1 and 2^(N-1) would be equivalent to
//! rotating left at the top window by values between 0 and N-1.
//!
//! Isolating the lowest bit of an N bit value is simple enough by anding the value with its
//! two's compliment, creating a power of 2 value. If we multiply our De Bruijin sequence
//! holding log2(N) leading zeros by the isolated bit and shifting down the top window, we
//! will get a unique value from 0 to N-1 for each possible isolated bit. Since all numbers are
//! included exactly once, we can use this as a mapping to the actual lowest set bit value.
//!
//! To find the highest bit, simply reverse the bits, find the lowest set bit, and then
//! subtract the result from [Number Of Bits] - 1.


//
// Constants - 8-bit
//

const DEBRUIJIN_NUMBER_8: u8 = 0x17;

const DEBRUIJIN_INDICES_8: [u8; 8] = [00, 01, 02, 04, 07, 03, 06, 05];

const DEBRUIJIN_SHIFT_8: u8 = 5;

//
// Constants - 16-bit
//

const DEBRUIJIN_NUMBER_16: u16 = 0x09AF;

const DEBRUIJIN_INDICES_16: [u16; 16] = [
    00, 01, 02, 05, 03, 09, 06, 11, 15, 04, 08, 10, 14, 07, 13, 12,
];

const DEBRUIJIN_SHIFT_16: u16 = 12;

//
// Constants - 32-bit
//

const DEBRUIJIN_NUMBER_32: u32 = 0x04653ADF;

const DEBRUIJIN_INDICES_32: [u32; 32] = [
    00, 01, 02, 06, 03, 11, 07, 16, 04, 14, 12, 21, 08, 23, 17, 26, 31, 05, 10, 15, 13, 20, 22, 25,
    30, 09, 19, 24, 29, 18, 28, 27,
];

const DEBRUIJIN_SHIFT_32: u32 = 27;

//
// Constants - 64-bit
//

const DEBRUIJIN_NUMBER_64: u64 = 0x0218A392_CD3D5DBF;

const DEBRUIJIN_INDICES_64: [u64; 64] = [
    00, 01, 02, 07, 03, 13, 08, 19, 04, 25, 14, 28, 09, 34, 20, 40, 05, 17, 26, 38, 15, 46, 29, 48,
    10, 31, 35, 54, 21, 50, 41, 57, 63, 06, 12, 18, 24, 27, 33, 39, 16, 37, 45, 47, 30, 53, 49, 56,
    62, 11, 23, 32, 36, 44, 52, 55, 61, 22, 43, 51, 60, 42, 59, 58,
];

const DEBRUIJIN_SHIFT_64: u64 = 58;

//
// Private Functions
//

fn get_lowest_set_bit_8(value: u8) -> u8 {
    let isolated_bit = value & ((!value).wrapping_add(1));
    let index = DEBRUIJIN_NUMBER_8.wrapping_mul(isolated_bit) >> DEBRUIJIN_SHIFT_8;
    DEBRUIJIN_INDICES_8[index as usize]
}

fn get_lowest_set_bit_16(value: u16) -> u16 {
    let isolated_bit = value & ((!value).wrapping_add(1));
    let index = DEBRUIJIN_NUMBER_16.wrapping_mul(isolated_bit) >> DEBRUIJIN_SHIFT_16;
    DEBRUIJIN_INDICES_16[index as usize]
}

fn get_lowest_set_bit_32(value: u32) -> u32 {
    let isolated_bit = value & ((!value).wrapping_add(1));
    let index = DEBRUIJIN_NUMBER_32.wrapping_mul(isolated_bit) >> DEBRUIJIN_SHIFT_32;
    DEBRUIJIN_INDICES_32[index as usize]
}

fn get_lowest_set_bit_64(value: u64) -> u64 {
    let isolated_bit = value & ((!value).wrapping_add(1));
    let index = DEBRUIJIN_NUMBER_64.wrapping_mul(isolated_bit) >> DEBRUIJIN_SHIFT_64;
    DEBRUIJIN_INDICES_64[index as usize]
}

//
// Public Functions
//

/// Gets the lowest set bit in constant time using a De Bruijin Sequence and Lookup Table.
///
/// # Examples
///
/// ```
/// let result = debruijin::get_lowest_set_bit(5);
/// assert_eq!(result, 0);
///```
/// # Note
///
/// If the input value is 0. This function will return 0.
///
pub fn get_lowest_set_bit(value: usize) -> usize {
    if cfg!(target_pointer_width = "8") {
        get_lowest_set_bit_8(value as u8) as usize
    } else if cfg!(target_pointer_width = "16") {
        get_lowest_set_bit_16(value as u16) as usize
    } else if cfg!(target_pointer_width = "32") {
        get_lowest_set_bit_32(value as u32) as usize
    } else if cfg!(target_pointer_width = "64") {
        get_lowest_set_bit_64(value as u64) as usize
    } else {
        0
    }
}

/// Gets the highest set bit in constant time using a De Bruijin Sequence and Lookup Table.
///
/// # Examples
///
/// ```
/// let result = debruijin::get_highest_set_bit(5);
/// assert_eq!(result, 2);
///```
///
/// # Note
///
/// If the input value is 0. This function will return 0.
///
pub fn get_highest_set_bit(value: usize) -> usize {
    let bits_of = core::mem::size_of::<usize>() * 8;
    bits_of - get_lowest_set_bit(value.reverse_bits()) - 1
}

//
// Unit Tests
//

#[cfg(test)]
mod tests {
    use super::*;

    //
    // Helper functions
    //

    fn simple_lowest_set_bit(value: usize) -> usize {
        for index in 0..core::mem::size_of::<u64>() * 8 {
            if (value & (1 << index)) != 0 {
                return index;
            }
        }

        //
        // "Edge Case", no bits set. Return 0 to match De Bruijin functionality
        //

        0
    }

    //
    // 8-bit Tests
    //

    #[test]
    fn debruijin8() {
        println!(" --- 8-bit Debruijin Test ---");

        //
        // 8-bits is small enough, just test everything
        //

        for value in u8::min_value()..=u8::max_value() {
            let simple = simple_lowest_set_bit(value as usize);
            let debruijin = get_lowest_set_bit_8(value) as usize;

            println!("{:X}: Expected: {}, Actual {}", value, simple, debruijin);

            assert_eq!(simple, debruijin);
        }
    }

    //
    // 16-bit Tests
    //

    #[test]
    fn debruijin16() {
        println!(" --- 16-bit Debruijin Test ---");

        //
        // 16-bits is also small enough, just test everything
        //

        for value in u16::min_value()..=u16::max_value() {
            let simple = simple_lowest_set_bit(value as usize);
            let debruijin = get_lowest_set_bit_16(value) as usize;

            println!("{:X}: Expected: {}, Actual {}", value, simple, debruijin);

            assert_eq!(simple, debruijin);
        }
    }

    //
    // 32-bit Tests
    //

    #[test]
    fn debruijin32() {
        println!(" --- 32-bit Debruijin Test ---");

        //
        // 32-bits is too big! Test individual isolated bits
        //

        let mut isolated_bit = 1;

        loop {
            let simple = simple_lowest_set_bit(isolated_bit as usize);
            let debruijin = get_lowest_set_bit_32(isolated_bit) as usize;

            println!(
                "{:X}: Expected: {}, Actual {}",
                isolated_bit, simple, debruijin
            );

            assert_eq!(simple, debruijin);

            if isolated_bit == 0x8000_0000 {
                break;
            }

            isolated_bit = isolated_bit << 1;
        }

        //
        // Test 0
        //

        let debruijin = get_lowest_set_bit_32(0) as usize;
        println!("{:X}: Expected: {}, Actual {}", 0, 0, debruijin);
        assert_eq!(0, debruijin);

        //
        // Test 0xFFFFFFFF
        //

        let debruijin = get_lowest_set_bit_32(0xFFFF_FFFF) as usize;
        println!(
            "{:X}: Expected: {}, Actual {}",
            0xFFFF_FFFFu32, 0, debruijin
        );
        assert_eq!(0, debruijin);

        //
        // Test 0xAAAAAAAA
        //

        let debruijin = get_lowest_set_bit_32(0xAAAA_AAAA) as usize;
        println!(
            "{:X}: Expected: {}, Actual {}",
            0xAAAA_AAAAu32, 1, debruijin
        );
        assert_eq!(1, debruijin);
    }

    //
    // 64-bit Tests
    //

    #[test]
    fn debruijin64() {
        println!(" --- 64-bit Debruijin Test ---");

        //
        // 64-bits is too big! Test individual isolated bits
        //

        let mut isolated_bit = 1;

        loop {
            let simple = simple_lowest_set_bit(isolated_bit as usize);
            let debruijin = get_lowest_set_bit_64(isolated_bit) as usize;

            println!(
                "{:X}: Expected: {}, Actual {}",
                isolated_bit, simple, debruijin
            );

            assert_eq!(simple, debruijin);

            if isolated_bit == 0x8000_0000_0000_0000 {
                break;
            }

            isolated_bit = isolated_bit << 1;
        }

        //
        // Test 0
        //

        let debruijin = get_lowest_set_bit_64(0) as usize;
        println!("{:X}: Expected: {}, Actual {}", 0, 0, debruijin);
        assert_eq!(0, debruijin);

        //
        // Test 0xFFFFFFFF`FFFFFFFF
        //

        let debruijin = get_lowest_set_bit_64(0xFFFF_FFFF_FFFF_FFFF) as usize;
        println!(
            "{:X}: Expected: {}, Actual {}",
            0xFFFF_FFFF_FFFF_FFFFu64, 0, debruijin
        );

        assert_eq!(0, debruijin);

        //
        // Test 0xAAAAAAAA`AAAAAAAA
        //

        let debruijin = get_lowest_set_bit_64(0xAAAA_AAAA_AAAA_AAAA) as usize;
        println!(
            "{:X}: Expected: {}, Actual {}",
            0xAAAA_AAAA_AAAA_AAAAu64, 1, debruijin
        );

        assert_eq!(1, debruijin);
    }
}

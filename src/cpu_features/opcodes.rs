//! # CPU Opcode Feaures
//!
//! Contains compile time evaluations for the presence of helpful opcodes.

/// Returns whether or not this platform has a `Count Leading Zeros` instruction
#[inline(always)]
pub fn count_leading_zeros_exists() -> bool {
    if cfg!(target_arch = "arm") {
        if cfg!(target_feature = "mclass") {
            if cfg!(target_feature = "v7") {
                true
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

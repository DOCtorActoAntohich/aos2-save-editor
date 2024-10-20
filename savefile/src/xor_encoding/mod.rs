pub mod key;
pub mod u8;

/// Nibbles are the most or the least significant 4 bits.
/// In Hex `0x8A`, `8` is the highest nibble, and `A` is the lowest.
/// So, after swaping, the number will be `0xA8.
///
/// # Examples
///
/// ```rust
/// use savefile::xor_encoding::swap_nibbles;
///
/// assert_eq!(0x8A, swap_nibbles(0xA8));
/// ```
pub const fn swap_nibbles(byte: u8) -> u8 {
    const HALF_BYTE: u32 = 4;
    byte.rotate_left(HALF_BYTE)
}

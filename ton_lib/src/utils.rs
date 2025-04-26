// return false if preconditions are not met
pub fn rewrite_bits(src: &[u8], src_offset_bits: usize, dst: &mut [u8], dst_offset_bits: usize, len: usize) -> bool {
    // Calculate total bits available in source and destination
    let src_total_bits = src.len() * 8;
    let dst_total_bits = dst.len() * 8;

    // Check preconditions
    if src_offset_bits + len > src_total_bits || dst_offset_bits + len > dst_total_bits {
        return false;
    }

    for i in 0..len {
        // Calculate the source bit position and extract the bit
        let src_bit_pos = src_offset_bits + i;
        let src_byte_index = src_bit_pos / 8;
        let src_bit_offset = 7 - (src_bit_pos % 8); // MSB is bit 7
        let src_bit = (src[src_byte_index] >> src_bit_offset) & 1;

        // Calculate the destination bit position and write the bit
        let dst_bit_pos = dst_offset_bits + i;
        let dst_byte_index = dst_bit_pos / 8;
        let dst_bit_offset = 7 - (dst_bit_pos % 8); // MSB is bit 7

        // Clear the target bit and set it to the source bit value
        dst[dst_byte_index] &= !(1 << dst_bit_offset); // Clear the bit
        dst[dst_byte_index] |= src_bit << dst_bit_offset; // Set the bit
    }

    true
}

#[cfg(feature = "sys")]
pub fn tonlib_set_verbosity_level(level: u32) {
    unsafe {
        tonlib_sys::tonlib_client_set_verbosity_level(level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rewrite_bits() {
        let src = vec![0b11001100, 0b10101010]; // Source bits
        let mut dst = vec![0b00000000, 0b00000000]; // Destination bits
        assert!(rewrite_bits(&src, 4, &mut dst, 8, 8));
        assert_eq!(dst, vec![0b00000000, 0b11001010]);

        let src = vec![0b11001100, 0b10101010]; // Source bits
        let mut dst = vec![0b00000000, 0b00000000]; // Destination bits
        assert!(rewrite_bits(&src, 0, &mut dst, 0, 16));
        assert_eq!(dst, src);

        let src = vec![0b11001100, 0b10101010]; // Source bits
        let mut dst = vec![0b00000000, 0b00000000]; // Destination bits
        assert!(rewrite_bits(&src, 0, &mut dst, 0, 8));
        assert_eq!(dst[0], src[0]);
        assert_eq!(dst[1], 0b00000000);

        assert!(!rewrite_bits(&src, 14, &mut dst, 6, 10));
    }
}

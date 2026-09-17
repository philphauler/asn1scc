#[cfg(test)]
mod tests {
    use crate::acn::*;
    use crate::BitStream;

    fn reset_for_decode(buf: &mut [u8]) -> BitStream {
        BitStream::attach_buffer_no_zero(buf)
    }

    // Null-terminated ASCII strings into [u8; max] buffers.
    //
    // The C runtime assumes max + 1 bytes for the terminator; the Rust backend
    // generates fixed-length strings as [u8; max]. A full-length string used to
    // index str_val[max] and panic ("index out of bounds: the len is 3 but the
    // index is 3").

    #[test]
    fn null_terminated_full_length_string_fits_max_buffer() {
        let mut stream = [b'A', b'B', b'C', 0u8];
        let mut bs = reset_for_decode(&mut stream);
        let mut out = [0u8; 3];
        assert!(acn_dec_string_ascii_null_terminated(&mut bs, 3, 0, &mut out));
        assert_eq!(&out, b"ABC");
    }

    #[test]
    fn null_terminated_string_with_room_stores_terminator() {
        let mut stream = [b'A', b'B', 0u8, 0xFF];
        let mut bs = reset_for_decode(&mut stream);
        let mut out = [0xEEu8; 4];
        assert!(acn_dec_string_ascii_null_terminated(&mut bs, 3, 0, &mut out));
        assert_eq!(&out[..3], b"AB\0");
    }

    #[test]
    fn null_terminated_string_without_terminator_fails_cleanly() {
        let mut stream = [b'A', b'B', b'C', b'D'];
        let mut bs = reset_for_decode(&mut stream);
        let mut out = [0u8; 3];
        assert!(!acn_dec_string_ascii_null_terminated(&mut bs, 3, 0, &mut out));
    }

    #[test]
    fn null_terminated_mult_full_length_string_fits_max_buffer() {
        let mut stream = [b'A', b'B', b'C', 0u8];
        let mut bs = reset_for_decode(&mut stream);
        let mut out = [0u8; 3];
        assert!(acn_dec_string_ascii_null_terminated_mult(&mut bs, 3, &[0u8], &mut out));
        assert_eq!(&out, b"ABC");
    }

    #[test]
    fn null_terminated_mult_without_terminator_fails_cleanly() {
        let mut stream = [b'A', b'B', b'C', b'D'];
        let mut bs = reset_for_decode(&mut stream);
        let mut out = [0u8; 3];
        assert!(!acn_dec_string_ascii_null_terminated_mult(&mut bs, 3, &[0u8], &mut out));
    }

    #[test]
    fn null_terminated_mult_two_byte_terminator_roundtrip() {
        let mut stream = [b'H', b'i', 0xAA, 0xBB, 0x11];
        let mut bs = reset_for_decode(&mut stream);
        let mut out = [0u8; 5];
        assert!(acn_dec_string_ascii_null_terminated_mult(&mut bs, 4, &[0xAA, 0xBB], &mut out));
        assert_eq!(&out[..3], b"Hi\0");
    }
}

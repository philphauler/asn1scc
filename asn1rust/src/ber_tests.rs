#[cfg(test)]
mod tests {
    use crate::ber::*;
    use crate::{ByteStream, ErrorCode};

    fn stream(buf: &mut [u8]) -> ByteStream {
        let count = buf.len() as i64;
        ByteStream { buf, count, current_byte: 0, encode_white_space: false }
    }

    #[test]
    fn length_above_int_max_is_rejected_not_negative() {
        // Long form, 4 length bytes: 0x9000_0000 fits a u32 but not an i32.
        let mut buf = [0x84u8, 0x90, 0x00, 0x00, 0x00];
        let mut bs = stream(&mut buf);
        let mut value: i32 = 0;
        let mut err = ErrorCode::InsufficientData;
        let ok = ber_decode_length(&mut bs, &mut value, &mut err);
        assert!(!ok, "a length above INT_MAX must fail as the C runtime does");
        assert_eq!(err, ErrorCode::BerLengthMismatch);
    }

    #[test]
    fn length_at_int_max_is_accepted() {
        let mut buf = [0x84u8, 0x7F, 0xFF, 0xFF, 0xFF];
        let mut bs = stream(&mut buf);
        let mut value: i32 = 0;
        let mut err = ErrorCode::InsufficientData;
        assert!(ber_decode_length(&mut bs, &mut value, &mut err));
        assert_eq!(value, i32::MAX);
    }

    #[test]
    fn short_and_indefinite_lengths_unchanged() {
        let mut buf = [0x05u8];
        let mut bs = stream(&mut buf);
        let mut value: i32 = 0;
        let mut err = ErrorCode::InsufficientData;
        assert!(ber_decode_length(&mut bs, &mut value, &mut err));
        assert_eq!(value, 5);

        let mut buf2 = [0x80u8];
        let mut bs2 = stream(&mut buf2);
        assert!(ber_decode_length(&mut bs2, &mut value, &mut err));
        assert_eq!(value, -1);
    }
}

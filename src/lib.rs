#![doc = include_str!("../README.md")]

#[macro_export]
#[doc(hidden)]
macro_rules! handle_digit {
    ($digit:expr) => {{
        // convert the char to an integer
        match $digit {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'3' => 3,
            b'4' => 4,
            b'5' => 5,
            b'6' => 6,
            b'7' => 7,
            b'8' => 8,
            b'9' => 9,
            _ => panic!("non-ascii digit found"),
        }
    }}
}

/// Const conversion of integer strings to any signed integer type
/// 
/// ```rust
/// # use intify_str::intify_str_unsigned;
/// let value: u8 = intify_str_unsigned!("128");
/// ```
#[macro_export]
macro_rules! intify_str_unsigned {
    ($s:expr) => {{
        let bytes = $s.as_bytes();
        let mut idx = 0;
        let mut out = 0;
        // for each char in the string
        while idx < bytes.len() {
            let c = bytes[idx];
            let value = $crate::handle_digit!(c);
            // add another "place" to the out value (initially 0 so no addition on first step)
            out *= 10;
            // put the provided digit into the newly added place
            out += value;
            // move on to the next character
            idx += 1;
        }
        out
    }};
}

/// Const conversion of integer strings to any signed integer type
/// 
/// ```rust
/// # use intify_str::intify_str_signed;
/// let value: i8 = intify_str_signed!("-128");
/// ```
#[macro_export]
macro_rules! intify_str_signed {
    ($s:expr) => {{
        let bytes = $s.as_bytes();
        let first = bytes[0];
        let mut negate = false;
        let slice = if first == b'-' || first == b'+' {
            negate = first == b'-';
            &bytes[1..]
        } else {
            bytes
        };
        let mut idx = 0;
        let mut out = 0;
        // for each char in the string
        while idx < slice.len() {
            let c = slice[idx];
            let value = $crate::handle_digit!(c);
            // add another "place" to the out value (initially 0 so no addition on first step)
            out *= 10;
            // if we are at the last index
            if idx+1 == slice.len() {
                // if the value is negative
                if negate {
                    // invert the sign to negative
                    out = -out;
                    // subtract the final position
                    out -= value;
                    break;
                }
            }
            // put the provided digit into the newly added place
            out += value;
            // move on to the next character
            idx += 1;
        }
        out
    }};
}

#[cfg(test)]
mod tests {
    use proptest::proptest;

    use super::*;

    #[test]
    fn round_trip_all_u8s() {
        for i in 0..=u8::MAX {
            let s = i.to_string();
            let v: u8 = intify_str_unsigned!(&s);
            assert_eq!(i, v);
        }
    }

    #[test]
    fn round_trip_all_u16s() {
        for i in (u8::MAX as u16)..=u16::MAX {
            let s = i.to_string();
            let v: u16 = intify_str_unsigned!(&s);
            assert_eq!(i, v);
        }
    }

    #[test]
    fn round_trip_all_i8s() {
        for i in i8::MIN..=i8::MAX {
            let s = i.to_string();
            let v: i8 = intify_str_signed!(&s);
            assert_eq!(i, v);
        }
    }

    #[test]
    fn all_signed_mins() {
        let s8 = i8::MIN.to_string();
        assert_eq!(i8::MIN, intify_str_signed!(s8));
        let s16 = i16::MIN.to_string();
        assert_eq!(i16::MIN, intify_str_signed!(s16));
        let s32 = i32::MIN.to_string();
        assert_eq!(i32::MIN, intify_str_signed!(s32));
        let s64 = i64::MIN.to_string();
        assert_eq!(i64::MIN, intify_str_signed!(s64));
        let s128 = i128::MIN.to_string();
        assert_eq!(i128::MIN, intify_str_signed!(s128));
    }

    #[test]
    fn round_trip_all_i16s() {
        for i in i16::MIN..=i16::MAX {
            let s = i.to_string();
            let v: i16 = intify_str_signed!(&s);
            assert_eq!(i, v);
        }
    }

    proptest! {
        #[test]
        fn round_trip_u32s(i in 0u32..=u32::MAX) {
            let s = i.to_string();
            let v: u32 = intify_str_unsigned!(&s);
            assert_eq!(i, v);
        }
    }

    proptest! {
        #[test]
        fn round_trip_i32s(i in i32::MIN..=i32::MAX) {
            let s = i.to_string();
            let v: i32 = intify_str_signed!(&s);
            assert_eq!(i, v);
        }
    }

    proptest! {
        #[test]
        fn round_trip_u128s(i in u128::MIN..=u128::MAX) {
            let s = i.to_string();
            let v: u128 = intify_str_unsigned!(&s);
            assert_eq!(i, v);
        }
    }

    proptest! {
        #[test]
        fn round_trip_i128s(i in i128::MIN..=i128::MAX) {
            let s = i.to_string();
            let v: i128 = intify_str_signed!(&s);
            assert_eq!(i, v);
        }
    }
}


/// Handle the raw number portion of the provided string, this assumes that the caller is providing
/// a bytes slice instead of a &str
#[macro_export]
macro_rules! handle_numbers {
    ($bytes:expr) => {{
        let mut idx = 0;
        let mut out = 0;
        // for each char in the string
        while idx < $bytes.len() {
            let c = $bytes[idx];
            // convert the char to an integer
            let value = match c {
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
            };
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

/// Const conversion of integer strings to any unsigned integer type
#[macro_export]
macro_rules! intify_str_unsigned {
    ($s:expr) => {{
        let bytes = $s.as_bytes();
        $crate::handle_numbers!(bytes)
    }};
}

/// Const conversion of integer strings to any signed integer type
/// 
/// note: Because the signed integer types have a larger negative value than a positive one, this
/// will fail if providing the minimum value is provided. For example 
/// 
/// ```rust,no_run
/// let value: i8 = intify_str_signed("-128");
/// //
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
        let ret = $crate::handle_numbers!(slice);
        if negate {
            -ret
        } else {
            ret
        }
    }};
}

#[cfg(test)]
mod tests {
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
        for i in i8::MIN+1..=i8::MAX {
            let s = i.to_string();
            let v: i8 = intify_str_signed!(&s);
            assert_eq!(i, v);
        }
    }

    #[test]
    fn round_trip_all_i16s() {
        for i in i16::MIN+1..=i16::MAX {
            let s = i.to_string();
            println!("attempting {i}");
            let v: i16 = intify_str_signed!(&s);
            assert_eq!(i, v);
        }
    }
}

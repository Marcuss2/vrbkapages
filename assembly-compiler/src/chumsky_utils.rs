use chumsky::prelude::*;
use chumsky::text::digits;
use num_traits::{Num, PrimInt, WrappingAdd};
use std::fmt::{Debug, Display};
use std::num::NonZeroU32;
use std::str::FromStr;

fn parse_raw_hex_number<T>(s: &str) -> T
where
    T: Num + PrimInt + WrappingAdd,
{
    let sign = s.starts_with('-');
    let mut result = T::zero();
    let start = 2 + sign as usize;
    for c in s[start..].chars() {
        let digit = match c {
            '0'..='9' => c as u8 - b'0',
            'a'..='f' => c as u8 - b'a' + 10,
            'A'..='F' => c as u8 - b'A' + 10,
            _ => {
                panic!("Invalid hex digit: {}", c);
            }
        };
        result = result.unsigned_shl(4) + T::from(digit).unwrap();
    }
    if sign && result != T::min_value() {
        (!result).wrapping_add(&T::one())
    } else {
        result
    }
}

fn parse_raw_bin_number<T>(s: &str) -> T
where
    T: Num + PrimInt + WrappingAdd,
{
    let sign = s.starts_with('-');
    let mut result = T::zero();
    let start = 2 + sign as usize;
    for c in s[start..].chars() {
        let digit = match c {
            '0' => T::zero(),
            '1' => T::one(),
            _ => {
                panic!("Invalid bin digit: {}", c);
            }
        };
        result = result.unsigned_shl(1) + digit;
    }
    if sign && result != T::min_value() {
        (!result).wrapping_add(&T::one())
    } else {
        result
    }
}

fn hex_number<'src, T>(
    signed: bool,
) -> impl Parser<'src, &'src str, T, extra::Err<Rich<'src, char>>>
where
    <T as Num>::FromStrRadixErr: Debug,
    T: num_traits::Num + std::fmt::Debug + PrimInt + num_traits::WrappingAdd,
    <T as Num>::FromStrRadixErr: std::fmt::Display,
{
    just('-')
        .or_not()
        .then(
            just("0x").ignore_then(
                digits(16)
                    .at_least(1)
                    .at_most((((T::max_value().count_ones() + 7) / 8) * 2) as usize),
            ),
        )
        .to_slice()
        .try_map(move |s: &str, span| {
            if !signed && s.starts_with('-') {
                return Err(Rich::custom(
                    span,
                    format!("Sign not allowed for unsigned number."),
                ));
            }
            Ok(parse_raw_hex_number(s))
        })
}

fn bin_number<'src, T>(
    signed: bool,
) -> impl Parser<'src, &'src str, T, extra::Err<Rich<'src, char>>>
where
    <T as Num>::FromStrRadixErr: Debug,
    T: Num + PrimInt + WrappingAdd,
    <T as Num>::FromStrRadixErr: Display,
{
    just('-')
        .or_not()
        .then(
            just("0b").ignore_then(
                digits(2)
                    .at_least(1)
                    .at_most((((T::max_value().count_ones() + 7) / 8) * 8) as usize),
            ),
        )
        .to_slice()
        .try_map(move |s: &str, span| {
            if !signed && s.starts_with('-') {
                return Err(Rich::custom(
                    span,
                    format!("Sign not allowed for unsigned number."),
                ));
            }
            Ok(parse_raw_bin_number(s))
        })
}

fn dec_number<'src, T>() -> impl Parser<'src, &'src str, T, extra::Err<Rich<'src, char>>>
where
    T: PrimInt,
    <T as Num>::FromStrRadixErr: Debug,
    <T as Num>::FromStrRadixErr: std::fmt::Display,
{
    just('-')
        .or_not()
        .then(digits(10).at_least(1))
        .to_slice()
        .try_map(|s: &str, span| T::from_str_radix(s, 10).map_err(|e| Rich::custom(span, e)))
}

// .count_ones() on signed integer returns 1 less than the number
// of bits and we need one extra bit for the sign as well, hence two. Yes, it sounds weird.
fn needed_bits_for_number<T: PrimInt>(num: T, signed: bool) -> u32 {
    if num < T::zero() {
        T::max_value().count_ones() + if signed { 2 } else { 0 } - num.leading_ones()
    } else {
        T::max_value().count_ones() + if signed { 2 } else { 0 } - num.leading_zeros()
    }
}

pub fn integer<'src, T>(
    bits: NonZeroU32,
    signed: bool,
) -> impl Parser<'src, &'src str, T, extra::Err<Rich<'src, char>>>
where
    T: FromStr + PrimInt + Display + std::fmt::Debug + num_traits::WrappingAdd,
    <T as Num>::FromStrRadixErr: Debug,
    <T as Num>::FromStrRadixErr: std::fmt::Display,
{
    choice((
        hex_number::<T>(signed),
        bin_number::<T>(signed),
        dec_number::<T>(),
    ))
    .try_map(move |num, span| {
        if !signed && num < T::zero() {
            return Err(Rich::custom(
                span,
                format!("Sign not allowed for unsigned number."),
            ));
        }
        if num == T::zero() {
            return Ok(num);
        }
        let needed_bits = needed_bits_for_number(num, signed);
        if needed_bits > bits.into() {
            return Err(Rich::custom(
                span,
                format!(
                    "Number out of range for {}-bit {} integer, needs {} bits",
                    bits,
                    if signed { "signed" } else { "unsigned" },
                    needed_bits
                ),
            ));
        }
        Ok(num)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_parses_success<'a, T: std::fmt::Debug + PartialEq>(
        parser: impl Parser<'a, &'a str, T, extra::Err<Rich<'a, char>>>,
        input: &'a str,
        expected: T,
    ) {
        let result = parser.parse(input);
        for err in result.errors() {
            println!("{}", err);
        }
        assert_eq!(result.has_errors(), false);
        assert_eq!(result.unwrap(), expected);
    }

    fn assert_parses_failure<'a, T: std::fmt::Debug>(
        parser: impl Parser<'a, &'a str, T, extra::Err<Rich<'a, char>>>,
        input: &'a str,
    ) {
        assert_eq!(parser.parse(input).has_errors(), true);
    }

    #[test]
    fn test_unsigned_8bit_hex_ff() {
        let parser = integer::<u8>(8.try_into().unwrap(), false);
        assert_parses_success(parser, "0xFF", 255u8);
    }

    #[test]
    fn test_unsigned_8bit_hex_0f() {
        let parser = integer::<u8>(8.try_into().unwrap(), false);
        assert_parses_success(parser, "0x0F", 15u8);
    }

    #[test]
    fn test_unsigned_8bit_bin_1111() {
        let parser = integer::<u8>(8.try_into().unwrap(), false);
        assert_parses_success(parser, "0b1111", 15u8);
    }

    #[test]
    fn test_unsigned_8bit_bin_0101() {
        let parser = integer::<u8>(8.try_into().unwrap(), false);
        assert_parses_success(parser, "0b0101", 5u8);
    }

    #[test]
    fn test_unsigned_8bit_dec_123() {
        let parser = integer::<u8>(8.try_into().unwrap(), false);
        assert_parses_success(parser, "123", 123u8);
    }

    #[test]
    fn test_unsigned_8bit_dec_42() {
        let parser = integer::<u8>(8.try_into().unwrap(), false);
        assert_parses_success(parser, "42", 42u8);
    }

    #[test]
    fn test_signed_8bit_hex_7f() {
        let parser = integer::<i8>(8.try_into().unwrap(), true);
        assert_parses_success(parser, "0x7F", 127i8);
    }

    #[test]
    fn test_signed_8bit_bin_1111() {
        let parser = integer::<i8>(8.try_into().unwrap(), true);
        assert_parses_success(parser, "0b1111", 15i8);
    }

    #[test]
    fn test_signed_8bit_dec_123() {
        let parser = integer::<i8>(8.try_into().unwrap(), true);
        assert_parses_success(parser, "123", 123i8);
    }

    #[test]
    fn test_signed_8bit_dec_neg_123() {
        let parser = integer::<i8>(8.try_into().unwrap(), true);
        assert_parses_success(parser, "-123", -123i8);
    }

    #[test]
    fn test_unsigned_16bit_hex_ffff() {
        let parser = integer::<u16>(16.try_into().unwrap(), false);
        assert_parses_success(parser, "0xFFFF", 65535u16);
    }

    #[test]
    fn test_unsigned_16bit_hex_1234() {
        let parser = integer::<u16>(16.try_into().unwrap(), false);
        assert_parses_success(parser, "0x1234", 0x1234u16);
    }

    #[test]
    fn test_unsigned_16bit_bin_max() {
        let parser = integer::<u16>(16.try_into().unwrap(), false);
        assert_parses_success(parser, "0b1111111111111111", 65535u16);
    }

    #[test]
    fn test_unsigned_16bit_dec_max() {
        let parser = integer::<u16>(16.try_into().unwrap(), false);
        assert_parses_success(parser, "65535", 65535u16);
    }

    #[test]
    fn test_signed_16bit_hex_7fff() {
        let parser = integer::<i16>(16.try_into().unwrap(), true);
        assert_parses_success(parser, "0x7FFF", 32767i16);
    }

    #[test]
    fn test_signed_16bit_dec_min() {
        let parser = integer::<i16>(16.try_into().unwrap(), true);
        assert_parses_success(parser, "-32768", -32768i16);
    }

    #[test]
    fn test_signed_16bit_bin_max() {
        let parser = integer::<i16>(16.try_into().unwrap(), true);
        assert_parses_success(parser, "0b0111111111111111", 32767i16);
    }

    #[test]
    fn test_signed_16bit_hex_min() {
        let parser = integer::<i16>(16.try_into().unwrap(), true);
        assert_parses_success(parser, "-0x8000", -0x8000i16); // sign is not allowed in hex numbers
    }

    #[test]
    fn test_unsigned_32bit_hex_max() {
        let parser = integer::<u32>(32.try_into().unwrap(), false);
        assert_parses_success(parser, "0xFFFFFFFF", 4294967295u32);
    }

    #[test]
    fn test_unsigned_32bit_hex_12345678() {
        let parser = integer::<u32>(32.try_into().unwrap(), false);
        assert_parses_success(parser, "0x12345678", 0x12345678u32);
    }

    #[test]
    fn test_unsigned_32bit_dec_max() {
        let parser = integer::<u32>(32.try_into().unwrap(), false);
        assert_parses_success(parser, "4294967295", 4294967295u32);
    }

    #[test]
    fn test_signed_32bit_hex_max() {
        let parser = integer::<i32>(32.try_into().unwrap(), true);
        assert_parses_success(parser, "0x7FFFFFFF", 2147483647i32);
    }

    #[test]
    fn test_signed_32bit_dec_min() {
        let parser = integer::<i32>(32.try_into().unwrap(), true);
        assert_parses_success(parser, "-2147483648", -2147483648i32);
    }

    #[test]
    fn test_signed_32bit_hex_min() {
        let parser = integer::<i32>(32.try_into().unwrap(), true);
        assert_parses_success(parser, "-0x80000000", -0x80000000);
    }

    #[test]
    fn test_number_range_signed_8bit_sign_bit() {
        let parser = integer::<i8>(8.try_into().unwrap(), true);
        assert_parses_success(parser, "0xFF", -1);
    }

    #[test]
    fn test_number_range_signed_8bit_too_large_dec() {
        let parser = integer::<i8>(8.try_into().unwrap(), true);
        assert_parses_failure(parser, "128"); // 128 is too large for 8-bit signed
    }

    #[test]
    fn test_number_range_signed_8bit_too_small() {
        let parser = integer::<i8>(8.try_into().unwrap(), true);
        assert_parses_failure(parser, "-129"); // -129 is too small for 8-bit signed
    }

    #[test]
    fn test_number_range_unsigned_8bit_too_large() {
        let parser = integer::<i8>(8.try_into().unwrap(), true);
        assert_parses_failure(parser, "256"); // 256 is too large for 8-bit unsigned
    }

    #[test]
    fn test_number_range_signed_16bit_sign_bit() {
        let parser = integer::<i16>(16.try_into().unwrap(), true);
        assert_parses_success(parser, "0x8000", -32768i16);
    }

    #[test]
    fn test_number_range_unsigned_16bit_too_large() {
        let parser = integer::<i16>(16.try_into().unwrap(), false);
        assert_parses_failure(parser, "0x10000"); // 65536 is too large for 16-bit unsigned
    }

    #[test]
    fn test_number_range_signed_16bit_too_small() {
        let parser = integer::<i16>(16.try_into().unwrap(), true);
        assert_parses_failure(parser, "-32769"); // -32769 is too small for 16-bit signed
    }

    #[test]
    fn test_unsigned_8bit_4bit_max() {
        let parser = integer::<u8>(4.try_into().unwrap(), false);
        assert_parses_success(parser, "15", 15u8);
    }

    #[test]
    fn test_unsigned_8bit_4bit_too_large() {
        let parser = integer::<u8>(4.try_into().unwrap(), false);
        assert_parses_failure(parser, "16");
    }

    #[test]
    fn test_unsigned_8bit_7bit_max() {
        let parser = integer::<u8>(7.try_into().unwrap(), false);
        assert_parses_success(parser, "127", 127u8);
    }

    #[test]
    fn test_unsigned_8bit_7bit_too_large() {
        let parser = integer::<u8>(7.try_into().unwrap(), false);
        assert_parses_failure(parser, "128");
    }

    #[test]
    fn test_signed_16bit_12bit_max() {
        let parser = integer::<i16>(12.try_into().unwrap(), true);
        assert_parses_success(parser, "2047", 2047i16);
    }

    #[test]
    fn test_signed_16bit_12bit_min() {
        let parser = integer::<i16>(12.try_into().unwrap(), true);
        assert_parses_success(parser, "-2048", -2048i16);
    }

    #[test]
    fn test_signed_16bit_12bit_too_large() {
        let parser = integer::<i16>(12.try_into().unwrap(), true);
        assert_parses_failure(parser, "2048");
    }

    #[test]
    fn test_signed_32bit_24bit_max() {
        let parser = integer::<i32>(24.try_into().unwrap(), true);
        assert_parses_success(parser, "8388607", 8388607i32);
    }

    #[test]
    fn test_signed_32bit_24bit_min() {
        let parser = integer::<i32>(24.try_into().unwrap(), true);
        assert_parses_success(parser, "-8388608", -8388608i32);
    }

    #[test]
    fn test_signed_32bit_24bit_too_large() {
        let parser = integer::<i32>(24.try_into().unwrap(), true);
        assert_parses_failure(parser, "8388608");
    }

    #[test]
    fn test_signed_32bit_20bit_signed() {
        let parser = integer::<i32>(20.try_into().unwrap(), true);
        assert_parses_success(parser, "-0x5678", -0x5678);
    }
}

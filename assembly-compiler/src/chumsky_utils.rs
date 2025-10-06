use chumsky::prelude::*;
use chumsky::text::digits;
use num_traits::Num;
use num_traits::bounds::LowerBounded;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::RangeInclusive;

#[derive(Debug)]
pub enum IntParsingError{
    FromStrRadixError,
    NegativeUnsignedError,
}

impl Display for IntParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntParsingError::FromStrRadixError => {
                write!(f, "Failed to parse integer due to wrong radix bytes")
            }
            IntParsingError::NegativeUnsignedError => {
                write!(f, "Cannot parse negative number as unsigned integer")
            }
        }
    }
}

impl Error for IntParsingError {}

// Taken from parse_int
#[inline]
pub fn parse_int<T: Num + LowerBounded + PartialOrd>(input: &str) -> Result<T, IntParsingError> {
    let input = input.trim();

    let is_signed = T::min_value() < T::zero();
    let (is_negative, input) = if let Some(input) = input.strip_prefix('-') {
        (true, input)
    } else {
        (false, input)
    };

    if is_negative && !is_signed {
        return Err(IntParsingError::NegativeUnsignedError);
    }

    if input.starts_with("_") {
        return T::from_str_radix("_", 2).map_err(|_| IntParsingError::FromStrRadixError);
    }

    let num: T =
    // hex
    if input.starts_with("0x") || input.starts_with("0X") {
        parse_with_base(&input[2..], 16).map_err(|_| IntParsingError::FromStrRadixError)?
    } else
    // binary
    if input.starts_with("0b") || input.starts_with("0B") {
        parse_with_base(&input[2..], 2).map_err(|_| IntParsingError::FromStrRadixError)?
    } else
    // octal
    if input.starts_with("0o") || input.starts_with("0O") {
        parse_with_base(&input[2..], 8).map_err(|_| IntParsingError::FromStrRadixError)?
    } else {
        // decimal
        parse_with_base(input, 10).map_err(|_| IntParsingError::FromStrRadixError)?
    };

    Ok(if is_negative {
        num * (T::zero() - T::one())
    } else {
        num
    })
}

#[inline]
fn parse_with_base<T: Num>(input: &str, base: u32) -> Result<T, T::FromStrRadixErr> {
    let input = input.chars().filter(|&c| c != '_').collect::<String>();
    T::from_str_radix(&input, base)
}

pub fn number<'src, Idx: Num + Display + PartialOrd + LowerBounded>(
    range: &RangeInclusive<Idx>,
) -> impl Parser<'src, &'src str, Idx, extra::Err<Rich<'src, char>>> {
    just("-")
        .or_not()
        .then(choice([
            just("0x"),
            just("0X"),
            just("0b"),
            just("0B"),
            just("0o"),
            just("0O"),
            just(""),
        ]))
        .then(digits(16).at_least(1))
        .to_slice()
        .try_map(move |slice, span| match parse_int::<Idx>(slice) {
            Ok(num) if range.contains(&num) => Ok(num),
            _ => {
                return Err(Rich::custom(
                    span,
                    format!(
                        "Invalid number, must be in range of: {} to {}",
                        range.start(),
                        range.end()
                    ),
                ));
            }
        })
}

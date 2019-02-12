use nom::IResult;
use nom::types::CompleteStr;

use crate::lexer::Token;
use ast::Integer;

#[derive(PartialEq, Eq)]
enum Base {
    Binary,
    Octal,
    Decimal,
    Hex
}

impl Base {
    pub fn radix(&self) -> u32 {
        match self {
            Base::Binary => 2,
            Base::Octal => 8,
            Base::Decimal => 10,
            Base::Hex => 16
        }
    }

    pub fn prefix(&self) -> &'static str {
        match self {
            Base::Binary => "0b",
            Base::Octal => "0",
            Base::Decimal => "",
            Base::Hex => "0x"
        }
    }
}

#[derive(PartialEq, Eq)]
enum ConstIntSize {
    Unspecified,
    Long,
    LongLong
}

#[derive(PartialEq, Eq)]
enum ConstIntSign {
    Unspecified,
    Unsigned
}

pub fn integer_literal(input: CompleteStr) -> IResult<CompleteStr, Token, u32> {
    let (input, (literal, base)) = literal(input)?;
    let (input, (sign, size)) = integer_suffix(input)?;
    let sign = match sign {
        Some(_) => ConstIntSign::Unsigned,
        None => ConstIntSign::Unspecified
    };
    let size = match size.map(|v| v.0) {
        Some("ll") | Some("LL") => ConstIntSize::LongLong,
        Some("l") | Some("L") => ConstIntSize::Long,
        _ => ConstIntSize::Unspecified
    };
    let value = convert(&literal, base, sign, size);
    Ok((input, Token::IntLiteral(value)))
}

/// Converts a hex, decimal, octal, or binary integer literal with an optional unsigned specifier and
/// optional size specifier to a concrete integer type, following the rules in the c99 standard.
///
/// Rules:
///
///  1. The valid integer types are signed or unsigned ints, long ints, and long long ints.
///
///  2. Decimal int literals must be treated as being signed, unless they have a sign specifier.
///
///  3. If the literal has a long size specifier ("l"), it must either be a long int or a long long int.
///
///  4. If the literal has a long long size specifier ("ll"), it must be a long long int.
///
///  5. The type of the integer is the smallest type that can hold the value while following the previous 4 rules.
///
/// Refer to section 6.4.4.1 of ISO/IEC 9899:1999 for a helpful chart.
/// 
/// The current implementation of this relies on the fact that invalid literals are rejected before being passed to this function,
/// and the fact that the standard library from_str_radix functions will return an error on overflow and underflow.
fn convert<'a>(value: &'a str, base: Base, sign: ConstIntSign, size: ConstIntSize) -> Integer {
    if sign != ConstIntSign::Unsigned && size == ConstIntSize::Unspecified {
        if let Ok(v) = i32::from_str_radix(value, base.radix()) {
            return Integer::I32(v);
        }
    }
    if (base != Base::Decimal || sign == ConstIntSign::Unsigned) && size == ConstIntSize::Unspecified {
        if let Ok(v) = u32::from_str_radix(value, base.radix()) {
            return Integer::U32(v);
        }
    }
    if sign != ConstIntSign::Unsigned && size != ConstIntSize::LongLong {
        if let Ok(v) = i64::from_str_radix(value, base.radix()) {
            return Integer::I64(v);
        }
    }
    if (base != Base::Decimal || sign == ConstIntSign::Unsigned) && size != ConstIntSize::LongLong {
        if let Ok(v) = u64::from_str_radix(value, base.radix()) {
            return Integer::U64(v);
        }
    }
    if sign != ConstIntSign::Unsigned {
        if let Ok(v) = i128::from_str_radix(value, base.radix()) {
            return Integer::I128(v);
        }
    }
    if base != Base::Decimal || sign == ConstIntSign::Unsigned {
        if let Ok(v) = u128::from_str_radix(value, base.radix()) {
            return Integer::U128(v);
        }
    }

    // the value is too large to fit into any of the available types
    // if it is a decimal int, truncate to a i128, otherwise truncate to a u128
    // TODO: better warning (with source name & line:col numbers) & propogate warning up instead of printing to console here
    println!("warning: constant integer literal {}{} truncated", base.prefix(), value);

    if base == Base::Decimal {
        Integer::I128(i128::max_value())
    }
    else {
        Integer::U128(u128::max_value())
    }
}

named!(literal(CompleteStr) -> (CompleteStr, Base), alt!(
    hex_integer_literal | octal_integer_literal |
    binary_integer_literal | decimal_integer_literal
));

named!(hex_integer_literal(CompleteStr) -> (CompleteStr, Base), do_parse!(
    alt!(tag!("0x") | tag!("0X")) >>
    literal: take_while1!(|c: char| c.is_digit(16)) >>
    (literal, Base::Hex)
));

named!(decimal_integer_literal(CompleteStr) -> (CompleteStr, Base), do_parse!(
    literal: take_while1!(|c: char| c.is_digit(10)) >>
    ((literal, Base::Decimal))
));

named!(octal_integer_literal(CompleteStr) -> (CompleteStr, Base), do_parse!(
    char!('0') >>
    literal: take_while1!(|c: char| c.is_digit(8)) >>
    (literal, Base::Octal)
));

// not part of c99 (or c11 for that matter),
// but accepted by gcc as an extension
named!(binary_integer_literal(CompleteStr) -> (CompleteStr, Base), do_parse!(
    alt!(tag!("0b") | tag!("0B")) >>
    literal: take_while1!(|c: char| c.is_digit(2)) >>
    (literal, Base::Binary)
));

/// The first element in the returned tuple is the unsigned_suffix, and the
/// second element is the size suffix (long or long long).
named!(integer_suffix(CompleteStr) -> (Option<CompleteStr>,Option<CompleteStr>), alt!(
    // sign before size
    do_parse!(
        sign: opt!(unsigned_suffix) >>
        size: opt!(alt!(long_long_suffix | long_suffix)) >>
        (sign, size)
    ) |
    // size before sign
    do_parse!(
        size: opt!(alt!(long_long_suffix | long_suffix)) >>
        sign: opt!(unsigned_suffix) >>
        (sign, size)
    )
));

named!(unsigned_suffix(CompleteStr) -> CompleteStr, alt!(tag!("u") | tag!("U")));
named!(long_suffix(CompleteStr) -> CompleteStr, alt!(tag!("l") | tag!("L")));
named!(long_long_suffix(CompleteStr) -> CompleteStr, alt!(tag!("ll") | tag!("LL")));

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::Lexer;
    use ast::Integer;

    #[test]
    fn small_decimal_integer_literal() {
        let mut input = Lexer::new("test.c".into(), "254");
        let token = input.next().unwrap().unwrap().1;
        assert_eq!(token, Token::IntLiteral(Integer::I32(254)));
        assert_eq!(input.next(), None);
    }

    #[test]
    fn big_decimal_integer_literal() {
        let mut input = Lexer::new("test.c".into(), "1000000000000000"); // too big to fit into i32
        let token = input.next().unwrap().unwrap().1;
        assert_eq!(token, Token::IntLiteral(Integer::I64(1000000000000000)));
        assert_eq!(input.next(), None);
    }

    #[test]
    fn unsigned_long_decimal_integer_literal() {
        let mut input = Lexer::new("test.c".into(), "17ul");
        let token = input.next().unwrap().unwrap().1;
        assert_eq!(token, Token::IntLiteral(Integer::U64(17)));
        assert_eq!(input.next(), None);
    }

    #[test]
    fn decimal_u32_max() {
        // this is i32::max_value() + 1, and since it as a decimal int without the unsigned specifier,
        // it turns into an i64 (long int).
        let source = "2147483648"; 
        let mut input = Lexer::new("test.c".into(), source);
        let token = input.next().unwrap().unwrap().1;
        assert_eq!(token, Token::IntLiteral(Integer::I64(2147483648)));
        assert_eq!(input.next(), None);
    }

    #[test]
    fn decimal_u32_max_unsigned_specifier() {
        // this is i32::max_value() + 1, since we specify that it is unsigned, the concrete type should be u32.
        let source = "2147483648u"; 
        let mut input = Lexer::new("test.c".into(), source);
        let token = input.next().unwrap().unwrap().1;
        assert_eq!(token, Token::IntLiteral(Integer::U32(2147483648)));
        assert_eq!(input.next(), None);
    }

    fn long_long_decimal_integer_literal() {
        let source = "22ll";
        let mut input = Lexer::new("test.c".into(), source);
        let token = input.next().unwrap().unwrap().1;
        assert_eq!(token, Token::IntLiteral(Integer::I128(22)));
        assert_eq!(input.next(), None);
    }

    fn unsigned_long_long_decimal_integer_literal() {
        let source = "22llu"; // order of sign and size specifiers don't matter
        let mut input = Lexer::new("test.c".into(), source);
        let token = input.next().unwrap().unwrap().1;
        assert_eq!(token, Token::IntLiteral(Integer::U128(22)));
        assert_eq!(input.next(), None);
    }

    #[test]
    fn small_hex_literal() {
        let source = "0x100";
        let mut input = Lexer::new("test.c".into(), source);
        let token = input.next().unwrap().unwrap().1;
        assert_eq!(token, Token::IntLiteral(Integer::I32(0x100)));
        assert_eq!(input.next(), None);
    }

    #[test]
    fn small_octal_literal() {
        let source = "014";
        let mut input = Lexer::new("test.c".into(), source);
        let token = input.next().unwrap().unwrap().1;
        assert_eq!(token, Token::IntLiteral(Integer::I32(12)));
        assert_eq!(input.next(), None);
    }

    #[test]
    fn small_binary_literal() {
        let source = "0b11110000"; // 0xF0
        let mut input = Lexer::new("test.c".into(),source);
        let token = input.next().unwrap().unwrap().1;
        assert_eq!(token, Token::IntLiteral(Integer::I32(0xF0)));
        assert_eq!(input.next(), None);
    }
}
use crate::combinators::many;
use crate::error::ParserError;
use crate::parser::Parser;
pub fn take_digit() -> Parser<char, String> {
    Box::new(|s| {
        if s.len() == 0 {
            return Err(ParserError::new(
                s,
                "Expected a digit, found nothing".to_string(),
            ));
        }
        let mut chars = s.chars();
        let next = chars.next().unwrap();
        match next {
            '0'..='9' => Ok((chars.collect(), next)),
            _ => Err(ParserError::new(
                s,
                format!("Expected a digit, found: `{}`", next),
            )),
        }
    })
}

#[allow(dead_code)]
pub fn take_alpha() -> Parser<char, String> {
    Box::new(|s| {
        if s.len() == 0 {
            return Err(ParserError::new(
                s,
                "Expected an alphabetic char, found nothing".to_string(),
            ));
        }
        let mut chars = s.chars();
        let next = chars.next().unwrap();
        match next {
            'A'..='z' => Ok((chars.collect(), next)),
            _ => Err(ParserError::new(
                s,
                format!("Expected an alphabetic char, found: `{}`", next),
            )),
        }
    })
}
pub fn take_char(c: char) -> Parser<char, String> {
    Box::new(move |s| {
        if s.len() == 0 {
            return Err(ParserError::new(
                s,
                format!("Expected `{}`, found nothing", c),
            ));
        }
        let mut chars = s.chars();
        let next = chars.next().unwrap();
        if next == c {
            Ok((chars.collect(), next))
        } else {
            Err(ParserError::new(
                s,
                format!("Expected `{}`, found `{}`", c, next),
            ))
        }
    })
}
pub fn take_cpredicate(predicate: Box<dyn Fn(char) -> bool>) -> Parser<char, String> {
    Box::new(move |s| {
        if s.len() == 0 {
            return Err(ParserError::new_no_reason(s));
        }
        let mut chars = s.chars();
        let next = chars.next().unwrap();
        if predicate(next) {
            Ok((chars.collect(), next))
        } else {
            Err(ParserError::new_no_reason(s))
        }
    })
}
pub fn take_whitespaces() -> Parser<Vec<char>, String> {
    Box::new(|s| many(take_cpredicate(Box::new(|c: char| c.is_whitespace())))(s))
}
pub fn take_str(str_to_match: String) -> Parser<String, String> {
    Box::new(move |s| {
        if str_to_match.len() > s.len() {
            return Err(ParserError::new(
                s.clone(),
                format!("Expected `{}` found `{}`", str_to_match, s),
            ));
        }
        let mut schars = s.chars();
        let chars_to_match = str_to_match.chars();
        for i in chars_to_match.into_iter() {
            if i != schars.next().unwrap() {
                return Err(ParserError::new(
                    s.clone(),
                    format!(
                        "Expected `{}` found `{}`",
                        str_to_match,
                        &s[..str_to_match.len() - 1]
                    ),
                ));
            }
        }
        Ok((schars.collect(), str_to_match.to_string()))
    })
}

pub fn take_one_of(strings: Vec<&'static str>) -> Parser<String, String> {
    Box::new(move |s| {
        let mut remaining = s;
        for i in strings.iter() {
            match take_str(i.to_string())(remaining) {
                Ok((remaining, matched)) => return Ok((remaining, matched)),
                Err(s) => remaining = s.remaining(),
            }
        }
        Err(ParserError::new(
            remaining,
            format!("Expected one of the {:#?}", strings),
        ))
    })
}

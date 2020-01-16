use crate::parser::Parser;

pub fn many<T: 'static>(a: Parser<T, String>) -> Parser<Vec<T>, String> {
    Box::new(move |s| {
        let mut result = vec![];
        let mut remaining = s.clone();
        while let Ok((rem, v)) = a(remaining.clone()) {
            result.push(v);
            remaining = rem;
        }
        Ok((remaining, result))
    })
}
pub fn many1<T: 'static>(predicate: Parser<T, String>) -> Parser<Vec<T>, String> {
    Box::new(move |s| {
        let mut values = vec![];
        match predicate(s.clone()) {
            Ok((mut remaining, val)) => {
                values.push(val);
                while let Ok((rem, val)) = predicate(remaining.clone()) {
                    values.push(val);
                    remaining = rem;
                }
                Ok((remaining, values))
            }
            Err(e) => Err(e),
        }
    })
}

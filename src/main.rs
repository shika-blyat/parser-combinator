fn pchar<'a>(val: &'a str, c: char) -> Result<String, String> {
    let mut iter = val.chars();
    if val.is_empty() {
        return Err(val.to_string());
    }
    let first_char = iter.nth(0).unwrap();
    if first_char == c {
        Ok(iter.collect())
    } else {
        Err(val.to_string())
    }
}
trait AnyOf: IntoIterator + Sized {
    fn any_of<X, T, B: (FnMut(&Self::Item) -> Result<T, X>)>(
        self,
        mut predicate: B,
    ) -> Result<Result<T, X>, String> {
        let mut iter = self.into_iter();
        let mut last_seen = match iter.next() {
            Some(x) => x,
            None => return Err("empty".to_string()),
        };
        for i in iter {
            if let Ok(t) = predicate(&last_seen) {
                return Ok(Ok(t));
            }
            last_seen = i;
        }
        Ok(predicate(&last_seen))
    }
}

impl<X, T: IntoIterator<Item = X>> AnyOf for T {}
fn main() {
    let list = ["15", "XBC", "AYZ"].as_ref();
    println!("{:#?}", list.any_of(|x| pchar(x, 'A')).unwrap());
    println!("{:#?}", list.any_of(|x| pchar(x, 'X')).unwrap());
    println!("{:#?}", [].any_of(|&&x| pchar(x, 'X')));
}

/*
println!(
    "{:#?}",
    pchar("ABC", 'A').and_then(|x| pchar(x.as_str(), 'B'))
);
println!("{:#?}", pchar("KBC", 'A').or_else(|_| pchar("BBC", 'B')));*/

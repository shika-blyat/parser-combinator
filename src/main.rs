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
trait AnyOf {
    type X;
    fn any_of<T, E>(self, predicate: impl FnMut(&Self::X) -> Result<T, E>) -> Option<Result<T, E>>;
}

impl<I: IntoIterator> AnyOf for I {
    type X = I::Item;

    fn any_of<T, E>(
        self,
        mut predicate: impl FnMut(&Self::X) -> Result<T, E>,
    ) -> Option<Result<T, E>> {
        let mut last_err = None;

        for i in self.into_iter() {
            match predicate(&i) {
                x @ Ok(_) => return Some(x),
                e @ Err(_) => last_err = Some(e),
            }
        }

        last_err
    }
}
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

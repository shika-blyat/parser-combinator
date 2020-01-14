#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
    remaining: String,
    reason: Option<String>,
}
#[allow(dead_code)]
impl ParserError {
    pub fn new_no_reason(remaining: String) -> Self {
        Self {
            remaining: remaining,
            reason: None,
        }
    }
    pub fn new_no_rem(reason: String) -> Self {
        let empty = "".to_string();
        Self {
            reason: Some(reason),
            remaining: empty,
        }
    }
    pub fn new(remaining: String, reason: String) -> Self {
        Self {
            reason: Some(reason),
            remaining: remaining,
        }
    }
    pub fn remaining(&self) -> String {
        self.remaining.clone()
    }
}

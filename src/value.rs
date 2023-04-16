use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Int(i64),
    Float(f64),
    Str(Cow<'a, str>),
}

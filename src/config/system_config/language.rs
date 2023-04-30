use num_derive::FromPrimitive;
use std::fmt::{Display, Formatter, Result};
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, FromPrimitive)]
pub(crate) enum Language {
    Ja,
    En,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            Language::Ja => write!(f, "Japanese"),
            Language::En => write!(f, "English"),
        }
    }
}

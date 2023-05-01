use crate::config::Select;
use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter, Result};
use strum_macros::EnumIter;

#[derive(EnumIter, FromPrimitive, ToPrimitive, Default)]
pub(crate) enum Language {
    En,
    #[default]
    Ja,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            Language::En => write!(f, "English"),
            Language::Ja => write!(f, "Japanese"),
        }
    }
}

impl Select for Language {}

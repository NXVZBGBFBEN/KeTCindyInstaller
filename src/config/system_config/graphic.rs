use crate::config::Select;
use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter, Result};
use strum_macros::EnumIter;

#[derive(EnumIter, FromPrimitive, ToPrimitive, Default)]
pub(crate) enum Graphic {
    #[default]
    Pict2e,
    Tpic,
    Tikz,
}

impl Display for Graphic {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            Graphic::Pict2e => write!(f, "pict2e"),
            Graphic::Tpic => write!(f, "Tpic"),
            Graphic::Tikz => write!(f, "TikZ"),
        }
    }
}

impl Select for Graphic {}

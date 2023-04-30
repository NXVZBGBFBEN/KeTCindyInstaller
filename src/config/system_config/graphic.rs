use num_derive::FromPrimitive;
use std::fmt::{Display, Formatter, Result};
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, FromPrimitive)]
pub(crate) enum Graphic {
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

use num_derive::FromPrimitive;
use std::fmt::{Display, Formatter, Result};
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, FromPrimitive)]
pub(crate) enum TexUsed {
    Platex,
    Uplatex,
    Latex,
    Xelatex,
    Pdflatex,
    Lualatex,
}

impl Display for TexUsed {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            TexUsed::Platex => write!(f, "pLaTeX"),
            TexUsed::Uplatex => write!(f, "upLaTeX"),
            TexUsed::Latex => write!(f, "LaTeX"),
            TexUsed::Xelatex => write!(f, "XeLaTeX"),
            TexUsed::Pdflatex => write!(f, "pdfLaTeX"),
            TexUsed::Lualatex => write!(f, "LuaLaTeX"),
        }
    }
}

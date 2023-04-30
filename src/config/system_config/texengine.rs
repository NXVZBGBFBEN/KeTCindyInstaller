use crate::config::Select;
use num_derive::FromPrimitive;
use std::fmt::{Display, Formatter, Result};
use strum_macros::EnumIter;

#[derive(EnumIter, FromPrimitive)]
pub(crate) enum TexEngine {
    Platex,
    Uplatex,
    Latex,
    Xelatex,
    Pdflatex,
    Lualatex,
}

impl Display for TexEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            TexEngine::Platex => write!(f, "pLaTeX"),
            TexEngine::Uplatex => write!(f, "upLaTeX"),
            TexEngine::Latex => write!(f, "LaTeX"),
            TexEngine::Xelatex => write!(f, "XeLaTeX"),
            TexEngine::Pdflatex => write!(f, "pdfLaTeX"),
            TexEngine::Lualatex => write!(f, "LuaLaTeX"),
        }
    }
}

impl Select for TexEngine {}

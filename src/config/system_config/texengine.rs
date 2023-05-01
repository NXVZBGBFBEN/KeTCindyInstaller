use crate::config::Select;
use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter, Result};
use strum_macros::EnumIter;

#[derive(EnumIter, FromPrimitive, ToPrimitive, Default)]
pub(crate) enum TexEngine {
    Latex,
    #[default]
    Platex,
    Uplatex,
    Xelatex,
    Pdflatex,
    Lualatex,
}

impl Display for TexEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            TexEngine::Latex => write!(f, "LaTeX"),
            TexEngine::Platex => write!(f, "pLaTeX"),
            TexEngine::Uplatex => write!(f, "upLaTeX"),
            TexEngine::Xelatex => write!(f, "XeLaTeX"),
            TexEngine::Pdflatex => write!(f, "pdfLaTeX"),
            TexEngine::Lualatex => write!(f, "LuaLaTeX"),
        }
    }
}

impl Select for TexEngine {}

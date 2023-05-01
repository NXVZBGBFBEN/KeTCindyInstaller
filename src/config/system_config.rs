use crate::config::{Config, Select};

mod graphic;
mod language;
mod texengine;
use graphic::Graphic;
use language::Language;
use texengine::TexEngine;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Error;

pub struct SystemConfig {
    lang: Language,
    tex: TexEngine,
    graphic: Graphic,
}

impl Display for SystemConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            " Language - {}\n \
              Engine   - {}\n \
              Graphic  - {}",
            self.lang, self.tex, self.graphic
        )
    }
}

impl Config<SystemConfig> for SystemConfig {
    fn pick() -> Result<SystemConfig, Error> {
        let lang = Language::select()?;
        let tex = TexEngine::select()?;
        let graphic = Graphic::select()?;
        Ok(Self { lang, tex, graphic })
    }
}

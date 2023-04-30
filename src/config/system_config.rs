use crate::config::Config;

mod graphic;
mod language;
mod texused;
use graphic::Graphic;
use language::Language;
use texused::TexUsed;

use std::fmt::Debug;
use std::io::Error;

#[derive(Debug)]
pub struct SystemConfig {
    lang: Language,
    tex: TexUsed,
    graphics: Graphic,
}

impl SystemConfig {
    pub fn new() -> Result<Self, Error> {
        let lang = Config::select::<Language>()?;
        let tex = Config::select::<TexUsed>()?;
        let graphics = Config::select::<Graphic>()?;
        Ok(Self {
            lang,
            tex,
            graphics,
        })
    }
}

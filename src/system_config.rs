mod graphic;
mod language;
mod texused;
use graphic::Graphic;
use language::Language;
use texused::TexUsed;

use dialoguer::Select;
use num_traits::FromPrimitive;
use std::fmt::Display;
use std::io::Error;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct SystemConfig {
    lang: Language,
    tex: TexUsed,
    graphics: Graphic,
}

impl SystemConfig {
    pub fn new() -> Result<Self, Error> {
        let lang = SystemConfig::select::<Language>()?;
        let tex = SystemConfig::select::<TexUsed>()?;
        let graphics = SystemConfig::select::<Graphic>()?;
        Ok(Self {
            lang,
            tex,
            graphics,
        })
    }
    fn select<T>() -> Result<T, Error>
    where
        T: IntoEnumIterator + Display + FromPrimitive,
    {
        let list: Vec<T> = T::iter().collect();
        let selection = Select::new().items(&list).default(0).interact()?;
        Ok(T::from_usize(selection).unwrap())
    }
}

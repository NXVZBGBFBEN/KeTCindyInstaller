mod path_config;
mod system_config;
pub use system_config::SystemConfig;

use dialoguer;
use num_traits::{FromPrimitive, ToPrimitive};
use std::fmt::Display;
use std::io::Error;
use strum::IntoEnumIterator;

pub trait Config<T> {
    fn pick() -> Result<T, Error>;
}

trait Select {
    fn select<T>() -> Result<T, Error>
    where
        T: IntoEnumIterator + FromPrimitive + ToPrimitive + Default + Display,
    {
        let list: Vec<T> = T::iter().collect();
        let selection = dialoguer::Select::new()
            .items(&list)
            .default(T::default().to_usize().unwrap())
            .interact()?;
        Ok(T::from_usize(selection).unwrap())
    }
}

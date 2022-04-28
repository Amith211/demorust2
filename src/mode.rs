use crate::{run_save_direct, run_save_with_saver};

use strum::{IntoEnumIterator, EnumMessage, Display};
use strum_macros::{EnumDiscriminants, EnumIter, EnumString};

#[derive(Debug, EnumDiscriminants, PartialEq, EnumString, Display)]
#[strum_discriminants(derive(EnumIter))]
#[strum_discriminants(derive(EnumString))]
#[strum_discriminants(derive(Display))]
pub enum Mode<T> {
    #[strum(ascii_case_insensitive)]
    Direct,
    #[strum(ascii_case_insensitive)]
    Saver,
    #[strum(disabled)]
    _Impossible(std::convert::Infallible, std::marker::PhantomData<T>),
}

impl<T> Mode<T>  {
    pub fn names() -> Vec<String> {
        let mut r:Vec<String> = vec![];
        for mode in ModeDiscriminants::iter() {
            r.push(mode.to_string());
        };
        r.pop();
        return r;
    }
}

impl<T> Mode<T> {}

impl Mode<String> {
    pub fn match_mode(&self) -> Option<fn(Option<String>)> {
        match *self {
            Mode::Direct => Some(run_save_direct),
            Mode::Saver => Some(run_save_with_saver),
            Mode::_Impossible(x, _) => match x {},
        }
    }
}
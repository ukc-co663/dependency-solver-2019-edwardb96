use serde::de::{self, Deserialize, Visitor};
use std::str::FromStr;
use std::fmt;
use std::marker::PhantomData;

pub struct DeserializeFromStr<T>(pub PhantomData<fn() -> T>);

impl<'de, T> Visitor<'de> for DeserializeFromStr<T>
    where T : Deserialize<'de> + FromStr<Err = String> {
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("correctly formatted string")
    }

    fn visit_str<E>(self, value: &str) -> Result<T,E>
        where E: de::Error {
        FromStr::from_str(value).map_err(|msg| de::Error::custom(msg))
    }
}

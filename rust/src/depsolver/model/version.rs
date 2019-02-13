use super::deserialize_from_str::DeserializeFromStr;
use serde::de::{Deserialize, Deserializer};
use std::marker::PhantomData;
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Version(pub Vec<i32>);

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.iter()
                              .map(|n| n.to_string())
                              .collect::<Vec<String>>()
                              .join("."))
    }
}

impl FromStr for Version {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version_parts = s.split('.')
                              .map(|ns| ns.parse().expect(&format!("version containted non number '{}'", ns)))
                              .collect::<Vec<i32>>();
        if !version_parts.is_empty() {
            Ok(Version(version_parts))
        } else {
            Err("Version cannot be empty".to_owned())
        }
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D : Deserializer<'de> {
        deserializer.deserialize_any(DeserializeFromStr(PhantomData))
    }
}

use super::constraint::PackageConstraint;
use super::deserialize_from_str::DeserializeFromStr;
use serde::de::{Deserialize, Deserializer};
use std::marker::PhantomData;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
   Install(PackageConstraint),
   Uninstall(PackageConstraint)
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Command::Install(c) =>
                write!(f, "+{}", c),
            Command::Uninstall(c) =>
                write!(f, "-{}", c)
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars();
        match it.next().ok_or("Cannot parse empty command!")? {
            '+' => Ok(Command::Install(FromStr::from_str(it.as_str())?)),
            '-' => Ok(Command::Uninstall(FromStr::from_str(it.as_str())?)),
            c => Err(format!("Unknown command prefix char {}", c))
        }
    }
}

impl<'de> Deserialize<'de> for Command {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D : Deserializer<'de> {
        deserializer.deserialize_any(DeserializeFromStr(PhantomData))
    }
}

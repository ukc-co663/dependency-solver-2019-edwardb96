use super::deserialize_from_str::{DeserializeFromStr};
use super::version::Version;
use super::constraint::PackageConstraint;
use serde::de::{Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;
use std::marker::PhantomData;

#[derive(Debug, Deserialize, Clone)]
pub struct Package {
    #[serde(default)]
    pub id: usize,
    pub name: String,
    pub version: Version,
    pub size: u64,
    #[serde(default)]
    #[serde(rename = "depends")]
    pub dependencies: Vec<Vec<PackageConstraint>>,
    #[serde(default)]
    pub conflicts: Vec<PackageConstraint>
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} v{}, id: {})",
               self.name, self.version, self.id)
    }
}

#[derive(Debug)]
pub struct PackageKey(pub String, pub Version);

impl FromStr for PackageKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars();
        let package_name: String = (&mut it).take_while(|&ch| ch != '=').collect();
        let version: Version = FromStr::from_str(it.as_str())?;
        Ok(PackageKey(package_name, version))
    }
}

impl fmt::Display for PackageKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let PackageKey(name, version) = self;
        write!(f, "{}={}", name, version)
    }
}

impl<'de> Deserialize<'de> for PackageKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D : Deserializer<'de> {
        deserializer.deserialize_any(DeserializeFromStr(PhantomData))
    }
}

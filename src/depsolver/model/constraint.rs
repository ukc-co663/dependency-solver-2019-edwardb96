use super::version::Version;
use super::deserialize_from_str::{DeserializeFromStr};
use itertools::Itertools;
use serde::de::{Deserialize, Deserializer};
use std::marker::PhantomData;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum PackageConstraint {
    Expanded { possibilities: Vec<usize> },
    Unexpanded { name: String, version_constraint: Option<(Relation, Version)> }
}

impl PackageConstraint {
    pub fn version_constraint(&self) -> &Option<(Relation, Version)> {
        match &self {
            PackageConstraint::Unexpanded { name: _, version_constraint } =>
                version_constraint,
            _ => panic!("attempted to get version constraint from expanded package constraint.")
        }
    }

    pub fn name(&self) -> &String {
        match &self {
            PackageConstraint::Unexpanded { name, version_constraint: _ } =>
                name,
            _ => panic!("attempted to get version constraint from expanded package constraint.")
        }
    }

    pub fn possibilities(&self) -> &Vec<usize> {
        match &self {
            PackageConstraint::Expanded { possibilities } => possibilities,
            _ => panic!("attempted to get possibilies from unexpanded package constraint.")
        }
    }

    pub fn possibilities_mut(&mut self) -> &mut Vec<usize> {
        match *self {
            PackageConstraint::Expanded { ref mut possibilities } => possibilities,
            _ => panic!("attempted to get possibilies from unexpanded package constraint.")
        }
    }

    pub fn version_fulfils_constraint(&self, package_version: &Version) -> bool {
        match &self.version_constraint() {
            None                                          => true,
            Some((Relation::Equal, version))              => *package_version == *version,
            Some((Relation::LessThan, version))           => *package_version < *version,
            Some((Relation::LessThanOrEqual, version))    => *package_version <= *version,
            Some((Relation::GreaterThan, version))        => *package_version > *version,
            Some((Relation::GreaterThanOrEqual, version)) => *package_version >= *version,
        }
    }
}

impl fmt::Display for PackageConstraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.version_constraint() {
            None                      => write!(f, "{}", self.name()),
            Some((relation, version)) => write!(f, "{}{}{}", self.name(), relation, &version)
        }
    }
}

impl FromStr for PackageConstraint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars().peekable();

        let package_name: String = (&mut it).peeking_take_while(|ch| match ch {
            '<' | '>' | '=' => false,
            _ => true
        }).collect();

        let inequality_string: String = (&mut it).peeking_take_while(|ch| match ch {
            '<' | '>' | '=' => true,
            _ => false
        }).collect::<String>();

        let constraint = if inequality_string.is_empty() {
            None
        } else {
            let inequality : Relation = FromStr::from_str(&inequality_string)?;
            let version : Version = FromStr::from_str(&it.collect::<String>())?;
            Some((inequality, version))
        };

        Ok(PackageConstraint::Unexpanded {
            name: package_name,
            version_constraint: constraint
        })
    }
}

impl<'de> Deserialize<'de> for PackageConstraint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D : Deserializer<'de> {
        deserializer.deserialize_any(DeserializeFromStr(PhantomData))
    }
}

#[derive(Debug, Clone)]
pub enum Relation {
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equal
}

impl FromStr for Relation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "=" => Ok(Relation::Equal),
            "<" => Ok(Relation::LessThan),
            ">" => Ok(Relation::GreaterThan),
            "<=" => Ok(Relation::LessThanOrEqual),
            ">=" => Ok(Relation::GreaterThanOrEqual),
            _ => Err(format!("Unrecognised relation {}", &s))
        }
    }
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Relation::Equal =>              write!(f, "="),
            Relation::LessThan =>           write!(f, "<"),
            Relation::GreaterThan =>        write!(f, ">"),
            Relation::GreaterThanOrEqual => write!(f, ">="),
            Relation::LessThanOrEqual =>    write!(f, "<=")
        }
    }
}

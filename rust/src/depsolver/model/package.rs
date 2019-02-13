use std::fmt;
use super::version::Version;
use super::constraint::PackageConstraint;

#[derive(Debug, Deserialize)]
pub struct Package {
    #[serde(default)]
    pub id : i32,
    pub name : String,
    pub version : Version,
    pub size : u32,
    #[serde(default)]
    #[serde(rename = "depends")]
    pub dependencies : Vec<Vec<PackageConstraint>>,
    #[serde(default)]
    pub conflicts : Vec<PackageConstraint>
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} v{}, id: {})",
               self.name, self.version, self.id)
    }
}

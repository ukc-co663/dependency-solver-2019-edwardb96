use std::fmt;
use super::version::Version;

#[derive(Debug)]
enum Relation {
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equal
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


#[derive(Debug)]
pub struct PackageConstraint<'a> {
    package_name : &'a str,
    version_constraint : Option<(Relation, Version)>
}

impl<'a> PackageConstraint<'a> {
    fn version_fulfils_constraint(&'a self, package_version : &Version) -> bool {
        match &self.version_constraint {
            None                                          => true,
            Some((Relation::Equal, version))              => *package_version == *version,
            Some((Relation::LessThan, version))           => *package_version < *version,
            Some((Relation::LessThanOrEqual, version))    => *package_version <= *version,
            Some((Relation::GreaterThan, version))        => *package_version > *version,
            Some((Relation::GreaterThanOrEqual, version)) => *package_version >= *version,
        }
    }
}

impl<'a> fmt::Display for PackageConstraint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.version_constraint {
            None                      => write!(f, "{}", self.package_name),
            Some((relation, version)) => write!(f, "{}{}{}", self.package_name, relation, &version)
        }
    }
}

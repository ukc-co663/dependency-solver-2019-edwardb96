use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Version(Vec<i32>);

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.iter()
                              .map(|n| n.to_string())
                              .collect::<Vec<String>>()
                              .join("."))
    }
}

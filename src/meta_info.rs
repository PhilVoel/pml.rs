use crate::ParseError;

#[derive(Debug)]
pub enum MetaInfo {
    Version(Version),
    Template(Template),
}

impl TryFrom<&str> for MetaInfo {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
}

impl TryFrom<&str> for Version {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Template {
    pub name: String,
    pub version: Version,
}

impl TryFrom<&str> for Template {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

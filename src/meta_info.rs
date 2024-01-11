#[derive(Debug)]
pub enum MetaInfo {
    Version(Version),
    Template(Template),
}

impl From<Version> for MetaInfo {
    fn from(value: Version) -> Self {
        MetaInfo::Version(value)
    }
}

impl From<Template> for MetaInfo {
    fn from(value: Template) -> Self {
        MetaInfo::Template(value)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

#[derive(Debug)]
pub struct Template {
}

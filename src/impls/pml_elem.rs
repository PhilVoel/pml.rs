use core::fmt::Display;
use crate::Element::{self, *};
use std::fmt::{Formatter, Result, Error};

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            PmlString(s) => write!(f, "{s}"),
            PmlBool(b) => write!(f, "{b}"),
            PmlI128(i) => write!(f, "{i}"),
            PmlI64(i) => write!(f, "{i}"),
            PmlI32(i) => write!(f, "{i}"),
            PmlI16(i) => write!(f, "{i}"),
            PmlI8(i) => write!(f, "{i}"),
            PmlU128(u) => write!(f, "{u}"),
            PmlU64(u) => write!(f, "{u}"),
            PmlU32(u) => write!(f, "{u}"),
            PmlU16(u) => write!(f, "{u}"),
            PmlU8(u) => write!(f, "{u}"),
            PmlF64(n) => write!(f, "{n}"),
            PmlF32(n) => write!(f, "{n}"),
            IncompleteString(_) => Err(Error),
        }
    }
}

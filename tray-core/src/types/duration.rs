use std::str::FromStr;

use club_kdl::{Error, FromKdlValue};

#[derive(Eq, PartialEq, Debug)]
pub struct Duration(pub String);

impl FromStr for Duration {
    type Err = jiff::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

impl<'de> FromKdlValue<'de> for Duration {
    fn from_kdl_value(value: &'de club_kdl::Value<'de>) -> club_kdl::Result<Self> {
        let v = value.as_string().ok_or_else(|| Error::TypeMismatch {
            expected: "Delta String",
            found: value.to_string(),
        })?;
        Self::from_str(v).map_err(|_| Error::Parse("Failed to parse Duration".into()))
    }
}

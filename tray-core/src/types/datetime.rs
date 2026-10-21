use std::{fmt::Display, str::FromStr};

use club_kdl::{Error, FromKdlValue, ToKdlValue};
use jiff::civil::DateTime;

#[derive(Eq, PartialEq, Debug)]
pub struct TrayDateTime(pub DateTime);

impl FromStr for TrayDateTime {
    type Err = jiff::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f = DateTime::from_str(s)?;
        Ok(Self(f))
    }
}

impl<'de> FromKdlValue<'de> for TrayDateTime {
    fn from_kdl_value(value: &'de club_kdl::Value<'de>) -> club_kdl::Result<Self> {
        let v = value.as_string().ok_or_else(|| Error::TypeMismatch {
            expected: "DateTime String",
            found: value.to_string(),
        })?;
        Self::from_str(v).map_err(|_| Error::Parse("Failed to parse DateTime".into()))
    }
}

impl Display for TrayDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.0.year(), self.0.month(), self.0.day())?;
        if self.0.hour() != 0 || self.0.minute() != 0 {
            write!(f, " {:02}:{:02}", self.0.hour(), self.0.minute())?;
        }
        Ok(())
    }
}

impl<'de> ToKdlValue for TrayDateTime {
    fn to_kdl_value(&self) -> club_kdl::Value<'_> {
        club_kdl::Value::String(self.to_string().into())
    }
}

#[cfg(test)]
mod tests {
    use jiff::civil::{date, time};

    use super::*;

    #[test]
    fn read_basic_date() {
        let input = "2026-10-14";
        let actual = TrayDateTime::from_str(input).unwrap();
        let expected = DateTime::from_parts(date(2026, 10, 14), time(0, 0, 0, 0));
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn read_basic_date_and_time() {
        let input = "2026-10-14 14:00";
        let actual = TrayDateTime::from_str(input).unwrap();
        let expected = DateTime::from_parts(date(2026, 10, 14), time(14, 0, 0, 0));
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn read_malformed_date_time() {
        let input = "10";
        let actual = TrayDateTime::from_str(input).unwrap_err();
        let expected_message =
            "expected four digit year (or leading sign for six digit year), but found end of input";
        assert_eq!(actual.to_string(), expected_message);
    }

    #[test]
    fn write_basic_date() {
        let input = "2026-10-14";
        let intermediary = TrayDateTime::from_str(input).unwrap();
        let actual = intermediary.to_string();
        assert_eq!(actual, input);
    }

    #[test]
    fn write_basic_date_and_time() {
        let input = "2026-10-14 14:00";
        let intermediary = TrayDateTime::from_str(input).unwrap();
        let actual = intermediary.to_string();
        assert_eq!(actual, input);
        let input = "2026-10-14 08:02";
        let intermediary = TrayDateTime::from_str(input).unwrap();
        let actual = intermediary.to_string();
        assert_eq!(actual, input)
    }
}

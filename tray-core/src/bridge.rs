use club_kdl::{KdlDeserialize, KdlSerialize};

pub trait ToKDL {
    /// Convert a Struct back into the KDL string representation. Also trims whitespace off the end.
    ///
    /// # Errors
    /// Can fail if `KdlSerialise` is not correctly implemented for the target type. See `club_kdl::Error` for the reasons. This is just a wrapper function.
    fn to_kdl(&self) -> Result<String, club_kdl::Error>
    where
        Self: KdlSerialize,
    {
        let out = club_kdl::to_string(self)?;
        Ok(out.trim().into())
    }
}

pub trait FromKDL<'de> {
    /// Convert text into a Struct representation.
    ///
    /// # Errors
    /// Can fail if `KdlDeserialize` is not correctly implemented for the target type. See `club_kdl::Error` for the reasons. This is just a wrapper function.
    fn from_kdl(text: &str) -> Result<Self, club_kdl::Error>
    where
        for<'a> Self: KdlDeserialize<'a>,
    {
        club_kdl::from_str::<Self>(text)
    }
}

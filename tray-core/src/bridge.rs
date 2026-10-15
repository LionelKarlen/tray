use club_kdl::KdlDeserialize;

pub trait ToKDL {
    fn to_kdl(&self) -> String;
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

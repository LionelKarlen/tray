use club_kdl::KdlDeserialize;

pub trait ToKDL {
    fn to_kdl(&self) -> String;
}

pub trait FromKDL<'de> {
    fn from_kdl(text: &str) -> Result<Self, club_kdl::Error>
    where
        for<'a> Self: KdlDeserialize<'a>,
    {
        club_kdl::from_str::<Self>(text)
    }
}

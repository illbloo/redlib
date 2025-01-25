use std::fmt;

#[derive(Debug, Clone)]
pub struct Fullname {
    pub kind: ThingKind,
    pub id: String,
}

impl fmt::Display for Fullname {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.kind, self.id)
    }
}

#[derive(Debug, Clone)]
pub enum ThingKind {
    Listing,
    Comment,
    Account,
    Link,
    Message,
    Subreddit,
    Award,
    PromoCampaign,
}

impl ThingKind {
    pub fn from_string(kind: &str) -> Option<Self> {
        match kind {
            "Listing" => Some(ThingKind::Listing),
            "t1" => Some(ThingKind::Comment),
            "t2" => Some(ThingKind::Account),
            "t3" => Some(ThingKind::Link),
            "t4" => Some(ThingKind::Message),
            "t5" => Some(ThingKind::Subreddit),
            "t6" => Some(ThingKind::Award),
            "t8" => Some(ThingKind::PromoCampaign),
            _ => None,
        }
    }

    pub fn from_fullname(fullname: &str) -> Option<Self> {
        let parts: Vec<&str> = fullname.split('_').collect();
        if parts.len() != 2 {
            return None;
        }
        ThingKind::from_string(parts[0])
    }
}

impl fmt::Display for ThingKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ThingKind::Listing => write!(f, "Listing"),
            ThingKind::Comment => write!(f, "t1"),
            ThingKind::Account => write!(f, "t2"),
            ThingKind::Link => write!(f, "t3"),
            ThingKind::Message => write!(f, "t4"),
            ThingKind::Subreddit => write!(f, "t5"),
            ThingKind::Award => write!(f, "t6"),
            ThingKind::PromoCampaign => write!(f, "t8"),
        }
    }
}

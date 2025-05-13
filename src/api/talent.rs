use facet::Facet;
use std::str::FromStr;

#[derive(Facet)]
#[repr(C)]
pub enum TalentPlatform {
    Twitch,
    Mixer,
    Facebook,
    Youtube,
}
impl FromStr for TalentPlatform {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TWITCH" => Ok(Self::Twitch),
            "MIXER" => Ok(Self::Mixer),
            "FACEBOOK" => Ok(Self::Facebook),
            "YOUTUBE" => Ok(Self::Youtube),
            _ => Err(()),
        }
    }
}

#[derive(Facet)]
#[non_exhaustive]
pub struct Talent {
    r#type: String,
    id: i64,
    name: String,
    stream: String,
    twitter: String,
    youtube: String,
    platform: TalentPlatform,
    pronouns: String,
}

pub type TalentSearchResults = crate::SearchResults<Talent>;

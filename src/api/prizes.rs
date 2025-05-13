use facet::Facet;
use std::str::FromStr;

use crate::Event;

/// The state of a prize
/// Unauthenticated API calls will only return prizes with `PrizeState::Accepted`
#[derive(Facet)]
#[repr(C)]
pub enum PrizeState {
    /// A prize pending review
    Pending,
    /// A prize that has been accepted for the event
    Accepted,
    /// A prize that has been denied
    Denied,
    /// A prize that has been flagged for some reason
    Flagged,
}

impl FromStr for PrizeState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PENDING" => Ok(Self::Pending),
            "ACCEPTED" => Ok(Self::Accepted),
            "DENIED" => Ok(Self::Denied),
            "FLAGGED" => Ok(Self::Flagged),
            _ => Err(()),
        }
    }
}

/// A single prize
#[derive(Facet)]
#[non_exhaustive]
pub struct Prize {
    #[facet(default = String::from("prize"))]
    r#type: String,
    id: i64,
    event: Event,
    name: String,
    state: PrizeState,
    startrun: Option<i64>,
    endrun: Option<i64>,
    starttime: Option<String>,
    endtime: Option<String>,
    start_draw_time: String,
    end_draw_time: String,
    description: String,
    shortdescription: String,
    image: String,
    altimage: String,
    imagefile: String,
    estimatedvalue: f64,
    minimumbid: f64,
    sumdonations: bool,
    provider: String,
    creator: String,
    creatorwebsite: bool,
}

/// Raw search results for a prizes query submitted to the API
pub type PrizeSearchResults = crate::SearchResults<Prize>;

use std::str::FromStr;

use facet::Facet;

/// The state of a bid.
///
/// Note: Many of these are not returned without authentication, as they are private.
#[derive(Facet)]
#[repr(C)]
pub enum BidState {
    /// A pending bid
    /// Not returned without authentication
    Pending,
    /// A denied bid
    /// Not returned without authentication
    Denied,
    /// A hidden bid
    /// Not returned without authentication
    Hidden,
    /// An accepted bid that is open for donations
    Opened,
    /// An accepted bid that is no longer accepting donations
    Closed,
}

impl FromStr for BidState {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "PENDING" => Ok(Self::Pending),
            "DENIED" => Ok(Self::Denied),
            "HIDDEN" => Ok(Self::Hidden),
            "OPENED" => Ok(Self::Opened),
            "CLOSED" => Ok(Self::Closed),
            _ => Err(()),
        }
    }
}

/// The type of bidwar this bid represents
#[derive(Facet)]
#[repr(C)]
#[non_exhaustive]
pub enum BidType {
    /// A challenge goal to meet
    Challenge,
    /// A bottom-level choice
    Option,
    /// A choice in a bidwar
    Choice,
}
impl FromStr for BidType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "challenge" => Ok(Self::Challenge),
            "option" => Ok(Self::Option),
            "choice" => Ok(Self::Choice),
            _ => Err(()),
        }
    }
}

/// Represents a Bid from the API
#[derive(Facet)]
#[non_exhaustive]
pub struct Bid {
    #[facet(default = String::from("bid"))]
    r#type: String,
    /// The tracker's internal ID for this bid.
    pub id: i64,
    /// The type of bid this bid represents
    pub bid_type: BidType,
    /// The incentive name or option name this bid represents
    pub name: String,
    /// The event ID this bid is from
    /// Is None when querying an event-specific bid list
    pub event: Option<i64>,
    /// The ID of the SpeedRun this bid applies to
    pub speedrun: i64,
    ///
    pub parent: Option<i64>,
    /// The state of this bid.
    pub state: BidState,
    /// The bid's description. Limited to 1024 characters.
    #[facet(max_inline_length = 1024)]
    pub description: String,
    /// A shorter description for tighter spaces. Limited to 256 characters.
    #[facet(max_inline_length = 256)]
    pub shortdescription: String,
    /// The estimated time to complete the mentioned incentive
    pub estimate: Option<String>,
    /// When to close the bid
    pub close_at: String,
    /// If true, this incentive takes place after the run. If false, this takes place during the run
    pub post_run: bool,
    /// How much this goal has to raise to be met
    /// None if this is a bidwar.
    pub goal: Option<f64>,
    pub chain_goal: Option<f64>,
    pub chain_remaining: Option<f64>,
    /// The total amount raised so far towards this bid
    pub total: f64,
    /// How many donations have been submitted for this bid
    pub count: i64,
    /// If this is an incentive to do something `x` times, this is how many dollars to repeat it at.
    pub repeat: Option<f64>,
    /// Whether this chains with other bids.
    pub chain: bool,
    /// How many options have been accepted for this bidwar.
    /// None if this is not a top-level bidwar bid, or not a bidwar.
    pub accepted_number: Option<i64>,
    pub istarget: bool,
    /// If this is a top-level bidwar bid that allows users to submit their own options(character names, filenames, etc)
    pub allowuseroptions: Option<bool>,
    /// The maximum length of an option for this bidwar.
    pub option_max_length: Option<u8>,
    /// When this was revealed
    pub revealedtime: String,
    pub level: i64,
}

/// Raw search results from the /bids or /events/{id}/bids endpoints
pub type BidSearchResults = crate::SearchResults<Bid>;

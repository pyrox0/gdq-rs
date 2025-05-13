use std::str::FromStr;

use facet::Facet;

use crate::{BidState, Event};

/// Where a donation comes from
#[derive(Facet)]
#[repr(C)]
pub enum DonationDomain {
    /// A locally generated donation(for testing, manually entered, etc)
    Local,
    /// No idea tbh
    ChipIn,
    /// A donation that's come from paypal
    PayPal,
}
impl FromStr for DonationDomain {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LOCAL" => Ok(Self::Local),
            "CHIPIN" => Ok(Self::ChipIn),
            "PAYPAL" => Ok(Self::PayPal),
            _ => Err(()),
        }
    }
}

/// The state of a donation's transaction
#[derive(Facet)]
#[repr(C)]
pub enum DonationTransactionState {
    /// A pending donation
    Pending,
    /// A completed donation
    Completed,
    /// A cancelled donation
    Cancelled,
    /// A donation that has been flagged by the payment processor or by the admins
    Flagged,
}
impl FromStr for DonationTransactionState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PENDING" => Ok(Self::Pending),
            "COMPLETED" => Ok(Self::Completed),
            "CANCELLED" => Ok(Self::Cancelled),
            "FLAGGED" => Ok(Self::Flagged),
            _ => Err(()),
        }
    }
}

/// Metadata for handling donation readers
#[derive(Facet)]
#[repr(C)]
pub enum DonationReadState {
    /// A donation pending review
    Pending,
    /// A donation that's ready for a host to read
    ReadyToRead,
    /// A donation that has been ignored by a host or the donations team
    Ignored,
    /// A donation that has been read live
    Read,
    /// A donation flagged for review
    Flagged,
}
impl FromStr for DonationReadState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PENDING" => Ok(Self::Pending),
            "READY" => Ok(Self::ReadyToRead),
            "IGNORED" => Ok(Self::Ignored),
            "READ" => Ok(Self::Read),
            "FLAGGED" => Ok(Self::Flagged),
            _ => Err(()),
        }
    }
}

/// The state of review for a donation's comment
#[derive(Facet)]
#[repr(C)]
pub enum DonationCommentState {
    /// Donation has no comment
    Absent,
    /// Donation comment pending review
    Pending,
    /// Donation comment has been denied and will not be publically shown
    Denied,
    /// Donation comment is approved for public showing on the tracker
    Approved,
    /// Donation comment has been flagged
    Flagged,
}
impl FromStr for DonationCommentState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ABSENT" => Ok(Self::Absent),
            "PENDING" => Ok(Self::Pending),
            "DENIED" => Ok(Self::Denied),
            "APPROVED" => Ok(Self::Approved),
            "FLAGGED" => Ok(Self::Flagged),
            _ => Err(()),
        }
    }
}

/// What language a donation comment is in
/// Almost always `un` since the built-in donation pages do not set the language
#[derive(Facet)]
#[repr(C)]
pub enum DonationCommentLanguage {
    /// Unknown language
    Unknown,
    /// English language comment
    English,
    /// French language comment
    French,
    /// German language comment
    German,
}
impl FromStr for DonationCommentLanguage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "un" => Ok(Self::Unknown),
            "en" => Ok(Self::English),
            "fr" => Ok(Self::French),
            "de" => Ok(Self::German),
            _ => Err(()),
        }
    }
}

/// A bid that a donation is going towards
#[derive(Facet)]
#[non_exhaustive]
pub struct DonationBid {
    #[facet(default = String::from("donationbid"))]
    r#type: String,
    /// The ID of this donation bid
    id: i64,
    /// The ID of the donation
    donation: i64,
    /// The ID of the bid this donation is for
    bid: i64,
    /// The name of the bid
    bid_name: String,
    /// The state of the bid
    bid_state: BidState,
    /// The amount of money sent towards this bid
    amount: f64,
}

#[derive(Facet)]
#[non_exhaustive]
pub struct Donation {
    #[facet(default = String::from("donation"))]
    r#type: String,
    id: i64,
    donor_name: String,
    event: i64,
    domain: DonationDomain,
    transactionstate: DonationTransactionState,
    readstate: DonationReadState,
    commentstate: DonationCommentState,
    amount: f64,
    currency: String,
    timereceived: String,
    comment: Option<String>,
    commentlanguage: DonationCommentLanguage,
    pinned: bool,
    bids: Vec<DonationBid>,
}

#[derive(Facet)]
pub struct Milestone {
    #[facet(default = String::from("milestone"))]
    r#type: String,
    id: i64,
    event: Event,
    start: f64,
    amount: f64,
    name: String,
    run: i64,
    visible: bool,
    description: String,
    short_description: String,
}

pub type DonationSearchResults = crate::SearchResults<Donation>;
pub type MilestoneSearchResults = crate::SearchResults<Milestone>;

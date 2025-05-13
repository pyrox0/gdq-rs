#![warn(missing_docs)]
#![forbid(unsafe_code)]
/// Bids, Bidwars, and other types of incentives
pub mod bids;
/// Backend information about countries and regions
pub mod countries;
/// Donations, donors, and milestones
pub mod donations;
/// Top-level events
pub mod events;
/// Interviews and other interstitials
pub mod interviews;
/// Prizes
pub mod prizes;
/// Speedruns and their videos
pub mod runs;
/// Search results and other utilities
pub mod search;
/// The folks that make these shows work
pub mod talent;

pub use bids::*;
pub use countries::*;
pub use donations::*;
pub use events::*;
pub use interviews::*;
pub use prizes::*;
pub use runs::*;
pub use search::*;
pub use talent::*;

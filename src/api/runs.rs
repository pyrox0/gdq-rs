use facet::Facet;
use std::str::FromStr;

use crate::{Event, Talent};

#[derive(Facet)]
#[repr(C)]
pub enum SpeedRunOnSiteOption {
    Onsite,
    Online,
    Hybrid,
}
impl FromStr for SpeedRunOnSiteOption {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ONSITE" => Ok(Self::Onsite),
            "ONLINE" => Ok(Self::Online),
            "HYBRID" => Ok(Self::Hybrid),
            _ => Err(()),
        }
    }
}

#[derive(Facet)]
#[non_exhaustive]
pub struct SpeedRunVideoLink {
    id: i64,
    link_type: String,
    url: String,
}

#[derive(Facet)]
#[non_exhaustive]
pub struct SpeedRun {
    #[facet(default = String::from("speedrun"))]
    r#type: String,
    id: i64,
    event: Event,
    name: String,
    display_name: String,
    twitch_name: String,
    description: String,
    category: String,
    coop: bool,
    onsite: SpeedRunOnSiteOption,
    console: String,
    release_year: Option<i64>,
    runners: Vec<Talent>,
    hosts: Vec<Talent>,
    commentators: Vec<Talent>,
    starttime: String,
    endtime: String,
    order: i64,
    run_time: String,
    setup_time: String,
    anchor_time: Option<String>,
    layout: String,
    video_links: Vec<SpeedRunVideoLink>,
    priority_tag: Option<String>,
    tags: Vec<String>,
}

pub type SpeedRunSearchResults = crate::SearchResults<SpeedRun>;

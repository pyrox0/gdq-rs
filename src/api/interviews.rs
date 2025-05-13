use facet::Facet;

use crate::{Event, Talent};

#[derive(Facet)]
#[non_exhaustive]
pub struct Interview {
    #[facet(default = String::from("interview"))]
    r#type: String,
    id: i64,
    event: Event,
    anchor: i64,
    order: i64,
    suborder: i64,
    length: String,
    tags: Vec<String>,
    social_media: bool,
    interviewers: Vec<Talent>,
    topic: String,
    public: bool,
    prerecorded: bool,
    producer: String,
    subjects: Vec<Talent>,
    camera_operator: String,
}

pub type InterviewSearchResults = crate::SearchResults<Interview>;

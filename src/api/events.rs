use facet::Facet;

#[derive(Facet)]
#[non_exhaustive]
pub struct Event {
    #[facet(default = String::from("event"))]
    r#type: String,
    id: i64,
    short: String,
    name: String,
    paypalcurrency: String,
    hashtag: String,
    datetime: String,
    timezone: String,
    receivername: String,
    receiver_solicitation_text: String,
    receiver_logo: String,
    receiver_privacy_policy: String,
    use_one_step_screening: bool,
    locked: bool,
    allow_donations: bool,
}

pub type EventSearchResults = crate::SearchResults<Event>;

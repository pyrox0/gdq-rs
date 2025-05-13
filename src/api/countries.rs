use facet::Facet;

/// A country from the API
#[derive(Facet)]
#[non_exhaustive]
pub struct Country {
    #[facet(default = String::from("country"))]
    r#type: String,
    /// Official ISO 3166 name for the country
    pub name: String,
    /// ISO 3166-1 two-letter code for the country
    pub alpha2: String,
    /// ISO 3166-1 three-letter code for the country
    pub alpha3: String,
    /// ISO 3166-1 numeric code for the country
    pub numeric: String,
}

/// A country region from the API
#[derive(Facet)]
#[non_exhaustive]
pub struct CountryRegion {
    #[facet(default = String::from("countryregion"))]
    r#type: String,
    /// The internal ID of the region
    pub id: i64,
    /// The name of the region
    pub name: String,
    /// The country that this region is a part of.
    pub country: String,
}

/// Raw search results from the /country endpoint
pub type CountrySearchResults = crate::SearchResults<Country>;
/// Raw search results from the /regions endpoint
pub type CountryRegionSearchResults = crate::SearchResults<CountryRegion>;

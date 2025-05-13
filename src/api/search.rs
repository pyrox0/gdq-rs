use facet::Facet;

/// A page of search results from the API.
#[derive(Facet)]
pub struct SearchResults<T> {
    /// The total number of results for your query.
    /// If there is more than one page of results, this count and the length of `results` will not match.
    pub count: i64,
    /// A URL representing the next page of results
    /// `null` if there is no next page
    pub next: Option<String>,
    /// A URL representing the previous page of results.
    /// `null` if there is no previous page.
    pub previous: Option<String>,
    /// The results returned by your query.
    pub results: Vec<T>,
}

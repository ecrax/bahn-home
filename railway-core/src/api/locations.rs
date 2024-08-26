use crate::Place;

/// The response given by [`Provider::locations`](crate::Provider::locations)
pub type LocationsResponse = Vec<Place>;

#[derive(Debug)]
/// The options for [`Provider::locations`](crate::Provider::locations)
pub struct LocationsOptions {
    /// What to query for.
    pub query: String,
    /// How many results to return.
    pub results: u64,
    /// What language to query in.
    pub language: Option<String>,
}

impl Default for LocationsOptions {
    fn default() -> Self {
        Self {
            query: Default::default(),
            results: 10,
            language: Default::default(),
        }
    }
}

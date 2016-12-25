//! Define types which validate their inputs for use throughout the app.

#[derive(Debug, PartialEq)]
pub struct Url(String);

impl Url {
    /// Get a URL. `Err` if the item passed in is not a spec-conformant URL.
    pub fn new(unvalidated_url: String) -> Result<Url, String> {
        // TODO: validate the URLs!
        Ok(Url(unvalidated_url))
    }

    /// Get the value of a valid URL.
    pub fn value(&self) -> String {
        self.0.clone()
    }
}

#[derive(Deserialize, Debug)]
pub(super) struct Book {
    title: Option<String>,
    author: Option<String>,
    editors: Option<Vec<String>>,
    translators: Option<Vec<String>>,
    cover: Option<String>,
    link: Option<String>,
    year: Option<u16>,
    review: Option<Review>,
}

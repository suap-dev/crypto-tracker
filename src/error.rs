#[derive(Clone, Debug)]
pub enum CryoError {
    FetchError(String),
    ParseError(String),
}

#[derive(Debug, Fail)]
pub enum CallbackError {
    #[fail(display = "Failed to parse callback xml")]
    Xml,
}

use crate::error::CryoError;

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    RequestData,
    Update(Result<String, CryoError>),
}

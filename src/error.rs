use serde::Serialize;

#[derive(Serialize)]
pub enum Error {
    InvalidSigma,
}

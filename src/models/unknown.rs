use fake::{Dummy, Fake};
use serde::Serialize;

#[derive(Debug, Dummy, Serialize)]
pub struct UnknownEvent {
    unknown: String,
}

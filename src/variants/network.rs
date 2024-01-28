use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Display, Serialize, Deserialize)]
pub enum Network {
    Test,
    Dev,
    XahauTest,
    SidechainTest,
    Custom(String),
}

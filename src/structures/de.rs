use crate::structures::Cell;
use serde::{Deserialize, Deserializer};

impl<'de> Deserialize<'de> for Cell {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _ = deserializer;
        unimplemented!()
    }
}

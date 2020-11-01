use crate::structures::MarkdownCell;
use serde::{ser::SerializeStruct, Serialize, Serializer};

impl Serialize for MarkdownCell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MarkdownCell", 3)?;
        s.serialize_field("cell_type", "markdown")?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("source", &self.source)?;
        s.end()
    }
}

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
mod de;
mod ser;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Jupyter {
    nbformat: usize,
    nbformat_minor: usize,
    metadata: Map<String, Value>,
    cells: Vec<Cell>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {}

#[derive(Clone, Debug, Serialize)]
pub enum Cell {
    Markdown(MarkdownCell),
    Code(CodeCell),
    Raw(RawCell),
}

#[derive(Clone, Debug, Deserialize)]
pub struct MarkdownCell {
    metadata: String,
    source: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeCell {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RawCell {}

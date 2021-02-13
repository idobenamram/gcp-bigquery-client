use serde::{Serialize, Deserialize};
use crate::model::table_field_schema::TableFieldSchema;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableSchema {
    /// Describes the fields in a table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<TableFieldSchema>>,
}

impl TableSchema {
    pub fn new(fields: Vec<TableFieldSchema>) -> Self {
        Self { fields: Some(fields) }
    }

    pub fn fields(&self) -> &Option<Vec<TableFieldSchema>> {
        &self.fields
    }

    pub fn field_count(&self) -> usize {
        self.fields.as_ref().map_or(0, |fields| fields.len())
    }
}

impl Default for TableSchema {
    fn default() -> Self {
        Self { fields: None }
    }
}
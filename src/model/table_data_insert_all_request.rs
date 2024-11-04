use crate::error::BQError;
use crate::model::table_data_insert_all_request_rows::TableDataInsertAllRequestRows;
use serde::{Deserialize, Serialize};

#[cfg(feature = "gzip")]
use flate2::{write::GzEncoder, Compression};
#[cfg(feature = "gzip")]
use std::io::Write;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableDataInsertAllRequest {
    /// [Optional] Accept rows that contain values that do not match the schema. The unknown values are ignored. Default is false, which treats unknown values as errors.
    ignore_unknown_values: bool,
    /// The resource type of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<String>,
    /// The rows to insert.
    rows: Vec<TableDataInsertAllRequestRows>,
    /// [Optional] Insert all valid rows of a request, even if invalid rows exist. The default value is false, which causes the entire request to fail if any invalid rows exist.
    skip_invalid_rows: bool,
    /// If specified, treats the destination table as a base template, and inserts the rows into an instance table named \"{destination}{templateSuffix}\". BigQuery will manage creation of the instance table, using the schema of the base template table. See https://cloud.google.com/bigquery/streaming-data-into-bigquery#template-tables for considerations when working with templates tables.
    #[serde(skip_serializing_if = "Option::is_none")]
    template_suffix: Option<String>,
}

impl TableDataInsertAllRequest {
    pub fn new() -> Self {
        TableDataInsertAllRequest {
            ignore_unknown_values: false,
            kind: None,
            rows: vec![],
            skip_invalid_rows: false,
            template_suffix: None,
        }
    }

    pub fn ignore_unknown_values(&mut self) -> &mut Self {
        self.ignore_unknown_values = true;
        self
    }

    pub fn kind(&mut self, kind: impl Into<String>) -> &mut Self {
        self.kind = Some(kind.into());
        self
    }

    pub fn add_row<T: Serialize>(&mut self, insert_id: Option<String>, object: T) -> Result<(), BQError> {
        let json = serde_json::to_value(object)?;
        self.rows.push(TableDataInsertAllRequestRows { insert_id, json });
        Ok(())
    }

    pub fn add_rows(&mut self, objects: Vec<TableDataInsertAllRequestRows>) -> Result<(), BQError> {
        self.rows.extend(objects);
        Ok(())
    }

    pub fn skip_invalid_rows(&mut self) -> &mut Self {
        self.skip_invalid_rows = true;
        self
    }

    pub fn template_suffix(&mut self, suffix: impl Into<String>) -> &mut Self {
        self.template_suffix = Some(suffix.into());
        self
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn clear(&mut self) {
        self.rows.clear()
    }
}

/// A gzipped version of `TableDataInsertAllRequest`.
#[cfg(feature = "gzip")]
pub struct TableDataInsertAllRequestGzipped {
    pub(crate) data: Vec<u8>,
}

#[cfg(feature = "gzip")]
impl TryFrom<TableDataInsertAllRequest> for TableDataInsertAllRequestGzipped {
    type Error = BQError;

    fn try_from(request: TableDataInsertAllRequest) -> Result<Self, Self::Error> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(serde_json::to_string(&request)?.as_bytes())?;
        let gzipped_data = encoder.finish()?;
        Ok(Self { data: gzipped_data })
    }
}

#[cfg(feature = "gzip")]
impl TableDataInsertAllRequestGzipped {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}


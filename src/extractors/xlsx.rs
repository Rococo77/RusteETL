//! XLSX extractor implementation.
//!
//! Extracts rows from an Excel `.xlsx` file using the calamine crate. It reads a
//! sheet (default first) and returns rows as Vec<Vec<String>>.

use crate::error::EtlError;
use calamine::{DataType, Reader, open_workbook_auto};

/// XLSX extractor config.
pub struct XlsxExtractor {
    /// Path to the xlsx file.
    pub path: String,
    /// Optional sheet name. If None the first sheet is used.
    pub sheet: Option<String>,
}

impl XlsxExtractor {
    /// Extract all rows from the configured sheet.
    pub fn extract(&self) -> Result<Vec<Vec<String>>, EtlError> {
        let mut wb = open_workbook_auto(&self.path)?;
        let sheet_name = if let Some(ref s) = self.sheet {
            s.clone()
        } else {
            // pick first sheet
            wb.sheet_names()
                .first()
                .cloned()
                .ok_or_else(|| EtlError::Other("No sheets found in workbook".to_string()))?
        };

        let range = wb
            .worksheet_range(&sheet_name)
            .ok_or_else(|| EtlError::Other(format!("Sheet not found: {}", sheet_name)))??;

        let mut out = Vec::new();
        for row in range.rows() {
            let r: Vec<String> = row
                .iter()
                .map(|c| match c {
                    DataType::String(s) => s.clone(),
                    DataType::Float(f) => f.to_string(),
                    DataType::Int(i) => i.to_string(),
                    DataType::Bool(b) => b.to_string(),
                    DataType::Empty => "".to_string(),
                    other => format!("{}", other),
                })
                .collect();
            out.push(r);
        }

        Ok(out)
    }
}

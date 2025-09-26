use crate::error::EtlError;
use std::fs::File;
use std::io::Read;

/// Simple JSON extractor supporting NDJSON (newline delimited JSON) and a top-level array of arrays or objects.
#[derive(Clone)]
pub struct JsonExtractor {
    pub path: String,
}

impl JsonExtractor {
    pub fn extract(&self) -> Result<Vec<Vec<String>>, EtlError> {
        let mut content = String::new();
        let mut file = File::open(&self.path).map_err(EtlError::Io)?;
        file.read_to_string(&mut content).map_err(EtlError::Io)?;

        // Try parsing whole content as a JSON value first. If it is an array,
        // accept array-of-arrays or array-of-objects. If parsing fails (e.g.
        // trailing NDJSON records), fall back to line-by-line NDJSON parsing.
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&content) {
            if let serde_json::Value::Array(arr) = v {
                let mut out = Vec::new();
                for item in arr {
                    match item {
                        serde_json::Value::Array(a) => {
                            let row = a
                                .into_iter()
                                .map(|cell| match cell {
                                    serde_json::Value::String(s) => s,
                                    other => other.to_string(),
                                })
                                .collect();
                            out.push(row);
                        }
                        serde_json::Value::Object(obj) => {
                            // preserve order as insertion order from serde_json
                            let row = obj
                                .into_iter()
                                .map(|(_k, v)| match v {
                                    serde_json::Value::String(s) => s,
                                    other => other.to_string(),
                                })
                                .collect();
                            out.push(row);
                        }
                        other => {
                            // scalar in array, wrap as single column
                            out.push(vec![other.to_string()]);
                        }
                    }
                }
                return Ok(out);
            }
            // if whole-file JSON parsed but is not array, fall through to NDJSON logic
        }

        // Otherwise assume NDJSON
        let mut out = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let v: serde_json::Value = serde_json::from_str(trimmed).map_err(|e| EtlError::Other(e.to_string()))?;
            match v {
                serde_json::Value::Array(a) => {
                    let row = a
                        .into_iter()
                        .map(|cell| match cell {
                            serde_json::Value::String(s) => s,
                            other => other.to_string(),
                        })
                        .collect();
                    out.push(row);
                }
                serde_json::Value::Object(obj) => {
                    let row = obj
                        .into_iter()
                        .map(|(_k, v)| match v {
                            serde_json::Value::String(s) => s,
                            other => other.to_string(),
                        })
                        .collect();
                    out.push(row);
                }
                other => out.push(vec![other.to_string()]),
            }
        }
        Ok(out)
    }

    /// Async wrapper around `extract` using blocking task.
    pub async fn extract_async(&self) -> Result<Vec<Vec<String>>, EtlError> {
        let me = self.clone();
        let res = tokio::task::spawn_blocking(move || me.extract())
            .await
            .map_err(|e| EtlError::Other(format!("Task join error: {}", e)))??;
        Ok(res)
    }
}

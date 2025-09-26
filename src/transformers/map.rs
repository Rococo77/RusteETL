/// Map transformer: applies a simple string operation to a specified column.
/// Supported operations: prefix, suffix, replace, uppercase, lowercase
pub struct MapTransformer {
    pub column: usize,
    pub operation: MapOp,
}

pub enum MapOp {
    Prefix(String),
    Suffix(String),
    Replace { from: String, to: String },
    Uppercase,
    Lowercase,
}

impl MapTransformer {
    pub fn transform(&self, data: Vec<Vec<String>>) -> Vec<Vec<String>> {
        data.into_iter()
            .map(|mut row| {
                if self.column < row.len() {
                    let cell = row[self.column].clone();
                    let new = match &self.operation {
                        MapOp::Prefix(p) => format!("{}{}", p, cell),
                        MapOp::Suffix(s) => format!("{}{}", cell, s),
                        MapOp::Replace { from, to } => cell.replace(from, to),
                        MapOp::Uppercase => cell.to_uppercase(),
                        MapOp::Lowercase => cell.to_lowercase(),
                    };
                    row[self.column] = new;
                }
                row
            })
            .collect()
    }
}

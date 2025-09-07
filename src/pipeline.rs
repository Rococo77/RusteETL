/// Orchestration du pipeline ETL
use crate::extractors::Extractor;
use crate::transformers::Transformer;
use crate::loaders::Loader;

pub struct Pipeline<'a> {
    pub extractor: &'a dyn Extractor,
    pub transformer: &'a dyn Transformer,
    pub loader: &'a dyn Loader,
}

impl<'a> Pipeline<'a> {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.extractor.extract()?;
        let data = self.transformer.transform(data);
        self.loader.load(data)?;
        Ok(())
    }
}

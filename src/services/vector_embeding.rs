use fastembed::TextEmbedding;

use crate::error::AppError;

// A wrapper for embeding model
pub struct Embeder {
    model: TextEmbedding,
}

impl Embeder {
    pub fn try_new() -> Result<Self, AppError> {
        Ok(Self {
            model: TextEmbedding::try_new(Default::default())?,
        })
    }
    pub fn embed(&mut self, string: &str) -> Result<Vec<f32>, AppError> {
        let documents = vec![string];
        Ok(self
            .model
            .embed(documents, None)
            .map_err(|_| AppError::VecEmbedingError)?
            .into_iter()
            .next()
            .unwrap())
    }
}

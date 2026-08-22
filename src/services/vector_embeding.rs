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
    pub fn embed(&mut self, description: &str, content: &str) -> Result<Vec<f32>, AppError> {
        let documents = vec![String::from(description) + content];
        Ok(self
            .model
            .embed(documents, None)
            .map_err(|_| AppError::VecEmbedingError)?
            .into_iter()
            .next()
            .unwrap())
    }
}

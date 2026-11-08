// Learning pipeline — processes feedback for improvement
pub struct LearningPipeline {
    pub iterations: u64,
    pub accuracy: f64,
}

impl LearningPipeline {
    pub fn new() -> Self { Self { iterations: 0, accuracy: 0.0 } }

    pub fn train(&mut self, feedback_score: f64) {
        self.iterations += 1;
        let alpha = 0.01;
        self.accuracy += alpha * (feedback_score - self.accuracy);
    }

    pub fn is_converged(&self, threshold: f64) -> bool {
        self.accuracy >= threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_learning() {
        let mut lp = LearningPipeline::new();
        for _ in 0..100 { lp.train(0.95); }
        assert!(lp.accuracy > 0.5);
        assert!(lp.is_converged(0.5));
    }
}

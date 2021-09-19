
use std::f32;

struct AdamOptimizer {
    learning_rate: f32,
    beta1: f32,
    beta2: f32,
    epsilon: f32,
    iteration: usize,
    m: Vec<f32>,
    v: Vec<f32>,
}

impl AdamOptimizer {
    fn new(learning_rate: f32, beta1: f32, beta2: f32, epsilon: f32, params: &[f32]) -> AdamOptimizer {
        AdamOptimizer {
            learning_rate,
            beta1,
            beta2,
            epsilon,
            iteration: 1,
            m: Vec::from(params),
            v: Vec::from(params),
        }
    }

    fn update(&mut self, params: &mut [f32], grads: &[f32]) {
        let size = params.len();
        for i in 0..size {
            self.m[i] = self.beta1 * self.m[i] + (1.0 - self.beta1) * grads[i];
            self.v[i] = self.beta2 * self.v[i] + (1.0 - self.beta2) * grads[i] * grads[i];
            let m_hat = self.m[i] / (1.0 - self.beta1.powf(self.iteration as f32));
            let v_hat = self.v[i] / (1.0 - self.beta2.powf(self.iteration as f32));
            params[i] -= self.learning_rate * m_hat / (v_hat.sqrt() + self.epsilon);
        }
        self.iteration += 1;
    }
}

struct SGDOptimizer {
    learning_rate: f32,
}

impl SGDOptimizer {
    fn new(learning_rate: f32) -> SGDOptimizer {
        SGDOptimizer { learning_rate }
    }

    fn update(&self, params: &mut [f32], grads: &[f32]) {
        let size = params.len();
        for i in 0..size {
            params[i] -= self.learning_rate * grads[i];
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_adam_optimizer() {
        use super::AdamOptimizer;
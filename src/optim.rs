
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
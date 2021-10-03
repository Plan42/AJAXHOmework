
use std::ops::{Add, Mul};

/// A multi-dimensional array of floating-point values.
struct Tensor {
    /// The values of the tensor as a vector of floating-point numbers.
    data: Vec<f32>,
    /// The size of each dimension of the tensor as a vector of unsigned integers.
    shape: Vec<usize>,
}

impl Tensor {
    /// Creates a new tensor with the given `data` and `shape`.
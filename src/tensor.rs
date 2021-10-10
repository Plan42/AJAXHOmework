
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
    fn new(data: Vec<f32>, shape: Vec<usize>) -> Tensor {
        Tensor { data, shape }
    }

    /// Creates a new tensor with the given `shape` and all values initialized to 0.
    fn from_shape(shape: Vec<usize>) -> Tensor {
        let size = shape.iter().product();

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
        Tensor {
            data: vec![0.0; size],
            shape,
        }
    }

    /// Reshapes the tensor to have the given `new_shape`.
    ///
    /// # Panics
    ///
    /// Panics if the new shape has a different size than the original shape.
    fn reshape(&mut self, new_shape: Vec<usize>) {
        let size: usize = self.shape.iter().product();
        let new_size = new_shape.iter().product();
        if size != new_size {
            panic!("Cannot reshape tensor of size {} to size {}", size, new_size);
        }
        self.shape = new_shape;
    }

    /// Returns the size of the tensor, i.e. the total number of elements.
    fn size(&self) -> usize {
        self.data.len()
    }

    /// Returns a reference to the shape of the tensor.
    fn shape(&self) -> &Vec<usize> {
        &self.shape
    }

    /// Returns the rank of the tensor, i.e. the number of dimensions.
    fn rank(&self) -> usize {
        self.shape.len()
use cudnn::Result;
use cudnn::scalar;
use cudnn::context;
use cudnn::tensor;

use cudnn::softmax;
pub use cudnn::softmax::{Algorithm, Mode};

pub struct Softmax {
    algo: Algorithm,
    mode: Mode,
}

impl Softmax {
    pub fn new(algo: softmax::Algorithm, mode: softmax::Mode) -> Softmax {
        Softmax { algo, mode }
    }

    pub fn foward<'a, T: scalar::Float>(&self,
                                        context: &mut context::Context,
                                        x: tensor::Tensor<'a, T>,
                                        y: tensor::TensorMut<'a, T>)
                                        -> Result<()> {
        {
            softmax::forward(context, self.algo, self.mode, T::ONE, x, T::ZERO, y)?;
        }
        Ok(())
    }

    pub fn backward<'a, T: scalar::Float>(&self,
                                          context: &mut context::Context,
                                          y: tensor::Tensor<'a, T>,
                                          dy: tensor::Tensor<'a, T>,
                                          dx: tensor::TensorMut<'a, T>)
                                          -> Result<()> {
        {
            softmax::backward(context,
                              self.algo,
                              self.mode,
                              T::ONE,
                              y,
                              Some(dy),
                              T::ZERO,
                              dx)?;
        }
        Ok(())
    }
}

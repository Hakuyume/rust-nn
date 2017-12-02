extern crate cuda;
extern crate cudnn;
extern crate nn;

use nn::dataset::MNIST;

const BATCH_SIZE: usize = 32;
const N_CLASSES: usize = 10;

fn main() {
    let mut context = cudnn::context::Context::new().unwrap();

    let x_desc = cudnn::tensor::Descriptor::new_4d(cudnn::tensor::Format::NCHW,
                                                   BATCH_SIZE,
                                                   MNIST::SIZE * MNIST::SIZE,
                                                   1,
                                                   1)
            .unwrap();
    let y_desc =
        cudnn::tensor::Descriptor::new_4d(cudnn::tensor::Format::NCHW, BATCH_SIZE, N_CLASSES, 1, 1)
            .unwrap();
    let w_desc = cudnn::filter::Descriptor::new_4d(cudnn::tensor::Format::NCHW,
                                                   N_CLASSES,
                                                   MNIST::SIZE * MNIST::SIZE,
                                                   1,
                                                   1)
            .unwrap();
    let conv_desc = cudnn::convolution::Descriptor::new_2d(0,
                                                           0,
                                                           1,
                                                           1,
                                                           1,
                                                           1,
                                                           cudnn::convolution::Mode::Convolution)
            .unwrap();
    let algo =
        cudnn::convolution::get_forward_algorithm(&mut context,
                                                  &x_desc,
                                                  &w_desc,
                                                  &conv_desc,
                                                  &y_desc,
                                                  cudnn::convolution::FwdPreference::PreferFastest)
                .unwrap();
    let workspace_size = cudnn::convolution::get_forward_workspace_size(&mut context,
                                                                        &x_desc,
                                                                        &w_desc,
                                                                        &conv_desc,
                                                                        &y_desc,
                                                                        algo)
            .unwrap();

    let mnist = MNIST::new("mnist").unwrap();
    let mut x = vec![0.; x_desc.len()];
    for i in 0..BATCH_SIZE {
        let (image, _) = mnist.train.get(i);
        for k in 0..MNIST::SIZE * MNIST::SIZE {
            x[i * MNIST::SIZE * MNIST::SIZE + k] = image[k].into();
        }
    }

    let mut x_dev = cuda::memory::Memory::new(x.len()).unwrap();
    cuda::memory::memcpy(&mut x_dev, &x).unwrap();
    let mut y_dev = cuda::memory::Memory::new(y_desc.len()).unwrap();
    let w_dev = cuda::memory::Memory::new(w_desc.len()).unwrap();

    let mut workspace = cuda::memory::Memory::new(workspace_size).unwrap();
    cudnn::convolution::forward(&mut context,
                                1.,
                                cudnn::tensor::Tensor::new(&x_desc, &x_dev),
                                cudnn::filter::Filter::new(&w_desc, &w_dev),
                                &conv_desc,
                                algo,
                                &mut workspace,
                                0.,
                                cudnn::tensor::TensorMut::new(&y_desc, &mut y_dev))
            .unwrap();

    let mut y = vec![0.; y_desc.len()];
    cuda::memory::memcpy(&mut y, &y_dev).unwrap();
    println!("{:?}", &y);
}

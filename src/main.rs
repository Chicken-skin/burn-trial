mod data;
mod inference;
mod model;
mod training;
use burn::{
    backend::{wgpu::AutoGraphicsApi, Autodiff, Wgpu},
    data::dataset::Dataset,
    optim::AdamConfig,
};

use crate::{model::ModelConfig, training::TrainingConfig};

fn main() {
    type MyBackend = Wgpu<AutoGraphicsApi, f32, i32>;
    type MyAutodiffBackend = Autodiff<MyBackend>;

    let device = burn::backend::wgpu::WgpuDevice::default();
    let artifact_dir = "./tmp/output";
    // training
    training::train::<MyAutodiffBackend>(
        artifact_dir,
        TrainingConfig::new(ModelConfig::new(10, 512), AdamConfig::new()),
        device.clone(),
    );
    // inference
    inference::infer::<MyBackend>(
        artifact_dir,
        device,
        burn::data::dataset::vision::MNISTDataset::test()
            .get(42)
            .unwrap(),
    );
}

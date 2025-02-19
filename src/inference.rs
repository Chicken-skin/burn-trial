use crate::{data::MnistBatcher, model::Model, training::TrainingConfig};
use burn::{
    config::Config,
    data::{dataloader::batcher::Batcher, dataset::vision::MNISTItem},
    module::Module,
    record::{CompactRecorder, Recorder},
    tensor::backend::Backend,
};

pub fn infer<B: Backend>(artifact_dir: &str, device: B::Device, item: MNISTItem) {
    let config = TrainingConfig::load(format!("{artifact_dir}/config.json"))
        .expect("Config should exist for the model");
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Training model should exist");

    let model: Model<B> = config.model.init(&device).load_record(record);

    let label = item.label;
    let batcher = MnistBatcher::new(device);
    let batch = batcher.batch(vec![item]);
    let output = model.forward(batch.images);
    let predicted = output.argmax(1).flatten::<1>(0, 1).into_scalar();

    println!("Predicted {} Expected {}", predicted, label);
}

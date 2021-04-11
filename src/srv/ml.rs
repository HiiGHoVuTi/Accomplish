extern crate csv;
use gbdt::config::Config;
use gbdt::decision_tree::{DataVec, PredVec};
use gbdt::gradient_boost::GBDT;
use gbdt::input::{InputFormat, load};


pub fn train() {
	let mut cfg = Config::new();
	cfg.set_feature_size(3);
	cfg.set_max_depth(5);
	cfg.set_iterations(50);
	cfg.set_shrinkage(0.1);
	cfg.set_loss("LogLikelyhood");
	cfg.set_debug(true);
	cfg.set_data_sample_ratio(1.0);
	cfg.set_feature_sample_ratio(1.0);
	cfg.set_training_optimization_level(2);
	let train_path = "activity.csv";
	let mut input_format = InputFormat::csv_format();
	input_format.set_feature_size(5);
	input_format.set_label_index(2);
	let mut input_dv: DataVec = load(train_path, input_format).expect("failed to load data");
	let mut gdbt = GBDT::new(&(cfg));
	gdbt.fit(&mut (input_dv));
	gdbt.save_model("xgb.model").expect("failed to save model");
	let model = GBDT::load_model("xgb.model").expect("failed to load the model");
	let predicted: PredVec = model.predict(&(input_dv));
	println!("{:#?}", predicted);
}


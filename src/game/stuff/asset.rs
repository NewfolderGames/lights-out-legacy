use std::any::Any;

pub trait StuffAsset : Any {

	const NAME: &'static str;

}
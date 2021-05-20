pub trait Stuff {

	type Asset;
	type Manager;

	fn get_asset(&self) -> &Self::Asset;

}
/// A pop data.
pub struct Pop {

	name: String,
	infection: f64,
	job: Option<String>,
	level: i32,
	experience: f64,

}

impl Pop {

	/// Creates a new pop.
	pub fn new(name: String, job: Option<String>, infection: f64) -> Self {

		Self {

			name,
			infection,
			job,
			experience: 0f64,
			level: 0,

		}

	}

	/// Adds the pop's infection
	pub fn add_infection(&mut self, amount: f64) {

		self.infection += amount;
		if self.infection > 1f64 { self.infection = 1f64 }

	}

	/// Returns the pop's name.
	pub fn get_name(&self) -> &String {

		&self.name

	}

	/// Returns the pop's infection.
	pub fn get_infection(&self) -> f64 {

		self.infection

	}

	/// Returns the pop's level.
	pub fn get_level(&self) -> i32 {

		self.level

	}

	/// Returns the pop's job.
	pub fn get_job(&self) -> Option<&String> {

		self.job.as_ref()

	}

	/// Heals the pop's infection.
	pub fn heal_infection(&mut self, amount: f64) {

		self.infection -= amount;
		if self.infection < 0f64 { self.infection = 0f64 }

	}

	/// Sets the pop's job.
	pub fn set_job(&mut self, job: Option<String>) {

		self.job = job;

	}

}
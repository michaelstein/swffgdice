
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Face {
	success: i32,
	advantage: i32,
	triumph: i32,
	failure: i32,
	threat: i32,
	despair: i32,
	light: i32,
	dark: i32
}

impl Face {
	pub fn new() -> Self {
		Self {
			success: 0,
			advantage: 0,
			triumph: 0,
			failure: 0,
			threat: 0,
			despair: 0,
			light: 0,
			dark: 0
		}
	}

	pub fn positive(success: i32, advantage: i32, triumph: i32) -> Self {
		Self {
			success: success,
			advantage: advantage,
			triumph: triumph,
			failure: 0,
			threat: 0,
			despair: 0,
			light: 0,
			dark: 0
		}
	}
	
	pub fn negative(failure: i32, threat: i32, despair: i32) -> Self {
		Self {
			success: 0,
			advantage: 0,
			triumph: 0,
			failure: failure,
			threat: threat,
			despair: despair,
			light: 0,
			dark: 0
		}
	}
	
	pub fn force(light: i32, dark: i32) -> Self {
		Self {
			success: 0,
			advantage: 0,
			triumph: 0,
			failure: 0,
			threat: 0,
			despair: 0,
			light: light,
			dark: dark
		}
	}

	pub fn balance(&mut self) {
		if self.success < self.failure {
			self.failure -= self.success;
			self.success = 0;
		} else {
			self.success -= self.failure;
			self.failure = 0;
		}

		if self.advantage < self.threat {
			self.threat -= self.advantage;
			self.advantage = 0;
		} else {
			self.advantage -= self.threat;
			self.threat = 0;
		}
	}
}

impl std::ops::Add for Face {
    type Output = Self;

	fn add(self, _rhs: Self) -> Self {
		Self {
			success: self.success + _rhs.success,
			advantage: self.advantage + _rhs.advantage,
			triumph: self.triumph + _rhs.triumph,
			failure: self.failure + _rhs.failure,
			threat: self.threat + _rhs.threat,
			despair: self.despair + _rhs.despair,
			light: self.light + _rhs.light,
			dark: self.dark + _rhs.dark
		}
	}
}

impl std::ops::AddAssign for Face {
	fn add_assign(&mut self, rhs: Self) {
		self.success += rhs.success;
		self.advantage += rhs.advantage;
		self.triumph += rhs.triumph;
		self.failure += rhs.failure;
		self.threat += rhs.threat;
		self.despair += rhs.despair;
		self.light += rhs.light;
		self.dark += rhs.dark;
	}
}

impl std::fmt::Display for Face {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut ret: Vec<String> = vec![];

		if self.success > 0 {
			let plural = if self.success >= 2 { "es" } else { "" };
			let s = format!("{} Success{}", self.success, plural);
			ret.push(s);
		}
		
		if self.failure > 0 {
			let plural = if self.failure >= 2 { "s" } else { "" };
			let s = format!("{} Failure{}", self.failure, plural);
			ret.push(s);
		}

		if self.advantage > 0 {
			let s = format!("{} Advantage", self.advantage);
			ret.push(s);
		}

		if self.threat > 0 {
			let s = format!("{} Threat", self.threat);
			ret.push(s);
		}

		if self.triumph > 0 {
			let s = format!("{} Triumph", self.triumph);
			ret.push(s);
		}

		if self.despair > 0 {
			let s = format!("{} Despair", self.despair);
			ret.push(s);
		}

		if self.light > 0 {
			let s = format!("{} Light", self.light);
			ret.push(s);
		}

		if self.dark > 0 {
			let s = format!("{} Dark", self.dark);
			ret.push(s);
		}

		write!(f, "{}", ret.join(", "))
	}
}
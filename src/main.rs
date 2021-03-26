use std::env;
use rand::thread_rng;
use rand::prelude::SliceRandom;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Face {
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
	fn new() -> Self {
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

	fn positive(success: i32, advantage: i32, triumph: i32) -> Self {
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
	
	fn negative(failure: i32, threat: i32, despair: i32) -> Self {
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
	
	fn force(light: i32, dark: i32) -> Self {
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

	fn balance(&mut self) {
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

	fn to_string(&self) -> String {
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

		ret.join(", ")
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

fn main() {
	let args: Vec<String> = env::args().collect();
	let re = Regex::new(r"([+-]?)([0-9]*)([bapsdcf])").unwrap();

	let mut numBoost = 0;
	let mut numAbility = 0;
	let mut numProficiency = 0;
	let mut numSetback = 0;
	let mut numDifficulty = 0;
	let mut numChallenge = 0;
	let mut numForce = 0;

	let mut i = 1;
	while i < args.len() {
		let arg = &args[i];

		for cap in re.captures_iter(&arg) {
			let sign = &cap[1];
			let num_str = &cap[2];
			let die = &cap[3];

			let num = match num_str.parse::<i32>() {
				Ok(val) => if sign == "-" { -val } else { val },
				Err(_e) => if sign == "-" { -1 } else { 1 }
			};

			// bapsdcf
			match die {
				"b" => numBoost += num,
				"a" => numAbility += num,
				"p" => numProficiency += num,
				"s" => numSetback += num,
				"d" => numDifficulty += num,
				"c" => numChallenge += num,
				"f" => numForce += num,
				_ => ()
			}
		}

		i += 1;
	}

	let boostDie = vec![
		Face::new(),
		Face::new(),
		Face::positive(1, 0, 0),
		Face::positive(1, 1, 0),
		Face::positive(0, 2, 0),
		Face::positive(0, 1, 0)
	];

	let abilityDie = vec![
		Face::new(),
		Face::positive(1, 0, 0),
		Face::positive(1, 0, 0),
		Face::positive(2, 0, 0),
		Face::positive(0, 1, 0),
		Face::positive(0, 1, 0),
		Face::positive(1, 1, 0),
		Face::positive(0, 2, 0)
	];

	let proficiencyDie = vec![
		Face::new(),
		Face::positive(1, 0, 0),
		Face::positive(1, 0, 0),
		Face::positive(2, 0, 0),
		Face::positive(2, 0, 0),
		Face::positive(0, 1, 0),
		Face::positive(1, 1, 0),
		Face::positive(1, 1, 0),
		Face::positive(1, 1, 0),
		Face::positive(0, 2, 0),
		Face::positive(0, 2, 0),
		Face::positive(1, 0, 1)
	];

	let setbackDie = vec![
		Face::new(),
		Face::new(),
		Face::negative(1, 0, 0),
		Face::negative(1, 0, 0),
		Face::negative(0, 1, 0),
		Face::negative(0, 1, 0)
	];

	let difficultyDie = vec![
		Face::new(),
		Face::negative(1, 0, 0),
		Face::negative(2, 0, 0),
		Face::negative(0, 1, 0),
		Face::negative(0, 1, 0),
		Face::negative(0, 1, 0),
		Face::negative(0, 2, 0),
		Face::negative(1, 1, 0)
	];

	let challengeDie = vec![
		Face::new(),
		Face::negative(1, 0, 0),
		Face::negative(1, 0, 0),
		Face::negative(2, 0, 0),
		Face::negative(2, 0, 0),
		Face::negative(0, 1, 0),
		Face::negative(0, 1, 0),
		Face::negative(1, 1, 0),
		Face::negative(1, 1, 0),
		Face::negative(0, 2, 0),
		Face::negative(0, 2, 0),
		Face::negative(0, 0, 1)
	];

	let forceDie = vec![
		Face::force(0, 1),
		Face::force(0, 1),
		Face::force(0, 1),
		Face::force(0, 1),
		Face::force(0, 1),
		Face::force(0, 1),
		Face::force(0, 2),
		Face::force(1, 0),
		Face::force(1, 0),
		Face::force(2, 0),
		Face::force(2, 0),
		Face::force(2, 0),
	];

	let mut rng = thread_rng();
	let mut result = Face::new();
	
	while numBoost > 0 {
		if let Some(face) = boostDie.choose(&mut rng) {
			println!("{}", face.to_string());
			result += *face;
		};
		numBoost -= 1;
	}
	
	while numAbility > 0 {
		if let Some(face) = abilityDie.choose(&mut rng) {
			println!("{}", face.to_string());
			result += *face;
		};
		numAbility -= 1;
	}
	
	while numProficiency > 0 {
		if let Some(face) = proficiencyDie.choose(&mut rng) {
			println!("{}", face.to_string());
			result += *face;
		};
		numProficiency -= 1;
	}
	
	while numSetback > 0 {
		if let Some(face) = setbackDie.choose(&mut rng) {
			println!("{}", face.to_string());
			result += *face;
		};
		numSetback -= 1;
	}
	
	while numDifficulty > 0 {
		if let Some(face) = difficultyDie.choose(&mut rng) {
			println!("{}", face.to_string());
			result += *face;
		};
		numDifficulty -= 1;
	}
	
	while numChallenge > 0 {
		if let Some(face) = challengeDie.choose(&mut rng) {
			println!("{}", face.to_string());
			result += *face;
		};
		numChallenge -= 1;
	}
	
	while numForce > 0 {
		if let Some(face) = forceDie.choose(&mut rng) {
			println!("{}", face.to_string());
			result += *face;
		};
		numForce -= 1;
	}

	result.balance();
	println!("{}", result.to_string());
}

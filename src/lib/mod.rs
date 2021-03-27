#![allow(non_snake_case)]

use rand::thread_rng;
use rand::prelude::SliceRandom;
use regex::Regex;

mod face;
pub use face::Face;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct DicePool {
	boost: i32,
	ability: i32,
	proficiency: i32,
	setback: i32,
	difficulty: i32,
	challenge: i32,
	force: i32
}

pub fn roll(args: &Vec<String>) -> Face {
	let re = Regex::new(r"([+-]?)([0-9]*)([bapsdcf])").unwrap();

	let mut dice = DicePool {
		boost: 0,
		ability: 0,
		proficiency: 0,
		setback: 0,
		difficulty: 0,
		challenge: 0,
		force: 0
	};

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
				"b" => dice.boost += num,
				"a" => dice.ability += num,
				"p" => dice.proficiency += num,
				"s" => dice.setback += num,
				"d" => dice.difficulty += num,
				"c" => dice.challenge += num,
				"f" => dice.force += num,
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
	let mut width = 0;
	
	while dice.boost > 0 {
		if let Some(face) = boostDie.choose(&mut rng) {
			let s = face.to_string();
			width = std::cmp::max(width, s.len());
			println!("{}", s);
			result += *face;
		};
		dice.boost -= 1;
	}
	
	while dice.ability > 0 {
		if let Some(face) = abilityDie.choose(&mut rng) {
			let s = face.to_string();
			width = std::cmp::max(width, s.len());
			println!("{}", s);
			result += *face;
		};
		dice.ability -= 1;
	}
	
	while dice.proficiency > 0 {
		if let Some(face) = proficiencyDie.choose(&mut rng) {
			let s = face.to_string();
			width = std::cmp::max(width, s.len());
			println!("{}", s);
			result += *face;
		};
		dice.proficiency -= 1;
	}
	
	while dice.setback > 0 {
		if let Some(face) = setbackDie.choose(&mut rng) {
			let s = face.to_string();
			width = std::cmp::max(width, s.len());
			println!("{}", s);
			result += *face;
		};
		dice.setback -= 1;
	}
	
	while dice.difficulty > 0 {
		if let Some(face) = difficultyDie.choose(&mut rng) {
			let s = face.to_string();
			width = std::cmp::max(width, s.len());
			println!("{}", s);
			result += *face;
		};
		dice.difficulty -= 1;
	}
	
	while dice.challenge > 0 {
		if let Some(face) = challengeDie.choose(&mut rng) {
			let s = face.to_string();
			width = std::cmp::max(width, s.len());
			println!("{}", s);
			result += *face;
		};
		dice.challenge -= 1;
	}
	
	while dice.force > 0 {
		if let Some(face) = forceDie.choose(&mut rng) {
			let s = face.to_string();
			width = std::cmp::max(width, s.len());
			println!("{}", s);
			result += *face;
		};
		dice.force -= 1;
	}

	result
}


mod capi {
	#[no_mangle]
	pub extern "C" fn roll(arg: *const std::os::raw::c_char) -> crate::Face {
		let mut ret = crate::Face::new();

		unsafe {
			let cstr = std::ffi::CStr::from_ptr(arg);
			if let Ok(str) = cstr.to_str() {
				let args = vec![str.to_string()];
				ret = crate::roll(&args);
			}
		}

		ret
	}
}

use serde::{Deserialize, Serialize}; // json crate
use std::env; // for cwd
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


// JSON stuff adapted from https://blog.logrocket.com/json-and-rust-why-serde_json-is-the-top-choice/
// todo: figure out how to toggle lint warnings (#[allow(non_snake_case)])
// todo: String in these structs should be an enum
#[derive(Debug, Deserialize, Serialize)]
struct Substat {
	key: String,
	value: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Artifact {
	name: String,
	level: usize,
	rarity: usize,
	mainStatKey: String,
	location: String,
	lock: bool,
	substats: Vec<Substat>,//array
	frameIndex: usize,
	setKey: String,
	slotKey: String
}

#[derive(Debug, Deserialize, Serialize)]
struct GOODData {
	format: String,
	source: String,
	version: usize,
	artifacts: Vec<Artifact>
}


// goal of this struct is to enable stuff like:
// todo: use flags system? pyro | burst | vape
// stats = arti1 + arti2 + raiden_e
// calcDamage(600%, atk, pyro, burst)
struct StatBlock {
	// todo: f32 sufficient for all fields?
	// naming convention follows GOOD format
	// see https://frzyc.github.io/genshin-optimizer/#/doc/StatKey
	hp: f32,	 			//HP
	hp_: f32,	 			//HP%
	atk: f32,	 			//ATK
	atk_: f32,	 			//ATK%
	def: f32,	 			//DEF
	def_: f32,	 			//DEF%
	eleMas: f32,	 		//Elemental Mastery
	enerRech_: f32,	 		//Energy Recharge%
	heal_: f32,	 			//Healing Bonus%
	critRate_: f32,	 		//Crit Rate
	critDMG_: f32,	 		//Crit DMG%
	physical_dmg_: f32,	 	//Physical DMG Bonus%
	anemo_dmg_: f32,	 	//Anemo DMG Bonus%
	geo_dmg_: f32,	 		//Geo DMG Bonus%
	electro_dmg_: f32,	 	//Electro DMG Bonus%
	hydro_dmg_: f32,	 	//Hydro DMG Bonus%
	pyro_dmg_: f32,	 		//Pyro DMG Bonus%
	cryo_dmg_: f32,	 		//Cryo DMG Bonus%
	dendro_dmg_: f32,	 	//Dendro DMG Bonus%
	// other damage modifiers that don't come from artifacts
	// stuff like Raiden E or Shimenewa 4pc bonus
	// todo: vape, catalyse? 
	normal_dmg_: f32,		// Normal Attack DMG Bonus%
	charged_dmg_: f32,		// Charged Attack DMG Bonus%
	skill_dmg_: f32,		// Skill DMG Bonus%
	burst_dmg_: f32,		// Normal Attack DMG Bonus%
	all_dmg_: f32,			// All DMG Bonus%
}

/*
struct GenshinJsonStruct {
	data:,
	image:,
	curve:,
	version:,
	index:,
	stats:,
	url:,
}

struct GenshinDatabase {
	data,
	image,
	curve,
	version,
	index,
	stats,
	url
}
*/

struct Character {

}

/*
// see wiki for formulas:
// https://genshin-impact.fandom.com/wiki/Damage
fn calcDamage() {

}
*/

fn readFile(filename: &str) -> String {
	// following section from https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
	let path = Path::new(&filename);
	let display = path.display();
	// Open the path in read-only mode, returns `io::Result<File>`
	let mut file = match File::open(&path) {
		Err(why) => panic!("couldn't open {}: {}", display, why),
		Ok(file) => file,
	};
	// Read the file contents into a string, returns `io::Result<usize>`
	let mut jsonString = String::new();
	match file.read_to_string(&mut jsonString) {
		Err(why) => panic!("couldn't read {}: {}", display, why),
		Ok(_) => (),
	}
	return jsonString;
}


fn main() -> std::io::Result<()> {
	let path = env::current_dir()?;
	println!("The current directory is {}", path.display());



	// parse artifact JSON
	let artifactPath = "./data/2023-01-15 15-31-44.ocr3.json";
	let artifactJsonString = readFile(artifactPath);
	let goodData: GOODData = serde_json::from_str(&artifactJsonString).unwrap();
	println!("First artifact is: {}", serde_json::to_string(&goodData.artifacts[0])?);

	// parse db JSON (unstructured)
	let dbPath = "./data/data.min.json";
	let dbJsonString = readFile(dbPath);
	// todo: more concise type I can use?
	let db: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&dbJsonString).unwrap();
	println!("{}", db["stats"]["weapons"]["dullblade"]["base"]["attack"]);
	for k in db.keys() {
		println!("{}", k);
	}
	//println!("db keys are {}", db.keys());
	Ok(()) // todo: what's this do?
}

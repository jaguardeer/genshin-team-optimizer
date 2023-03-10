#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)] // todo: temp, learn proper fix (serde variant attributes)
use serde::{Deserialize, Serialize}; // json crate
use std::env; // for cwd
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use itertools::Itertools;

// todo: learn Rust fundamentals
// 	- lifetimes
// 	- ownership
// 	- what is borrowing?


// JSON stuff adapted from https://blog.logrocket.com/json-and-rust-why-serde_json-is-the-top-choice/
// todo: figure out how to toggle lint warnings (#[allow(non_snake_case)]) OR use https://serde.rs/variant-attrs.html
// todo: String in these structs should be an enum
// todo: StatBlock can be implemented as a macro

//****** EXTERNALLY MANDATED DATA LAYOUTS

//****** GOOD Format: https://frzyc.github.io/genshin-optimizer/#/doc
#[derive(Debug, Deserialize, Serialize)]
struct Substat {
	key: StatKey,
	value: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Artifact {
	name: String,
	level: usize,
	rarity: usize,
	mainStatKey: StatKey,
	location: String,
	lock: bool,
	substats: Vec<Substat>,//array
	frameIndex: usize,
	setKey: String,
	slotKey: SlotKey
}

#[derive(Debug, Deserialize, Serialize)]
struct GOODData {
	format: String,
	source: String,
	version: usize,
	artifacts: Vec<Artifact>
}

// todo: can I separate MainStat and Substat keys?
#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
enum StatKey {
	hp,	 				//HP
	hp_,	 			//HP%
	atk,	 			//ATK
	atk_,	 			//ATK%
	def,	 			//DEF
	def_,	 			//DEF%
	eleMas,	 			//Elemental Mastery
	enerRech_,	 		//Energy Recharge%
	heal_,	 			//Healing Bonus%
	critRate_,	 		//Crit Rate
	critDMG_,	 		//Crit DMG%
	physical_dmg_,	 	//Physical DMG Bonus%
	anemo_dmg_,	 		//Anemo DMG Bonus%
	geo_dmg_,	 		//Geo DMG Bonus%
	electro_dmg_,	 	//Electro DMG Bonus%
	hydro_dmg_,	 		//Hydro DMG Bonus%
	pyro_dmg_,	 		//Pyro DMG Bonus%
	cryo_dmg_,	 		//Cryo DMG Bonus%
	dendro_dmg_,	 	//Dendro DMG Bonus%
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
enum SlotKey {
	flower,
	plume,
	sands,
	goblet,
	circlet
}

//****** GENSHIN DATABASE FORMAT: https://github.com/theBowja/genshin-db
// CURVE SECTION
#[derive(Debug, Deserialize, Serialize)]
enum WeaponCurveType {
	GROW_CURVE_ATTACK_101,
	GROW_CURVE_ATTACK_102,
	GROW_CURVE_ATTACK_103,
	GROW_CURVE_ATTACK_104,
	GROW_CURVE_ATTACK_105,
	GROW_CURVE_CRITICAL_101,
	GROW_CURVE_ATTACK_201,
	GROW_CURVE_ATTACK_202,
	GROW_CURVE_ATTACK_203,
	GROW_CURVE_ATTACK_204,
	GROW_CURVE_ATTACK_205,
	GROW_CURVE_CRITICAL_201,
	GROW_CURVE_ATTACK_301,
	GROW_CURVE_ATTACK_302,
	GROW_CURVE_ATTACK_303,
	GROW_CURVE_ATTACK_304,
	GROW_CURVE_ATTACK_305,
	GROW_CURVE_CRITICAL_301,
}

#[derive(Debug, Deserialize, Serialize)]
enum CharacterCurveType {
	GROW_CURVE_HP_S4,
	GROW_CURVE_ATTACK_S4,
	GROW_CURVE_HP_S5,
	GROW_CURVE_ATTACK_S5,
}

#[derive(Debug, Deserialize, Serialize)]
enum EnemyCurveType {
	GROW_CURVE_HP,
	GROW_CURVE_ATTACK,
	GROW_CURVE_DEFENSE,
	GROW_CURVE_KILL_EXP,
	GROW_CURVE_HP_LITTLEMONSTER,
	GROW_CURVE_MHP,
	GROW_CURVE_MATK,
	GROW_CURVE_HP_2,
	GROW_CURVE_ATTACK_2,
}

#[derive(Debug, Deserialize, Serialize)]
struct CharacterCurves {
	GROW_CURVE_HP_S4: f32,
	GROW_CURVE_ATTACK_S4: f32,
	GROW_CURVE_HP_S5: f32,
	GROW_CURVE_ATTACK_S5: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct WeaponCurves {
	GROW_CURVE_ATTACK_101: f32,
	GROW_CURVE_ATTACK_102: f32,
	GROW_CURVE_ATTACK_103: f32,
	GROW_CURVE_ATTACK_104: f32,
	GROW_CURVE_ATTACK_105: f32,
	GROW_CURVE_CRITICAL_101: f32,
	GROW_CURVE_ATTACK_201: f32,
	GROW_CURVE_ATTACK_202: f32,
	GROW_CURVE_ATTACK_203: f32,
	GROW_CURVE_ATTACK_204: f32,
	GROW_CURVE_ATTACK_205: f32,
	GROW_CURVE_CRITICAL_201: f32,
	GROW_CURVE_ATTACK_301: f32,
	GROW_CURVE_ATTACK_302: f32,
	GROW_CURVE_ATTACK_303: f32,
	GROW_CURVE_ATTACK_304: f32,
	GROW_CURVE_ATTACK_305: f32,
	GROW_CURVE_CRITICAL_301: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct EnemyCurves {
	GROW_CURVE_HP: f32,
	GROW_CURVE_ATTACK: f32,
	GROW_CURVE_DEFENSE: f32,
	GROW_CURVE_KILL_EXP: f32,
	GROW_CURVE_HP_LITTLEMONSTER: f32,
	GROW_CURVE_MHP: f32,
	GROW_CURVE_MATK: f32,
	GROW_CURVE_HP_2: f32,
	GROW_CURVE_ATTACK_2: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct CurveDB {
	// todo: do I really have to use hashmap? the db json uses (int-as-string, value) pairs to represent an array
	characters:  HashMap<i64, CharacterCurves>,//[CharacterCurves; 100],
	weapons:  HashMap<i64, WeaponCurves>,//[WeaponCurves; 100],
	enemies:  HashMap<i64, EnemyCurves>,//[EnemyCurves; 200],
}

#[derive(Debug, Deserialize, Serialize)]
struct StatsDB {
	characters: serde_json::Value, 
	weapons: serde_json::Value, 
	talents: serde_json::Value, 
	enemies: serde_json::Value, 
}

enum SpecializedKey {
	FIGHT_PROP_CRITICAL_HURT,
	FIGHT_PROP_HEAL_ADD,
	FIGHT_PROP_ATTACK_PERCENT,
	FIGHT_PROP_ELEMENT_MASTERY,
	FIGHT_PROP_HP_PERCENT,
	FIGHT_PROP_CHARGE_EFFICIENCY,
	FIGHT_PROP_CRITICAL,
	FIGHT_PROP_PHYSICAL_ADD_HURT,
	FIGHT_PROP_ELEC_ADD_HURT,
	FIGHT_PROP_ROCK_ADD_HURT,
	FIGHT_PROP_FIRE_ADD_HURT,
	FIGHT_PROP_WATER_ADD_HURT,
	FIGHT_PROP_DEFENSE_PERCENT,
	FIGHT_PROP_ICE_ADD_HURT,
	FIGHT_PROP_WIND_ADD_HURT,
	FIGHT_PROP_GRASS_ADD_HURT,
}

struct PromotionEntry {
	maxlevel: i32,
	hp: f32,
	attack: f32,
	defense: f32,
	specialized: f32,
}

// TODO
#[derive(Debug, Deserialize, Serialize)]
struct CharacterEntry {
	base: serde_json::Value,
	curve: serde_json::Value,
	specialized: serde_json::Value,
	promotion: serde_json::Value//PromotionEntry,
}

// todo: all fields are Value until I can decide what they really are
#[derive(Debug, Deserialize, Serialize)]
struct GenshinDatabase {
	data: serde_json::Value,
	image: serde_json::Value,
	curve: CurveDB,// todo
	version: serde_json::Value,
	index: serde_json::Value,
	stats: StatsDB,
	url: serde_json::Value,
}


// ****** MY STRUCTS
// goal of this struct is to enable stuff like:
// todo: use flags system? pyro | burst | vape
// stats = arti1 + arti2 + raiden_e
// calcDamage(600%, atk, pyro, burst)
use std::ops::Add;

#[derive(Debug, Default, Copy, Clone)]
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
	// base stats todo: should this be in here or in a different struct?
	base_hp: f32,
	base_atk: f32,
	base_def: f32,
}

impl Add for StatBlock {
	type Output = StatBlock;

	fn add(self, other: StatBlock) -> StatBlock {
		StatBlock {
			hp: self.hp + other.hp,
			hp_: self.hp_ + other.hp_,
			atk: self.atk + other.atk,
			atk_: self.atk_ + other.atk_,
			def: self.def + other.def,
			def_: self.def_ + other.def_,
			eleMas: self.eleMas + other.eleMas,
			enerRech_: self.enerRech_ + other.enerRech_,
			heal_: self.heal_ + other.heal_,
			critRate_: self.critRate_ + other.critRate_,
			critDMG_: self.critDMG_ + other.critDMG_,
			physical_dmg_: self.physical_dmg_ + other.physical_dmg_,
			anemo_dmg_: self.anemo_dmg_ + other.anemo_dmg_,
			geo_dmg_: self.geo_dmg_ + other.geo_dmg_,
			electro_dmg_: self.electro_dmg_ + other.electro_dmg_,
			hydro_dmg_: self.hydro_dmg_ + other.hydro_dmg_,
			pyro_dmg_: self.pyro_dmg_ + other.pyro_dmg_,
			cryo_dmg_: self.cryo_dmg_ + other.cryo_dmg_,
			dendro_dmg_: self.dendro_dmg_ + other.dendro_dmg_,
			normal_dmg_: self.normal_dmg_ + other.normal_dmg_,
			charged_dmg_: self.charged_dmg_ + other.charged_dmg_,
			skill_dmg_: self.skill_dmg_ + other.skill_dmg_,
			burst_dmg_: self.burst_dmg_ + other.burst_dmg_,
			all_dmg_: self.all_dmg_ + other.all_dmg_,
			base_hp: self.base_hp + other.base_hp,
			base_atk: self.base_atk + other.base_atk,
			base_def: self.base_def + other.base_def,
		}
	}
}

fn setField(statBlock: &mut StatBlock, key: StatKey, val: f32) {
	match key {
		StatKey::hp => statBlock.hp = val,
		StatKey::hp_ => statBlock.hp_ = val,
		StatKey::atk => statBlock.atk = val,
		StatKey::atk_ => statBlock.atk_ = val,
		StatKey::def => statBlock.def = val,
		StatKey::def_ => statBlock.def_ = val,
		StatKey::eleMas => statBlock.eleMas = val,
		StatKey::enerRech_ => statBlock.enerRech_ = val,
		StatKey::heal_ => statBlock.heal_ = val,
		StatKey::critRate_ => statBlock.critRate_ = val,
		StatKey::critDMG_ => statBlock.critDMG_ = val,
		StatKey::physical_dmg_ => statBlock.physical_dmg_ = val,
		StatKey::anemo_dmg_ => statBlock.anemo_dmg_ = val,
		StatKey::geo_dmg_ => statBlock.geo_dmg_ = val,
		StatKey::electro_dmg_ => statBlock.electro_dmg_ = val,
		StatKey::hydro_dmg_ => statBlock.hydro_dmg_ = val,
		StatKey::pyro_dmg_ => statBlock.pyro_dmg_ = val,
		StatKey::cryo_dmg_ => statBlock.cryo_dmg_ = val,
		StatKey::dendro_dmg_ => statBlock.dendro_dmg_ = val,
	}
}

fn getMainstatValue(mainStatKey: StatKey, level: i8) {

}

fn statBlockFromGoodArtifact(goodArtifact: &Artifact) -> StatBlock {
	//println!("hey");
	let mut block = StatBlock::default();
	setField(&mut block, goodArtifact.mainStatKey, 1337.0);
	for substat in &goodArtifact.substats {
		setField(&mut block, substat.key, substat.value);
	}
	return block;
}

// todo
struct CharacterBase {
/*
	baseStats: BaseStats,
	curveTypes: CurveTypes,
	specialType: StatKey,
	promotionStats: PromotionStats,
*/
}

struct BaseStats {
	hp: f32,
	atk: f32,
	def: f32,
	critRate_: f32,
	critDMG_: f32,
}

struct CurveTypes {
	hp: CharacterCurveType,
	atk: CharacterCurveType,
	def: CharacterCurveType,
}

struct CharacterInstance {
	level: i32,
	characterBase: CharacterBase,
}

struct HitInfo {

}

/*
baseStats = charStats + weaponStats
*/


// see wiki for formulas:
// https://genshin-impact.fandom.com/wiki/Damage
fn calcDamage() {

}

//fn calculateStat()


fn readFile(filename: &str) -> String {
	// following section from https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
	let path = Path::new(&filename);
	let display = path.display();
	// Open the path in read-only mode, returns `io::Result<File>`
	let mut file = File::open(&path).unwrap_or_else(|why| panic!("couldn't open {}: {}", display, why));
	// Read the file contents into a string, returns `io::Result<usize>`
	let mut jsonString = String::new();
	file.read_to_string(&mut jsonString).unwrap_or_else(|why| panic!("couldn't read {}: {}", display, why));
	return jsonString;
}

fn calcEnemyDefence(enemyLevel: f32) -> f32 {
	5.0 * enemyLevel + 500.0
}

fn calcDefenceMult(defence: f32, attackerLevel: f32) -> f32 {
	defence / (defence + 5.0 * attackerLevel + 500.0)
}

fn calcResistMult(resistance: f32) -> f32 {
	match resistance {
		x if x < 0.0 	=> 1.0 - (resistance / 2.0),
		x if x >= 0.75 	=> 1.0 / (4.0 * resistance + 1.0),
		_ 				=> 1.0 - resistance,
	}
}

fn example_xiangling() {
	// skill level
	// enemy stats
}


fn main() -> std::io::Result<()> {
	// pwd
	let path = env::current_dir()?;
	println!("The current directory is {}", path.display());

	// parse artifact JSON
	let artifactPath = "./data/2023-01-15 15-31-44.ocr3.json";
	let artifactJsonString = readFile(artifactPath);
	let goodData: GOODData = serde_json::from_str(&artifactJsonString).expect("parsing artifacts");
	println!("First artifact is: {}", serde_json::to_string(&goodData.artifacts[0])?);

	// select 5 star artis only
	let mut artifacts: Vec<_> = goodData.artifacts.iter()
		.filter(|a| a.rarity == 5)
		.collect();
	// sort by slot
	artifacts.sort_unstable_by(|a, b| b.slotKey.cmp(&a.slotKey));
	// groupby slot (todo: can i use map() or similiar instead of for loop?)
	let artifacts: HashMap<_, _> = artifacts.iter()
		.group_by(|arti| &arti.slotKey)
		.into_iter()
		.map(|(k, g)| (k, g.map(|arti| statBlockFromGoodArtifact(&arti)).collect::<Vec<_>>()))
		.collect();
	// debug: print
	for (k, v) in &artifacts {
		println!("{k:?}: {:?}", v.len());
	}
	/*
	//let x = artifacts.get(&SlotKey::sands).unwrap();
	//println!("{:?}", x[0]);
	// calc combinations BIG todo
	use std::time::{Instant};
	let startTime = Instant::now();
	let mut i: usize = 0;
	let mut bestStats = StatBlock::default();
	for s in &artifacts[&SlotKey::sands] {
		for c in &artifacts[&SlotKey::circlet] {
			for p in &artifacts[&SlotKey::plume] {
				for f in &artifacts[&SlotKey::flower] {
					for g in &artifacts[&SlotKey::goblet] {
						let stats = *s + *c + *p + *f + *g;
						if stats.atk_ > bestStats.atk {
							bestStats = stats;
						}
						i += 1;
						if i % 100_000_000 == 0 { println!("{i}") }
					}
				}
			}
		}
	}
	println!("{:?}", bestStats);
	println!("{i} combinations took {:.7}", startTime.elapsed().as_secs_f64());
	*/
	/*
	let artiGroups: HashMap<_, Vec<_>> = artifacts.iter()
		.group_by(|arti| arti.slotKey)
        .into_iter()
		.map(|(ge0, group)| (ge0, group.cloned().collect()))
    	.collect();
	println!("{:?}", artiGroups);
	*/
	// test StatBlock stuff
	//let x = statBlockFromGoodArtifact(&goodData.artifacts[0]);
	//println!("{:?}", x);

	// parse db JSON 
	let dbPath = "./data/data.min.json";
	let dbJsonString = readFile(dbPath);

	// todo: more concise type I can use?
	let db: GenshinDatabase = serde_json::from_str(&dbJsonString).expect("parsing GenshinDatabase JSON");
	println!("{}", db.stats.weapons["dullblade"]["base"]["attack"]);
	println!("{}", db.curve.characters[&1].GROW_CURVE_HP_S4);
	let foo = &db.curve.characters[&1]; // todo: learn borrowing
	println!("{}", foo.GROW_CURVE_HP_S4);
	match db.curve.characters.get(&0) {
		None => panic!("couldn't get it"),
		Some(curve) => println!("val is {}", curve.GROW_CURVE_HP_S4),
	}

	Ok(())
}

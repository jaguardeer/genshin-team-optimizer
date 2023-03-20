//****** GOOD Format: https://frzyc.github.io/genshin-optimizer/#/doc

#![allow(non_snake_case, non_camel_case_types)]

use serde::{Deserialize, Serialize}; // json crate

#[derive(Debug, Deserialize, Serialize)]
pub struct Substat {
	pub key: StatKey,
	pub value: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Artifact {
	pub name: String,
	pub level: usize,
	pub rarity: usize,
	pub mainStatKey: StatKey,
	pub location: String,
	pub lock: bool,
	pub substats: Vec<Substat>,//array
	pub frameIndex: usize,
	pub setKey: String,
	pub slotKey: SlotKey
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GOOD_Data {
	pub format: String,
	pub source: String,
	pub version: usize,
	pub artifacts: Vec<Artifact>
}

// todo: can I separate MainStat and Substat keys?
#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum StatKey {
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
pub enum SlotKey {
	flower,
	plume,
	sands,
	goblet,
	circlet
}
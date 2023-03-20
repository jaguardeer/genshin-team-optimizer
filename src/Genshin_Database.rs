//****** GENSHIN DATABASE FORMAT: https://github.com/theBowja/genshin-db

#![allow(non_snake_case, non_camel_case_types)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize}; // json crate

// CURVE SECTION

#[derive(Debug, Deserialize, Serialize)]
pub enum WeaponCurveType {
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
pub enum CharacterCurveType {
	GROW_CURVE_HP_S4,
	GROW_CURVE_ATTACK_S4,
	GROW_CURVE_HP_S5,
	GROW_CURVE_ATTACK_S5,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EnemyCurveType {
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
pub struct CharacterCurves {
	pub GROW_CURVE_HP_S4: f32,
	pub GROW_CURVE_ATTACK_S4: f32,
	pub GROW_CURVE_HP_S5: f32,
	pub GROW_CURVE_ATTACK_S5: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeaponCurves {
	pub GROW_CURVE_ATTACK_101: f32,
	pub GROW_CURVE_ATTACK_102: f32,
	pub GROW_CURVE_ATTACK_103: f32,
	pub GROW_CURVE_ATTACK_104: f32,
	pub GROW_CURVE_ATTACK_105: f32,
	pub GROW_CURVE_CRITICAL_101: f32,
	pub GROW_CURVE_ATTACK_201: f32,
	pub GROW_CURVE_ATTACK_202: f32,
	pub GROW_CURVE_ATTACK_203: f32,
	pub GROW_CURVE_ATTACK_204: f32,
	pub GROW_CURVE_ATTACK_205: f32,
	pub GROW_CURVE_CRITICAL_201: f32,
	pub GROW_CURVE_ATTACK_301: f32,
	pub GROW_CURVE_ATTACK_302: f32,
	pub GROW_CURVE_ATTACK_303: f32,
	pub GROW_CURVE_ATTACK_304: f32,
	pub GROW_CURVE_ATTACK_305: f32,
	pub GROW_CURVE_CRITICAL_301: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnemyCurves {
	pub GROW_CURVE_HP: f32,
	pub GROW_CURVE_ATTACK: f32,
	pub GROW_CURVE_DEFENSE: f32,
	pub GROW_CURVE_KILL_EXP: f32,
	pub GROW_CURVE_HP_LITTLEMONSTER: f32,
	pub GROW_CURVE_MHP: f32,
	pub GROW_CURVE_MATK: f32,
	pub GROW_CURVE_HP_2: f32,
	pub GROW_CURVE_ATTACK_2: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurveDB {
	// todo: do I really have to use hashmap? the db json uses (int-as-string, value) pairs to represent an array
	pub characters:  HashMap<i32, CharacterCurves>,//[CharacterCurves; 100],
	pub weapons:  HashMap<i32, WeaponCurves>,//[WeaponCurves; 100],
	pub enemies:  HashMap<i32, EnemyCurves>,//[EnemyCurves; 200],
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnemyStatsEntry {
	pub resistance: EnemyResistances,
	pub base: EnemyBaseStats,
	pub curve: EnemyStatCurves,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnemyBaseStats {
	pub hp: f32,
	pub attack: f32,
	pub defense: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnemyStatCurves {
	pub hp: EnemyCurveType,
	pub attack: EnemyCurveType,
	pub defense: EnemyCurveType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnemyResistances {
	pub physical: f32,
    pub pyro: f32,
    pub dendro: f32,
    pub hydro: f32,
    pub geo: f32,
    pub anemo: f32,
    pub cryo: f32,
    pub electro: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StatsDB {
	pub characters: HashMap<String, CharacterStatsEntry>,
	pub weapons: serde_json::Value, 
	pub talents: serde_json::Value, // HashMap<String, TalentStatsEntry> // FXP, x = decimals, P = Percent
	pub enemies: HashMap<String, EnemyStatsEntry>, 
}

pub enum SpecializedKey {
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

pub struct PromotionEntry {
	pub maxlevel: i32,
	pub hp: f32,
	pub attack: f32,
	pub defense: f32,
	pub specialized: f32,
}

// TODO
#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterStatsEntry {
	pub base: serde_json::Value,
	pub curve: serde_json::Value,
	pub specialized: serde_json::Value,
	pub promotion: serde_json::Value//PromotionEntry,
}

// todo: all fields are Value until I can decide what they really are
#[derive(Debug, Deserialize, Serialize)]
pub struct GenshinDatabase {
	pub data: serde_json::Value,
	pub image: serde_json::Value,
	pub curve: CurveDB,// todo
	pub version: serde_json::Value,
	pub index: serde_json::Value,
	pub stats: StatsDB,
	pub url: serde_json::Value,
}


// todo
pub struct CharacterBase {
/*
	baseStats: BaseStats,
	curveTypes: CurveTypes,
	specialType: StatKey,
	promotionStats: PromotionStats,
*/
}

pub struct BaseStats {
	pub hp: f32,
	pub atk: f32,
	pub def: f32,
	pub critRate_: f32,
	pub critDMG_: f32,
}

pub struct CurveTypes {
	pub hp: CharacterCurveType,
	pub atk: CharacterCurveType,
	pub def: CharacterCurveType,
}
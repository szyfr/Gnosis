

//= Allows
#![allow(dead_code)]


//= Imports


//= Constants

const SKILL_HEALTHMAX: i32 = 20;
const SKILL_HEALTHREGENMAX: i32 = 20;

const SKILL_STAMINAMAX: i32 = 20;
const SKILL_STAMINAREGENMAX: i32 = 20;

const SKILL_MANAMAX: i32 = 20;
const SKILL_MANAREGENMAX: i32 = 20;

const SKILL_MOVEMENTSPEEDMAX: i32 = 20;


//= Enumerations

/// Definition of all skills
#[derive(PartialEq, Eq, Hash)]
pub enum Skill{
	Health,
	HealthRegen,

	Stamina,
	StaminaRegen,

	Mana,
	ManaRegen,

	MovementSpeed,
}


//= Structures


//= Procedures
import { TownType } from "../fightGenerator/types"

export type ReservedHero = {
    id: number,
    name: string
}

export type ReservedHeroFull = {
    id: number,
    town: TownType,
    skills: { skills: ConcreteSkill[] } | undefined,
    spells: { spells: string[] } | undefined,
    favorite_enemies: { enemies: string[] } |undefined
}

export enum Mastery {
    None = "MASTERY_NONE",
    Basic = "MASTERY_BASIC",
    Advanced = "MASTERY_ADVANCED",
    Expert = "MASTERY_EXPERT",
    ExtraExpert = "MASTERY_EXTRA_EXPERT"
}

export type SkillData = {
    id: number,
    game_id: string,
    names: { names: string[] }
}

export type PerkData = {
    id: number,
    game_id: string,
    names: { names: string[] },
    basic_skill: string
}

export type ConcreteSkill = {
    slot: number,
    skill: string,
    mastery: Mastery,
    perks: string[]
}

export enum MagicSchool {
    Destructive = "MAGIC_SCHOOL_DESTRUCTIVE",
    Light = "MAGIC_SCHOOL_LIGHT",
    Dark = "MAGIC_SCHOOL_DARK",
    Summoning = "MAGIC_SCHOOL_SUMMONING",
    Runic = "MAGIC_SCHOOL_RUNIC",
    Warcries = "MAGIC_SCHOOL_WARCRIES"
}

export type SpellData = {
    id: number,
    game_id: string,
    name: string
}
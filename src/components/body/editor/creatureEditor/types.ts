import { TownType } from "../fightGenerator/types"
import { Mastery } from "../reserveHeroesGenerator/types"

type CreatureSpellModel = {
    spell: string,
    mastery: Mastery
}

export type KnownSpellsModel = {
    spells: CreatureSpellModel[]
}

export type ResourcesModel = {
    wood: number,
    ore: number,
    mercury: number,
    sulfur: number,
    crystal: number,
    gem: number,
    gold: number
}

type Upgrades = {
    upgrades: string[]
}

type Abilities = {
    abilities: string[]
}

export enum MagicElement {
    None = "ELEMENT_NONE",
    Fire = "ELEMENT_FIRE",
    Air = "ELEMENT_AIR",
    Water = "ELEMENT_WATER",
    Earth = "ELEMENT_EARTH"
}

type MagicElementModel = {
    first: MagicElement,
    second: MagicElement
}

export type CreatureEditableModel = {
    id: number,
    attack: number,
    defence: number,
    min_damage: number,
    max_damage: number,
    speed: number,
    initiative: number,
    health: number,
    is_flying: boolean,
    known_spells: KnownSpellsModel,
    spell_points: number,
    exp: number,
    power: number,
    tier: number,
    grow: number,
    town: TownType,
    cost: ResourcesModel,
    is_generatable: boolean,
    shared: string,
    size: number,
    range: number,
    is_upgrade: boolean,
    shots: number,
    name: string,
    upgrades: Upgrades,
    base_creature: string,
    pair_creature: string,
    abilities: Abilities,
    magic_element: MagicElementModel,
    desc: string
}
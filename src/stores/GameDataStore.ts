import { create } from "zustand"
import { ArtifactClassType, ArtifactSlotType } from "../components/body/editor/fightGenerator/elements/artifactsGenerator/types"
import { TownType } from "../components/body/editor/fightGenerator/types"

export type ArtifactModel = {
    id: number,
    name: string,
    slot: ArtifactSlotType,
    class: ArtifactClassType,
    desc: string,
    cost: number,
    is_generatable: boolean,
    attack: number,
    defence: number,
    spell_power: number,
    knowledge: number,
    luck: number,
    morale: number,
    name_txt: string,
    desc_txt: string
}

export type CreatureModel = {
    id: number,
    attack: number,
    defence: number,
    speed: number,
    initiative: number,
    exp: number,
    power: number,
    tier: number,
    town: TownType,
    grow: number,
    is_generatable: boolean,
    name: string,
    game_id: string,
    inner_name: string | null
}

export type AbilityModel = {
    id: number,
    name: string,
    game_id: string
}

export type Editable = {
    name: string
}

export type HeroModel = {
    id: string,
    editable: Editable
}

export type SpellModel = {
    game_id: string,
    name: string
}

type Data = {
    artifacts: ArtifactModel[],
    creatures: CreatureModel[],
    abilities: AbilityModel[],
    spells: SpellModel[],
    heroes: HeroModel[]
}

type Action = {
    load_artifacts: (values: ArtifactModel[]) => void,
    load_creatures: (values: CreatureModel[]) => void,
    load_abilities: (values: AbilityModel[]) => void,
    load_spells: (values: SpellModel[]) => void,
    load_heroes: (values: HeroModel[]) => void
}

const useGameDataStore = create<Data & Action>((set) => ({
    artifacts: [],
    creatures: [],
    abilities: [],
    heroes: [],
    spells: [],

    load_artifacts(values) {
        set({artifacts: values});
    },
    load_creatures(values) {
        set({creatures: values});
    },
    load_abilities(values) {
        set({abilities: values});
    },
    load_heroes(values) {
        set({heroes: values});
    },
    load_spells(values) {
        set({spells: values})
    },
}))

export default useGameDataStore;
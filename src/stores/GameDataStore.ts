import { create } from "zustand"
import { ArtifactSlotType } from "../components/body/editor/fightGenerator/elements/artifactsGenerator/types"
import { TownType } from "../components/body/editor/fightGenerator/types"

export type ArtifactModel = {
    id: number,
    name: string,
    slot: ArtifactSlotType
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
    inner_name: string | null
}

type Data = {
    artifacts: ArtifactModel[],
    creatures: CreatureModel[]
}

type Action = {
    load_artifacts: (values: ArtifactModel[]) => void
    load_creatures: (values: CreatureModel[]) => void
}

const useGameDataStore = create<Data & Action>((set) => ({
    artifacts: [],
    creatures: [],

    load_artifacts(values) {
        set({artifacts: values});
    },
    load_creatures(values) {
        set({creatures: values});
    },
}))

export default useGameDataStore;
export type CreatableCreatureModel = {
    id: number,
    base_creature: number | undefined,
    inner_name: string | undefined,
    parent_creature: number | undefined,
    upgrades: number[]
}

export type SessionConfig = {
    name: string,
    current_id: number,
    created_ids: number [],
    models: CreatableCreatureModel [],
    selected_abilities: number []
}
export type CreatableCreatureModel = {
    id: number,
    base_creature: number | undefined,
    inner_name: string | undefined,
    parent_creature: number | undefined,
    upgrades: number[]
}
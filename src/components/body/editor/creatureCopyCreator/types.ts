export type CreatableCreatureModel = {
    id: number,
    baseCreature: number | undefined,
    innerName: string | undefined,
    parentCreature: number | undefined,
    upgrades: number[]
}
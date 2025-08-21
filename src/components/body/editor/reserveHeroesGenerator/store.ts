import { create } from "zustand"
import { ConcreteSkill, ReservedHero, ReservedHeroFull, SkillData } from "./types"
import { TownType } from "../fightGenerator/types";

type Actions = {
    loadHeroes: (values: ReservedHero[]) => void,
    loadBaseSkills: (values: SkillData[]) => void,
    loadReservedHero: (value: ReservedHeroFull) => void,
    unloadReserveHero: () => void,
    updateFreeSlots: (values: number[]) => void,
    updateSkills: (value: { skills: ConcreteSkill[] }) => void;
    addSpell: (value: string) => void,
    removeSpell: (value: string) => void,
    updateEnemies: (values: string[]) => void
}

type Store = {
    heroes: ReservedHero[] | undefined,
    baseSkills: SkillData[] | undefined,
    freeSlots: number[] | undefined,

    currentId: number | undefined,
    currentTown: TownType | undefined,
    currentSkills: { skills: ConcreteSkill[] } | undefined,
    currentSpells: { spells: string[] } | undefined,
    currentEnemies: { enemies: string[] } | undefined,

    actions: Actions 
}

const store = create<Store>((set, get) => ({
    heroes: undefined,
    baseSkills: undefined,
    freeSlots: [...Array(6).keys()],

    currentId: undefined,
    currentTown: undefined,
    currentSpells: undefined,
    currentSkills: undefined,
    currentEnemies: undefined,

    actions: {
        loadHeroes(values) {
            set({heroes: values});
        },
        loadBaseSkills(values) {
            set({baseSkills: values});
        },
        loadReservedHero(value) {
            set({
                currentId: value.id,
                currentTown: value.town,
                currentSkills: value.skills,
                currentSpells: value.spells,
                currentEnemies: value.favorite_enemies,
                freeSlots: get().freeSlots?.filter(s => !value.skills?.skills.some(v => v.slot == s))
            })
        },
        unloadReserveHero() {
            set({
                currentId: undefined,
                currentTown: undefined,
                currentSkills: undefined,
                currentSpells: undefined,
                currentEnemies: undefined,
                freeSlots: [...Array(6).keys()]
            })
        },
        updateSkills(value) {
            set({
                currentSkills: value
            })
        },
        updateFreeSlots(values) {
            set({freeSlots: values});
        },
        addSpell(value) {
            set({currentSpells: {...get().currentSpells, spells: [...get().currentSpells?.spells!, value]}})
        },
        removeSpell(value) {
            set({currentSpells: {...get().currentSpells, spells: [...get().currentSpells?.spells!.filter(s => s != value)!]}})
        },
        updateEnemies(values) {
            set({currentEnemies: {...get().currentEnemies, enemies: values}});
        },
    }
}))

export namespace ReserveHeroesGenerator {
    export const useActions = () => store(state => state.actions);
    export const useHeroes = () => store(state => state.heroes);
    export const useBaseSkills = () => store(state => state.baseSkills);
    
    export const useCurrentId = () => store(state => state.currentId);
    export const useCurrentTown = () => store(state => state.currentTown);
    export const useSkills = () => store(state => state.currentSkills);
    export const useSpells = () => store(state => state.currentSpells);
    export const useFreeSlots = () => store(state => state.freeSlots);
    export const useFavoriteEnemies = () => store(state => state.currentEnemies);
}
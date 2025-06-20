import { create } from "zustand"
import { AssetGenerationType, DifficultyMappedValue, TownType } from "../../types"
import { ArmyGenerationStatElement, CreatureIds, CreatureTiers, CreatureTowns, FightAssetStackModel, StackCountGenerationType, StackGenerationParam, StackGenerationRules, StackUnitGenerationType, StatGenerationRule, StatGenerationType } from "./types"

type Actions = {
    loadAsset: (value: FightAssetStackModel) => void,
    loadStatsParams: (value: ArmyGenerationStatElement[]) => void,
    setBasePowers: (value: DifficultyMappedValue) => void,
    setPowersGrow: (value: DifficultyMappedValue) => void,
    setConcreteCounts: (value: DifficultyMappedValue) => void,
    setTowns: (value: TownType []) => void
    setTiers: (value: number []) => void,
    setConcreteCreatures: (value: number []) => void,
    setRules: (value: StackGenerationParam []) => void,
    addStatParam: (value: ArmyGenerationStatElement) => void,
    removeStatParam: (value: number) => void,
    setCurrentStatParamElement: (value: number) => void,
    updateStatParamElementStats: (elementId: number, value: StatGenerationType []) => void,
    updateStatParamElementRule: (elementId: number, value: StatGenerationRule) => void,
    updateStatParamElementPriority: (elementId: number, value: number) => void
}

type Store = {
    id: number | undefined,
    typeGenerationMode: StackUnitGenerationType | undefined,
    countGenerationMode: StackCountGenerationType | undefined,
    powerBasedGenerationType: AssetGenerationType | undefined,
    basePowers: DifficultyMappedValue | undefined,
    powersGrow: DifficultyMappedValue | undefined,
    towns: CreatureTowns | undefined,
    tiers: CreatureTiers | undefined,
    concreteCreatures: CreatureIds | undefined,
    concreteCount: DifficultyMappedValue | undefined,
    rules: StackGenerationRules | undefined,
    statParams: ArmyGenerationStatElement [] | undefined,
    currentStatParamElement: number | undefined,

    actions: Actions
}

const useCurrentStackAssetStore = create<Store>((set, get) => ({
    id: undefined,
    typeGenerationMode: undefined,
    countGenerationMode: undefined,
    powerBasedGenerationType: undefined,
    basePowers: undefined,
    powersGrow: undefined,
    towns: undefined,
    tiers: undefined,
    generationRule: undefined,
    concreteCreatures: undefined,
    concreteCount: undefined,
    rules: undefined,
    statParams: undefined,
    currentStatParamElement: undefined,

    actions: {
        loadAsset(value) {
            set({
                id: value.id,
                typeGenerationMode: value.type_generation_mode,
                countGenerationMode: value.count_generation_mode,
                powerBasedGenerationType: value.power_based_generation_type,
                basePowers: value.base_powers,
                powersGrow: value.powers_grow,
                towns: value.towns,
                tiers: value.tiers,
                concreteCreatures: value.concrete_creatures,
                concreteCount: value.concrete_count,
                rules: value.generation_rule
            })
        },
        loadStatsParams(value) {
            set({statParams: value});
        },
        setBasePowers(value) {
            set({basePowers: value});
        },
        setPowersGrow(value) {
            set({powersGrow: value});
        },
        setConcreteCounts(value) {
            set({concreteCount: value});
        },
        setTowns(value) {
            set({towns: {...get().towns, towns: value}})
        },
        setTiers(value) {
            set({tiers: {...get().tiers, tiers: value}})
        },
        setConcreteCreatures(value) {
            set({concreteCreatures: {...get().concreteCreatures, ids: value}});
        },
        setRules(value) {
            set({rules: {...get().rules, params: value}})
        },
        addStatParam(value) {
            set({statParams: [...get().statParams!, value]})
        },
        removeStatParam(value) {
            set({statParams: get().statParams?.filter(s => s.id != value)});
        },
        setCurrentStatParamElement(value) {
            set({currentStatParamElement: value});
        },
        updateStatParamElementStats(elementId, value) {
            const updatedElements = get().statParams?.map(s => {
                if (s.id == elementId) {
                    s.stats = {...s.stats, values: value};
                    return s;
                } else {
                    return s;
                }
            });
            set({statParams: updatedElements});
        },
        updateStatParamElementRule(elementId, value) {
            const updatedElements = get().statParams?.map(s => {
                if (s.id == elementId) {
                    s.rule = value;
                    return s;
                } else {
                    return s;
                }
            });
            set({statParams: updatedElements});
        },
        updateStatParamElementPriority(elementId, value) {
            const updatedElements = get().statParams?.map(s => {
                if (s.id == elementId) {
                    s.priority = value;
                    return s;
                } else {
                    return s;
                }
            });
            set({statParams: updatedElements});
        },
    }
}));

export const useCurrentStackActions = () => useCurrentStackAssetStore(state => state.actions);
export const useCurrentStackId = () => useCurrentStackAssetStore(state => state.id);
export const useTypeGenerationMode = () => useCurrentStackAssetStore(state => state.typeGenerationMode);
export const useCountGenerationMode = () => useCurrentStackAssetStore(state => state.countGenerationMode);
export const useBasePowers = () => useCurrentStackAssetStore(state => state.basePowers);
export const usePowersGrow = () => useCurrentStackAssetStore(state => state.powersGrow);
export const usePowerBasetGenerationType = () => useCurrentStackAssetStore(state => state.powerBasedGenerationType);
export const useConcreteCounts = () => useCurrentStackAssetStore(state => state.concreteCount);
export const useCurrentStackTowns = () => useCurrentStackAssetStore(state => state.towns);
export const useCurrentStackTiers = () => useCurrentStackAssetStore(state => state.tiers);
export const useConcreteCreatures = () => useCurrentStackAssetStore(state => state.concreteCreatures);
export const useCurrentStackRules = () => useCurrentStackAssetStore(state => state.rules);
export const useStatParams = () => useCurrentStackAssetStore(state => state.statParams);
export const useCurrentStatParamElement = () => useCurrentStackAssetStore(state => state.currentStatParamElement);
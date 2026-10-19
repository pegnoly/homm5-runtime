import { create } from "zustand"
import {AssetGenerationType, DifficultyMappedValue, TownTypeExtended} from "../../types"
import {
    CalculationType,
    CreatureIds,
    CreatureTiers,
    CreatureTowns,
    FightAssetStackModel,
    SelectionType,
    StackGenerationParam,
    StackGenerationRules,
    StackStatsRules,
    StatRuleModel
} from "./types"
import { UpdateStackDataPayload } from "./data"

type Actions = {
    loadAsset: (value: FightAssetStackModel) => void,
    updateBaseData: (value: UpdateStackDataPayload) => void,
    setBasePowers: (value: DifficultyMappedValue) => void,
    setPowersGrow: (value: DifficultyMappedValue) => void,
    setSpecificCounts: (value: DifficultyMappedValue) => void,
    setTowns: (value: TownTypeExtended []) => void
    setTiers: (value: number []) => void,
    setSpecificCreatures: (value: number []) => void,
    setRules: (value: StackGenerationParam []) => void,
    setStatsRules: (value: StatRuleModel []) => void
}

type Store = {
    id: number | undefined,
    selectionType: SelectionType | undefined,
    calculationType: CalculationType | undefined,
    updateType: AssetGenerationType | undefined,
    basePowers: DifficultyMappedValue | undefined,
    powersGrow: DifficultyMappedValue | undefined,
    towns: CreatureTowns | undefined,
    tiers: CreatureTiers | undefined,
    specificCreatures: CreatureIds | undefined,
    specificCount: DifficultyMappedValue | undefined,
    rules: StackGenerationRules | undefined,
    statRules: StackStatsRules | undefined,

    actions: Actions
}

const store = create<Store>((set, get) => ({
    id: undefined,
    selectionType: undefined,
    calculationType: undefined,
    updateType: undefined,
    basePowers: undefined,
    powersGrow: undefined,
    towns: undefined,
    tiers: undefined,
    specificCreatures: undefined,
    specificCount: undefined,
    rules: undefined,
    statRules: undefined,

    actions: {
        loadAsset(value) {
            set({
                id: value.id,
                selectionType: value.selection_type,
                calculationType: value.calculation_type,
                updateType: value.update_type,
                basePowers: value.base_powers,
                powersGrow: value.powers_grow,
                towns: value.towns,
                tiers: value.tiers,
                specificCreatures: value.specific_creatures,
                specificCount: value.specific_count,
                rules: value.rules,
                statRules: value.stat_rules
            })
        },
        updateBaseData(value) {
            set({selectionType: value.selectionType, calculationType: value.calculationType, updateType: value.updateType});
        },
        setBasePowers(value) {
            set({basePowers: value});
        },
        setPowersGrow(value) {
            set({powersGrow: value});
        },
        setSpecificCounts(value) {
            set({specificCount: value});
        },
        setTowns(value) {
            set({towns: {...get().towns, towns: value}})
        },
        setTiers(value) {
            set({tiers: {...get().tiers, tiers: value}})
        },
        setSpecificCreatures(value) {
            set({specificCreatures: {...get().specificCreatures, ids: value}});
        },
        setRules(value) {
            set({rules: {...get().rules, rules: value}})
        },
        setStatsRules(value) {
            set({statRules: {...get().statRules, rules: value}})
        }
    }
}));

export namespace FightStackGeneratorStore {
    export const useActions = () => store(state => state.actions);
    export const useCurrentId = () => store(state => state.id);
    export const useSelectionType = () => store(state => state.selectionType);
    export const useCalculationType = () => store(state => state.calculationType);
    export const useUpdateType = () => store(state => state.updateType);
    export const useTowns = () => store(state => state.towns);
    export const useTiers = () => store(state => state.tiers);
    export const useSpecificCreatures = () => store(state => state.specificCreatures);
    export const useSpecificCounts = () => store(state => state.specificCount);
    export const useBasePowers = () => store(state => state.basePowers);
    export const usePowersGrow = () => store(state => state.powersGrow);
    export const useGenerationRules = () => store(state => state.rules);
    export const useStatsRules = () => store(state => state.statRules);
}
import { create } from "zustand"
import { AssetGenerationType, DifficultyMappedValue } from "../../types"
import { ArtifactSlotType, FightAssetArtifactsModel, OptionalArtifacts, RequiredArtifacts } from "./types"

type Actions = {
    loadAsset: (value: FightAssetArtifactsModel | null) => void,
    setBasePowers: (value: DifficultyMappedValue) => void,
    setPowersGrow: (value: DifficultyMappedValue) => void,
    addRequiredArtifact: (value: number) => void,
    removeRequiredArtifact: (value: number) => void,
    addOptionalArtifact: (slot: ArtifactSlotType, value: number) => void,
    removeOptionalArtifact: (slot: ArtifactSlotType, value: number) => void,
    updateSheetId: (value: number) => void
}

type Store = {
    id: number | undefined,
    generationType: AssetGenerationType | undefined,
    basePowers: DifficultyMappedValue | undefined,
    powersGrow: DifficultyMappedValue | undefined,
    required: RequiredArtifacts | undefined,
    optional: OptionalArtifacts | undefined, 
    sheetId: number | null | undefined,

    actions: Actions
}

const useCurrentArtifactsAssetStore = create<Store>((set, get) => ({
    id: undefined,
    generationType: undefined,
    basePowers: undefined,
    powersGrow: undefined,
    required: undefined,
    optional: undefined,
    sheetId: undefined,

    actions: {
        loadAsset(value) {
            if (!value) {
                return;
            }
            set({
                id: value.id,
                generationType: value.generation_type,
                basePowers: value.base_powers,
                powersGrow: value.powers_grow,
                required: value.required,
                optional: value.optional,
                sheetId: value.sheet_id
            })
        },
        setBasePowers(value) {
            set({basePowers: value});
        },
        setPowersGrow(value) {
            set({powersGrow: value});
        },
        addRequiredArtifact(value) {
            set({required: {...get().required, ids: [...get().required?.ids!, value]}})
        },
        removeRequiredArtifact(value) {
            const currentIds = get().required?.ids;
            const updatedIds = currentIds?.filter(id => id != value);
            set({required: {ids: updatedIds!}});
        },
        addOptionalArtifact(slot, value) {
            const currentValues = get().optional?.values;
            currentValues![slot].push(value); 
            set({optional: {values: currentValues!}})
        },
        removeOptionalArtifact(slot, value) {
            const currentValues = get().optional?.values;
            currentValues![slot] = currentValues![slot].filter(art => art != value);
            set({optional: {values: currentValues!}});
        },
        updateSheetId(value) {
            set({sheetId: value});
        },
    }
}));

export const useCurrentArtifactsActions = () => useCurrentArtifactsAssetStore(state => state.actions);
export const useCurrentArtifactsAssetId = () => useCurrentArtifactsAssetStore(state => state.id);
export const useArtifactsGenerationType = () => useCurrentArtifactsAssetStore(state => state.generationType);
export const useArtifactsBasePowers = () => useCurrentArtifactsAssetStore(state => state.basePowers);
export const useArtifactsPowerGrow = () => useCurrentArtifactsAssetStore(state => state.powersGrow);
export const useRequiredArtifacts = () => useCurrentArtifactsAssetStore(state => state.required);
export const useOptionalArtifacts = () => useCurrentArtifactsAssetStore(state => state.optional);
export const useArtifactsSheetId = () => useCurrentArtifactsAssetStore(state => state.sheetId);
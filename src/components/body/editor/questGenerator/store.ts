import { create } from "zustand"
import {OneOfQuestProgress, Quest, QuestProgress} from "./types"

type Actions = {
    loadQuests: (value: Quest []) => void,

    loadCurrentQuest: (value: Quest) => void,
    loadCurrentProgress: (value: QuestProgress) => void,
    resetCurrentProgress: () => void,

    setCurrentQuestId: (value: number) => void,
    setCurrentQuestName: (value: string) => void,
    setCurrentQuestScriptName: (value: string) => void,
    setCurrentQuestDirectory: (value: string) => void,
    setCurrentQuestDesc: (value: string) => void,
    setCurrentQuestIsActive: (value: boolean) => void,
    setCurrentQuestIsSecondary: (value: boolean) => void,

    setCurrentProgressNumber: (value: number) => void,
    setCurrentProgressId: (value: number) => void,
    setCurrentProgressText: (value: string) => void,
    setCurrentProgressConcatenate: (value: boolean) => void,
    setCurrentProgressOneOf: (value: OneOfQuestProgress) => void
}

type Store = {
    quests: Quest [] | undefined,

    currentQuestId: number | undefined,
    currentQuestName: string | undefined,
    currentQuestDesc: string | undefined,
    currentQuestScriptName: string | undefined,
    currentQuestDirectory: string | undefined,
    currentQuestIsSecondary: boolean | undefined,
    currentQuestIsActive: boolean | undefined,

    currentProgressNumber: number | undefined,
    currentProgressId: number | undefined,
    currentProgressText: string | undefined,
    currentProgressOneOf: OneOfQuestProgress | undefined,
    currentProgressIsConcatenate: boolean | undefined,

    actions: Actions
}

const questGeneratorStore = create<Store>((set) => ({
    quests: undefined,
    
    currentQuestId: undefined,
    currentQuestName: undefined,
    currentQuestDesc: undefined,
    currentQuestScriptName: undefined,
    currentQuestDirectory: undefined,
    currentQuestIsActive: undefined,
    currentQuestIsSecondary: undefined,

    currentProgressNumber: undefined,
    currentProgressId: undefined,
    currentProgressIsConcatenate: undefined,
    currentProgressText: undefined,
    currentProgressOneOf: undefined,

    actions: {
        loadQuests(value) {
            set({quests: value});
        },
        loadCurrentQuest(value) {
            console.log("Loading ", value)
            set({
                currentQuestId: value.id,
                currentQuestName: value.name,
                currentQuestDesc: value.desc,
                currentQuestScriptName: value.script_name,
                currentQuestIsActive: value.is_active,
                currentQuestIsSecondary: value.is_secondary,
                currentQuestDirectory: value.directory
            })
        },
        resetCurrentProgress() {
            set({
                currentProgressId: undefined,
                currentProgressText: undefined,
                currentProgressOneOf: undefined,
                currentProgressIsConcatenate: undefined
            })
        },
        loadCurrentProgress(value) {
            set({
                currentProgressId: value.id,
                currentProgressText: value.text == null ? undefined : value.text,
                currentProgressOneOf: value.one_of_progress == null ? { text: "", start_value: 0, count: 0 } : value.one_of_progress,
                currentProgressNumber: value.number,
                currentProgressIsConcatenate: value.concatenate
            })
        },
        setCurrentQuestId(value) {
            set({currentQuestId: value});
        },
        setCurrentQuestName(value) {
            set({currentQuestName: value});
        },
        setCurrentQuestScriptName(value) {
            set({currentQuestScriptName: value});
        },
        setCurrentQuestDesc(value) {
            set({currentQuestDesc: value});
        },
        setCurrentQuestDirectory(value) {
            set({currentQuestDirectory: value});
        },
        setCurrentQuestIsActive(value) {
            set({currentQuestIsActive: value});
        },
        setCurrentQuestIsSecondary(value) {
            set({currentQuestIsSecondary: value});
        },
        setCurrentProgressNumber(value) {
            set({currentProgressNumber: value});
        },
        setCurrentProgressId(value) {
            set({currentProgressId: value});
        },
        setCurrentProgressText(value) {
            set({currentProgressText: value});
        },
        setCurrentProgressConcatenate(value) {
            set({currentProgressIsConcatenate: value});
        },
        setCurrentProgressOneOf(value) {
            set({currentProgressOneOf: value});
        }
    }
}));

export const useQuests = () => questGeneratorStore(state => state.quests);
export const useQuestsActions = () => questGeneratorStore(state => state.actions);
export const useCurrentQuestId = () => questGeneratorStore(state => state.currentQuestId);
export const useCurrentQuestName = () => questGeneratorStore(state => state.currentQuestName);
export const useCurrentQuestScriptName = () => questGeneratorStore(state => state.currentQuestScriptName);
export const useCurrentQuestDesc = () => questGeneratorStore(state => state.currentQuestDesc);
export const useCurrentQuestDirectory = () => questGeneratorStore(state => state.currentQuestDirectory);
export const useCurrentQuestIsActive = () => questGeneratorStore(state => state.currentQuestIsActive);
export const useCurrentQuestIsSecondary = () => questGeneratorStore(state => state.currentQuestIsSecondary);
export const useCurrentProgressNumber = () => questGeneratorStore(state => state.currentProgressNumber);
export const useCurrentProgressId = () => questGeneratorStore(state => state.currentProgressId);
export const useCurrentProgressText = () => questGeneratorStore(state => state.currentProgressText);
export const useCurrentProgressIsConcatenate = () => questGeneratorStore(state => state.currentProgressIsConcatenate);
export const useCurrentProgressOneOf = () => questGeneratorStore(state => state.currentProgressOneOf);
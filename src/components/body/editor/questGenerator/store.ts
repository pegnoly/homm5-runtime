import { create } from "zustand"
import {OneOfQuestProgress, Quest, QuestProgress, QuestUpdateSource, QuestUpdateSources} from "./types"

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
    setCurrentQuestUpdateSources: (value: QuestUpdateSource[]) => void,
    addCurrentQuestUpdateSource: (value: QuestUpdateSource) => void,
    updateCurrentQuestUpdateSource: (source: string, value: QuestUpdateSource) => void,
    deleteCurrentQuestUpdateSource: (source: string) => void,

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
    currentQuestUpdateSources: QuestUpdateSources | undefined,

    currentProgressNumber: number | undefined,
    currentProgressId: number | undefined,
    currentProgressText: string | undefined,
    currentProgressOneOf: OneOfQuestProgress | undefined,
    currentProgressIsConcatenate: boolean | undefined,

    actions: Actions
}

const store = create<Store>((set) => ({
    quests: undefined,
    
    currentQuestId: undefined,
    currentQuestName: undefined,
    currentQuestDesc: undefined,
    currentQuestScriptName: undefined,
    currentQuestDirectory: undefined,
    currentQuestIsActive: undefined,
    currentQuestIsSecondary: undefined,
    currentQuestUpdateSources: undefined,

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
            set({
                currentQuestId: value.id,
                currentQuestName: value.name,
                currentQuestDesc: value.desc,
                currentQuestScriptName: value.script_name,
                currentQuestIsActive: value.is_active,
                currentQuestIsSecondary: value.is_secondary,
                currentQuestDirectory: value.directory,
                currentQuestUpdateSources: value.update_sources
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
        },
        setCurrentQuestUpdateSources(value) {
            set({currentQuestUpdateSources: { sources: value }})
        },
        addCurrentQuestUpdateSource(value) {
            set(state => ({
                currentQuestUpdateSources: {
                    sources: [...(state.currentQuestUpdateSources?.sources ?? []), value]
                }
            }))
        },
        updateCurrentQuestUpdateSource(source, value) {
            set(state => ({
                currentQuestUpdateSources: {
                    sources: (state.currentQuestUpdateSources?.sources ?? []).map(item =>
                        item.source == source ? value : item
                    )
                }
            }))
        },
        deleteCurrentQuestUpdateSource(source) {
            set(state => ({
                currentQuestUpdateSources: {
                    sources: (state.currentQuestUpdateSources?.sources ?? []).filter(
                        item => item.source != source
                    )
                }
            }))
        }
    }
}));

export namespace QuestGeneratorStore {
    export const useQuests = () => store(state => state.quests);
    export const useActions = () => store(state => state.actions);
    export const useQuestId = () => store(state => state.currentQuestId);
    export const useQuestName = () => store(state => state.currentQuestName);
    export const useQuestScriptName = () => store(state => state.currentQuestScriptName);
    export const useQuestDesc = () => store(state => state.currentQuestDesc);
    export const useQuestDirectory = () => store(state => state.currentQuestDirectory);
    export const useQuestIsActive = () => store(state => state.currentQuestIsActive);
    export const useQuestIsSecondary = () => store(state => state.currentQuestIsSecondary);
    export const useQuestUpdateSources = () => store(state => state.currentQuestUpdateSources);
    export const useProgressNumber = () => store(state => state.currentProgressNumber);
    export const useProgressId = () => store(state => state.currentProgressId);
    export const useProgressText = () => store(state => state.currentProgressText);
    export const useProgressIsConcatenate = () => store(state => state.currentProgressIsConcatenate);
    export const useProgressOneOf = () => store(state => state.currentProgressOneOf);
}
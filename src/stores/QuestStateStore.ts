import { create } from "zustand"

export type QuestLoadingData = {
    id: string,
    name: string
}

export enum QuestEditionState {
    NotSelected,
    BaseDataEdit,
    ProgressEdit,
    TextEdit
}

type State = {
    edition_state: QuestEditionState,
    current_map_quests: QuestLoadingData[] 
}

type Action = {
    set_edition_state: (state: QuestEditionState) => void,
    set_current_map_quests: (quests: QuestLoadingData[]) => void,
    add_quest_loading_data: (qd: QuestLoadingData) => void
}

export const useQuestStateStore = create<State & Action>((set, get) => ({
    edition_state: QuestEditionState.NotSelected,
    current_map_quests: [],

    set_edition_state(state) {
        set({edition_state: state})
    },

    set_current_map_quests(quests) {
        set({current_map_quests: quests})
    },

    add_quest_loading_data(qd) {
        set({current_map_quests: [...get().current_map_quests, qd]})
    },
}))
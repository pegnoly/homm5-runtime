import { create } from "zustand"

export enum QuestEditionState {
    NotSelected,
    BaseDataEdit,
    ProgressEdit,
    TextEdit
}

type State = {
    edition_state: QuestEditionState
}

type Action = {
    set_edition_state: (state: QuestEditionState) => void,
}

export const useQuestStateStore = create<State & Action>((set) => ({
    edition_state: QuestEditionState.NotSelected,
    set_edition_state(state) {
        set({edition_state: state})
    },
}))
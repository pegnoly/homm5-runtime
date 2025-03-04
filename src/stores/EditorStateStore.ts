import { create } from "zustand"

export enum EditorState {
    Quest,
    Dialog,
    ReserveHeroes
}

type State = {
    editor_state: EditorState
}

type Action = {
    set_editor_state: (state: EditorState) => void
}

export const useEditorStateStore = create<State & Action>((set) => ({
    editor_state: EditorState.Quest,
    set_editor_state(state) {
        set({editor_state: state})
    },
}))
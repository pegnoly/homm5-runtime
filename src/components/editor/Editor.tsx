import { Button, Col, Row, Segmented } from "antd";
import { EditorState, useEditorStateStore } from "../../stores/EditorStateStore";
import QuestMain from "../quest/main";
import QuestGenerator from "../quest/generate";
import DialogGeneratorMain from "../dialog/Main";
import DialogGeneratorGlobals from "../dialog/Global";
import ReserveHeroesMain from "../reserve_heroes/main";
import { invoke } from "@tauri-apps/api/core";
import BanksConfiguratorMain from "../bank_configurator/main";

function Editor() {

    async function applyModifications() {
        await invoke("apply_modifications")
    }

    return <>
        <Row>
            <Col span={6}>
                <div style={{display: 'flex', flexDirection: 'column', justifyContent: 'space-between', gap: 10}}>
                    <EditorStateSelector/>
                    <EditorGlobals/>
                    <Button onClick={applyModifications}>Apply modifications to map</Button>
                </div>
            </Col>
            <Col span={18}>
                <EditorWindow/>
            </Col>
        </Row>
    </>
}

function EditorStateSelector() {

    const setEditorState = useEditorStateStore((state) => state.set_editor_state)

    function editorStateChanged(newState: EditorState) {
        setEditorState(newState)
    }

    return <div style={{display: 'flex', justifyContent: 'center'}}>
        <Segmented 
            defaultValue={EditorState.Quest}
            onChange={editorStateChanged}
            vertical
            options={[
                {value: EditorState.Dialog, label: "Edit dialogs"},
                {value: EditorState.Quest, label: "Edit quests"},
                {value: EditorState.ReserveHeroes, label: "Edit reserve heroes"},
                {value: EditorState.Banks, label: "Configure banks"}
            ]}>

        </Segmented>
    </div>
}

function EditorWindow() {

    const editorState = useEditorStateStore((state) => state.editor_state)

    switch (editorState) {
        case EditorState.Quest:
            return <QuestMain/>
        case EditorState.Dialog:
            return <DialogGeneratorMain/>
        case EditorState.ReserveHeroes:
            return <ReserveHeroesMain/>
        case EditorState.Banks:
            return <BanksConfiguratorMain/>
    }
}

function EditorGlobals() {

    const editorState = useEditorStateStore((state) => state.editor_state)

    switch (editorState) {
        case EditorState.Quest:
            return <QuestGenerator/>
        case EditorState.Dialog:
            return <DialogGeneratorGlobals/>
    }
}

export default Editor;
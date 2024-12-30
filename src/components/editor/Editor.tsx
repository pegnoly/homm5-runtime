import { Col, Row, Segmented } from "antd";
import { EditorState, useEditorStateStore } from "../../stores/EditorStateStore";
import QuestMain from "../quest/main";
import QuestGenerator from "../quest/generate";

function Editor() {
    return <>
        <Row>
            <Col span={6}>
                <div style={{display: 'flex', flexDirection: 'column', justifyContent: 'space-between', gap: 10}}>
                    <EditorStateSelector/>
                    <EditorGlobals/>
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
            onChange={editorStateChanged}
            vertical
            options={[
                {value: EditorState.Dialog, label: "Edit dialogs"},
                {value: EditorState.Quest, label: "Edit quests"}
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
            return <></>
    }
}

function EditorGlobals() {

    const editorState = useEditorStateStore((state) => state.editor_state)

    switch (editorState) {
        case EditorState.Quest:
            return <QuestGenerator/>
        case EditorState.Dialog:
            return <></>
    }
}

export default Editor;
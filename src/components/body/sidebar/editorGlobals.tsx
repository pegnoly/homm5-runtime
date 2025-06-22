import { EditorState, useEditorState } from '../../../stores/EditorStateStore';
import DialogGeneratorGlobals from '../editor/dialogGenerator/globals';
import FightGeneratorGlobals from '../editor/fightGenerator/globals';
import styles from '../styles.module.css';

function EditorGlobals() {

    return (
    <div className={styles.editor_globals}>
        <RenderEditorGlobals/>
    </div>
    )
}

function RenderEditorGlobals() {
    const editorState = useEditorState();

    switch (editorState) {
        case EditorState.FightGenerator:
            return <FightGeneratorGlobals/> 
        case EditorState.Dialog:
            return <DialogGeneratorGlobals/>
        default:
            return null;
    }
}

export default EditorGlobals;
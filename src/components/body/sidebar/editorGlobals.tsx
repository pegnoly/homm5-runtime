import { EditorState, useEditorState } from '../../../stores/EditorStateStore';
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
        default:
            return null;
    }
}

export default EditorGlobals;
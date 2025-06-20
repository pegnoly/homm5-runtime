import styles from '../styles.module.css';
import EditorGlobals from './editorGlobals';
import EditorModeSelector from './editorModeSelector';

function Sidebar() {

    return (
    <div className={styles.sidebar}>
        <EditorModeSelector/>
        <EditorGlobals/>
    </div>
    )
}

export default Sidebar;
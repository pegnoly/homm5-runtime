import { Group } from '@mantine/core';
import styles from '../styles.module.css';
import EditorGlobals from './editorGlobals';
import EditorModeSelector from './editorModeSelector';
import EditorTimeline from '@/components/timeline';

function Sidebar() {
    return (
    <div className={styles.sidebar}>
        <EditorModeSelector/>
        <EditorGlobals/>
        <div className={styles.editor_timeline_activator}>
            <Group justify='center' align='end' pos="relative" top="38%">
                <EditorTimeline/>
            </Group>
        </div>
    </div>
    )
}

export default Sidebar;
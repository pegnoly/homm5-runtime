import { Card } from "@mantine/core";
import { EditorState, useEditorState } from "../../../stores/EditorStateStore";
import styles from '../styles.module.css';
import { Link } from "react-router";

function EditorModeLink(params: {
    state: EditorState,
    label: string
}) {
    const editorState = useEditorState();

    return (
    <Card style={{backgroundColor: editorState == params.state ? 'lightgreen' : 'white'}} key={params.state} withBorder radius={0}>
        <Link className={styles.mode_link} to={`/editor/${params.state}`}>{params.label}</Link>
    </Card>   
    )
}

export default EditorModeLink;
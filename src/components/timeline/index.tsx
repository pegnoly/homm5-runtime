import { Button, Dialog, List } from "@mantine/core";
import { EditorTimelineStore } from "./store";

function EditorTimeline() {
    const timelineItems = EditorTimelineStore.useItems();
    const activityStatus = EditorTimelineStore.useActivity();
    const actions = EditorTimelineStore.useActions();

    return (
    <>
        <Button onClick={() => actions.changeActivity(!activityStatus)} variant='gradient' bg="dark">Open timeline</Button>
        <Dialog position={{left: 20, top: 20}} opened={activityStatus} style={{width: 400}}>
            <List>{timelineItems.map(t => (
                <div style={{width: 350}}>
                    <span style={{fontSize: 13, fontWeight: 'bold'}}>{`[${t.timestamp}]: `}</span>
                    <span style={{ fontSize: 12, wordBreak: "break-word", wordWrap: 'break-word'}}>{t.message}</span>
                </div>
            ))}</List>
        </Dialog>
    </>
    )
}

export default EditorTimeline;
import { Button, Dialog, List, Text } from "@mantine/core";
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
                // <List.Item w={350}>
                <div style={{width: 350}}>
                    <Text style={{ width: 350, fontSize: 12, wordBreak: "break-word", wordWrap: 'break-word'}}>{t}</Text>
                </div>
                // </List.Item>
            ))}</List>
        </Dialog>
    </>
    )
}

export default EditorTimeline;
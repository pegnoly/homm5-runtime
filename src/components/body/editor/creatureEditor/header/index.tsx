import { Button } from "@mantine/core";
import CreatureToEditSelector from "./creatureSelector";

function CreatureEditorHeader() {
    return <div style={{display: 'flex', flexDirection: 'row', justifyContent: 'space-around', alignItems: 'center'}}>
        <CreatureToEditSelector/>
        <Button
            radius={0}
        >Generate game files for creature</Button>
    </div>
}

export default CreatureEditorHeader;
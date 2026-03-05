import { Button, ButtonGroup } from "@mantine/core";
import CreatureToEditSelector from "./creatureSelector";
import { CreatureEditorStore } from "../store";
import { invoke } from "@tauri-apps/api/core";
import CreatureTextsEditor from "../body/texts";
import { EditorTimelineStore } from "@/components/timeline/store";

function CreatureEditorHeader() {
    const currentCreature = CreatureEditorStore.useCurrent();
    const actions = EditorTimelineStore.useActions();

    async function generateCreature() {
        await invoke<string>("generate_creature_file", {id: currentCreature?.id})
            .then((value) => actions.addItem(value));
    }

    return <div style={{display: 'flex', flexDirection: 'row', justifyContent: 'space-around', alignItems: 'center'}}>
        <CreatureToEditSelector/>
        <ButtonGroup>
            <CreatureTextsEditor/>
            <Button
                radius={0}
                disabled={currentCreature == undefined}
                onClick={() => generateCreature()}
            >Generate game files for creature</Button>
        </ButtonGroup>
    </div>
}

export default CreatureEditorHeader;
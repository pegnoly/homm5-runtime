import { Button, ButtonGroup } from "@mantine/core";
import CreatureToEditSelector from "./creatureSelector";
import { CreatureEditorStore } from "../store";
import { invoke } from "@tauri-apps/api/core";
import CreatureTextsEditor from "../body/texts";

function CreatureEditorHeader() {
    const currentCreature = CreatureEditorStore.useCurrent();

    async function generateCreature() {
        await invoke("generate_creature_file", {id: currentCreature?.id});
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
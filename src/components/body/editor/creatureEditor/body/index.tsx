import CreatureNumberPropsEditor from "./numberProps";
import CreaturesInteractionEditor from "./creaturesInteraction";
import CreatureCostEditor from "./cost";
import CreatureSpellsEditor from "./spells";
import {useParams} from "react-router";
import {CreatureEditorStore} from "@/components/body/editor/creatureEditor/store.ts";
import {invoke} from "@tauri-apps/api/core";
import {CreatureEditableModel} from "@/components/body/editor/creatureEditor/types.ts";
import CreatureTextsEditor from "@/components/body/editor/creatureEditor/body/texts.tsx";
import CreatureDataGenerator from "@/components/body/editor/creatureEditor/body/dataGenerator.tsx";

function CreatureEditorBody() {
    const actions = CreatureEditorStore.useActions();
    const { id } = useParams();

    if (id == undefined) {
        return null;
    }

    invoke<CreatureEditableModel>("load_creature", {id: parseInt(id!)})
        .then((value) => actions.loadCreature(value));

    return (
        <div style={{width: '100%', height: '100%', overflow: 'hidden'}}>
            <div style={{width: '100%', height: '90%', display: 'flex', flexDirection: 'row', gap: '5%', overflow: 'hidden'}}>
                <CreatureNumberPropsEditor/>
                <CreaturesInteractionEditor/>
                <div style={{width: '40%', display: 'flex', flexDirection: 'column', paddingTop: '2%'}}>
                    <CreatureCostEditor/>
                    <CreatureSpellsEditor/>
                </div>
            </div>
            <div style={{display: 'flex', flexDirection: 'row'}}>
                <CreatureTextsEditor/>
                <CreatureDataGenerator/>
            </div>
        </div>
    )
}

export default CreatureEditorBody;
import CreatureNumberPropsEditor from "./numberProps";
import CreaturesInteractionEditor from "./creaturesInteraction";
import CreatureSpellsEditor from "./spells";
import {useParams} from "react-router";
import {CreatureEditorStore} from "@/components/body/editor/creatureEditor/store.ts";
import {invoke} from "@tauri-apps/api/core";
import {CreatureEditableModel} from "@/components/body/editor/creatureEditor/types.ts";
import {Group, Tabs} from "@mantine/core";
import CreatureDataGenerator from "@/components/body/editor/creatureEditor/body/dataGenerator.tsx";
import CreatureTextsEditor from "@/components/body/editor/creatureEditor/body/texts.tsx";

enum CreatureEditorState {
    Main = "main",
    Abilities = "abils",
    Texts = "texts"
}

function CreatureEditorBody() {
    const actions = CreatureEditorStore.useActions();
    const { id } = useParams();

    if (id == undefined) {
        return null;
    }

    invoke<CreatureEditableModel>("load_creature", {id: parseInt(id!)})
        .then((value) => actions.loadCreature(value));

    return (
        <div style={{width: '100%', height: '100%', overflow: "hidden"}}>
            <Tabs defaultValue={CreatureEditorState.Main}>
                <Tabs.List>
                    <Tabs.Tab value={CreatureEditorState.Main}>
                        Основные параметры
                    </Tabs.Tab>
                    <Tabs.Tab value={CreatureEditorState.Abilities}>
                        Способности и заклинания
                    </Tabs.Tab>
                    <Tabs.Tab value={CreatureEditorState.Texts}>
                        Тексты
                    </Tabs.Tab>
                </Tabs.List>

                <Tabs.Panel value={CreatureEditorState.Main}>
                    <Group>
                        <CreatureNumberPropsEditor/>
                        <CreaturesInteractionEditor/>
                    </Group>
                </Tabs.Panel>
                <Tabs.Panel value={CreatureEditorState.Abilities}>
                    <CreatureSpellsEditor/>
                </Tabs.Panel>
                <Tabs.Panel value={CreatureEditorState.Texts}>
                    <CreatureTextsEditor/>
                </Tabs.Panel>
            </Tabs>
            <CreatureDataGenerator/>
        </div>
    )
}

export default CreatureEditorBody;
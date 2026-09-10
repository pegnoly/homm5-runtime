import {CreatureModel} from "@/stores/GameDataStore.ts";
import {List, Text} from "@mantine/core";
import {Link} from "react-router";
import {EditorState} from "@/stores/EditorStateStore.ts";
import {CreatureEditorStore} from "@/components/body/editor/creatureEditor/store.ts";

function CreaturesEditorList({models}: {models: CreatureModel[]}) {

    const currentCreature = CreatureEditorStore.useCurrent();
    console.log(models);
    return (
        <>
            <List>{models.map((model, index) => (
                <Link key={index} style={{textDecoration: 'none'}} to={`/editor/${EditorState.CreatureEditor}/focused/${model.id}`}>
                    <div>
                        <Text bg={currentCreature?.id == model.id ? "green" : "silver"}
                              size='sm'
                              lineClamp={1}>{model.inner_name != null && model.inner_name != "" ?
                            `${model.inner_name}[${model.id}]` : `${model.name}[${model.id}]`}</Text>
                    </div>
                </Link>
            ))}</List>
        </>
    )
}

export default CreaturesEditorList;
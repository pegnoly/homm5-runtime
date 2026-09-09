import CreatureToEditSelector, {CreatureSelectionRequest} from "./creatureSelector";
import useGameDataStore, {CreatureModel} from "@/stores/GameDataStore.ts";
import {useState} from "react";
import CreaturesEditorList from "@/components/body/editor/creatureEditor/sidebar/list.tsx";

function CreatureEditorSidebar() {
    const creaturesData = useGameDataStore(state => state.creatures);
    const [selectedCreatures, setSelectedCreatures] = useState<CreatureModel[]>(creaturesData);
    function selectionUpdated(request: CreatureSelectionRequest) {
        console.log("Selection request: ", request)
        if (request.id != undefined && !Number.isNaN(request.id)) {
            const specificSelection = creaturesData.find(c => c.id == request.id);
            if (specificSelection != undefined) {
                console.log("Specific selection: ", specificSelection)
                setSelectedCreatures([specificSelection])
            }
            return
        } else {
            let updatedCreatures = request.townsFilter?.length != 0 ?
                creaturesData.filter(c => request.townsFilter?.includes(c.town_extended)) :
                creaturesData;
            updatedCreatures = request.nameFilter != "" ?
                updatedCreatures.filter(c => {
                    const check = request.nameFilter?.toLowerCase();
                    if (c.inner_name != null && c.inner_name.length > 0 && c.inner_name.toLowerCase().includes(check!)) {
                        return true;
                    }
                    return c.name.toLowerCase().includes(check!);

                }) : updatedCreatures;

            setSelectedCreatures(updatedCreatures);
        }
    }

    return (
        <div style={{display: "flex", flexDirection: "column", width: '100%', height: '100%'}}>
            <div style={{width: '100%', height: '15%'}}>
                <CreatureToEditSelector onSelectionUpdated={selectionUpdated}/>
            </div>
            <div style={{width: '100%', height: '85%', overflow: 'auto'}}>
                <CreaturesEditorList models={selectedCreatures}/>
            </div>
        </div>
    )
}
export default CreatureEditorSidebar;
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { useCommonActions, useCurrentMapId } from "../../../stores/common";
import { Select, Text } from "@mantine/core";
import styles from '../styles.module.css'

export type MapData = {
    id: number,
    name: string
}

function MapSelector() {
    const [maps, setMaps] = useState<MapData[]>([]);
    const currentMapId = useCurrentMapId();
    const commonActions = useCommonActions();

    useEffect(() => {
        if (maps.length == 0) {
            invoke<MapData[]>("load_maps")
                .then((maps_value) => {
                    setMaps(maps_value);
                    invoke<number | null>("load_current_map")
                        .then((id_value) => {
                            if (id_value != null) {
                                commonActions.setCurrentMapId(id_value);
                            }
                        })
                })
        }
    }, [maps])

    async function selectMap(map_id: number) {
        commonActions.setCurrentMapId(map_id)
        await invoke("select_map", {id: map_id});
    }

    return (
    <div className={styles.map_selector}>
        <Text style={{fontFamily: 'cursive', fontWeight: 'bold', fontSize: 17}}>Current map:  </Text>
        <Select 
            value={currentMapId?.toString()}
            onChange={(value) => selectMap(parseInt(value!))}
            data={maps.map(m => ({label: m.name, value: m.id.toString()}))}
        />
    </div>
    )
}

export default MapSelector;
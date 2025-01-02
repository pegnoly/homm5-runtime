import { useEffect, useState } from "react";
import { MapData } from "../../types";
import { invoke } from "@tauri-apps/api/core";
import { Select, Typography } from "antd";

function MapSelector() {

    const [maps, setMaps] = useState<MapData[]>([])
    const [currentMapId, setCurrentMapId] = useState<number | null>(null);

    useEffect(() => {
        if (maps.length == 0) {
            invoke<MapData[]>("load_maps")
                .then((maps_value) => {
                    setMaps(maps_value);
                    invoke<number | null>("load_current_map")
                        .then((id_value) => {
                            if (id_value != null) {
                                setCurrentMapId(id_value);
                            }
                        })
                })
        }
    }, [maps])

    async function selectMap(map_id: number) {
        setCurrentMapId(map_id)
        await invoke("select_map", {id: map_id});
    }

    return <>
        <Typography.Text style={{fontFamily: 'cursive', fontWeight: 'bold', fontSize: 17}}>Current map:  </Typography.Text>
        <Select 
            value={currentMapId}
            onChange={selectMap}
        >{maps.map((m, i) => (
            <Select.Option key={i} value={m.id}>{m.name}</Select.Option>
        ))}</Select>
    </>
}

export default MapSelector;
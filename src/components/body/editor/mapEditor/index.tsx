import {Button, Select, Text} from "@mantine/core";
import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/core";
import {useState} from "react";

function MapEditorLayout() {

    const [mapSize, setMapSize] = useState<string | undefined>(undefined);
    const [mapPath, setMapPath] = useState<string | undefined>(undefined);

    listen<string>("map_directory_picked", (e) => {
        setMapPath(e.payload);
    });

    return (
        <>
            <Select
                label="Select map size"
                radius={0}
                w={250}
                value={mapSize}
                onChange={(e) => setMapSize(e!)}
                data={[
                    { label: "72", value: "72" },
                    { label: "96", value: "96" },
                    { label: "136", value: "136" },
                    { label: "176", value: "176" },
                    { label: "216", value: "216" },
                    { label: "256", value: "256" },
                    { label: "312", value: "312" },
                ]}
            />
            <Button onClick={() => invoke("pick_map_xdb_directory")} radius={0}>Pick map directory</Button>
            <Text>{mapPath}</Text>
            <Button
                radius={0}
                disabled={mapSize == undefined || mapPath == undefined}
                onClick={() => invoke("move_map_to_dir", {path: mapPath, size: parseInt(mapSize!)})}
            >Create</Button>
        </>
    )
}

export default MapEditorLayout;
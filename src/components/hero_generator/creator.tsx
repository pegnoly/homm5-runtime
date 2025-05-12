import { useState } from "react";
import { HeroAssetSimple } from "./main";
import { Button, Input, Modal, Space, Typography } from "antd";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

function HeroAssetCreator(params: { assetCreatedCallback: (created: HeroAssetSimple) => void }) {
    const [opened, setOpened] = useState<boolean>(false);
    const [directory, setDirectory] = useState<string | null>(null);
    const [name, setName] = useState<string | null>(null);
    const [tableName, setTableName] = useState<string | null>(null);

    async function close() {
        setOpened(false)
    }

    listen<string>("hero_lua_directory_picked", (event => setDirectory(event.payload)));

    async function create() {
        close();
        await invoke<HeroAssetSimple>("init_new_generatable_hero", {name: name, path: directory, tableName: tableName})
            .then((value) => params.assetCreatedCallback(value));
    }

    return <div style={{position: 'sticky'}}>
        <Button onClick={() => setOpened(true)}>Create asset</Button>
        <Modal
            open={opened}
            onCancel={close}
            onClose={close}
        >
            <Space direction="vertical" style={{display: 'flex'}}>
                <Button onClick={() => invoke("pick_hero_lua_generation_directory")}>Pick directory to generate hero script</Button>
                <Typography.Text>{directory}</Typography.Text>
                <Input type="text" placeholder="Enter hero name" value={name!} onChange={(e) => setName(e.target.value)}/>
                <Input type="text" placeholder="Enter lua table name" value={tableName!} onChange={(e) => setTableName(e.target.value)}/>
                <Button disabled={!directory || !name || !tableName} onClick={create}>Create</Button>
            </Space>
        </Modal>
    </div>
}

export default HeroAssetCreator;
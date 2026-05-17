import { useDisclosure } from "@mantine/hooks";
import { listen } from "@tauri-apps/api/event";
import { useState } from "react";
import { FightAssetSimple } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { Button, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, Stack, Text, TextInput, Tooltip } from "@mantine/core";
import {EditorTimelineStore} from "@/components/timeline/store.ts";
import {GetCurrentTimestamp} from "@/components/timeline/types.ts";

function FightAssetCreator({onCreated}: {
    onCreated: (value: FightAssetSimple) => void
}) {
    const [opened, {open, close}] = useDisclosure(false);
    const [directory, setDirectory] = useState<string | null>(null);
    const [name, setName] = useState<string | null>("");
    const [tableName, setTableName] = useState<string | null>("");

    const actions = EditorTimelineStore.useActions();

    listen<string>("hero_lua_directory_picked", (event => setDirectory(event.payload)));

    async function create() {
        close();
        await invoke<FightAssetSimple>("init_new_asset", {name: name, path: directory, tableName: tableName})
            .then((value) => {
                onCreated(value);
                actions.addItem({
                    message: `${name} added to sheets successfully`,
                    timestamp: GetCurrentTimestamp()
                });
                actions.changeActivity(true);
                setTimeout(() => {
                    actions.changeActivity(false);
                }, 3000);
            })
            .catch((error) => {
                actions.addItem({
                    timestamp: GetCurrentTimestamp(),
                    message: error.toString()
                })
                actions.changeActivity(true);
                setTimeout(() => {
                    actions.changeActivity(false);
                }, 3000);
            })
    }

    return <div style={{position: 'sticky'}}>
        <Button radius={0} onClick={open}>Create asset</Button>
        <ModalRoot centered opened={opened} onClose={close}>
            <ModalOverlay/>
            <ModalContent>
                <ModalHeader>
                    <ModalTitle>Fight asset creation</ModalTitle>
                    <ModalCloseButton/>
                </ModalHeader>
                <ModalBody>
                    <Stack>
                        <Button 
                            onClick={() => invoke("pick_hero_lua_generation_directory")}
                        >Pick directory to generate script</Button>
                        <Tooltip label={directory}>
                            <Text lineClamp={1}>{directory}</Text>
                        </Tooltip>
                        <TextInput 
                            label="Asset name" 
                            placeholder="Enter name" 
                            value={name!} 
                            onChange={(e) => setName(e.target.value)}
                        />
                        <TextInput 
                            label="Name of lua table for generated code" 
                            placeholder="Enter lua table name" 
                            value={tableName!} 
                            onChange={(e) => setTableName(e.target.value)}
                        />
                        <Group justify="end">
                            <Button 
                                disabled={!directory || !name || !tableName} 
                                onClick={create}
                            >Create</Button>
                        </Group>
                    </Stack>
                </ModalBody>
            </ModalContent>
        </ModalRoot>
    </div>
}

export default FightAssetCreator;
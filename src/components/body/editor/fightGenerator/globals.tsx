import { Button, Stack, Text } from "@mantine/core";
import { useFightAssetId, useFightAssetName } from "./store";
import { invoke } from "@tauri-apps/api/core";

function FightGeneratorGlobals() {
    //const _actions = useFightAssetActions();
    const assetId = useFightAssetId();
    const assetName = useFightAssetName();

    async function startGeneration() {
        await invoke("generate_current_hero_script", {assetId: assetId});
    }

    async function sync() {
        await invoke("sync_asset", {assetId: assetId});
    }

    return (
    <Stack>
        {
            assetName == undefined ?
            <Text c="red">Asset not selected</Text> :
            <div style={{display: 'flex', flexDirection: 'column'}}>
                <Text style={{fontWeight: 'bold'}} c="dark">Current asset:</Text>
                <Text size="sm">{assetName}</Text>
            </div>
        }
        <Button onClick={startGeneration} radius={0} size="xs" disabled={assetId == undefined}>Generate script for asset</Button>
        <Button onClick={sync} radius={0} size="xs">Test sync</Button>
    </Stack>
    )
}

export default FightGeneratorGlobals;
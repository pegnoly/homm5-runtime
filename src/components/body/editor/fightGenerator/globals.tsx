import { Button, Stack, Text } from "@mantine/core";
import { FightAssetStore } from "./store";
import { invoke } from "@tauri-apps/api/core";

function FightGeneratorGlobals() {
    const assetId = FightAssetStore.useAssetId();
    const assetName = FightAssetStore.useAssetName();

    async function startGeneration() {
        await invoke("generate_current_hero_script", {assetId: assetId});
    }

    return (
    <Stack>
        {
            assetName == undefined ?
            <Text c="red" style={{fontSize: 14}}>Asset not selected</Text> :
            <div style={{display: 'flex', flexDirection: 'row'}}>
                <Text style={{fontWeight: 'bold', fontSize: 14}} c="dark">Current asset: </Text>
                <Text style={{fontSize: 14}}>{assetName}</Text>
            </div>
        }
        <Button onClick={startGeneration} radius={0} size="xs" disabled={assetId == undefined}>Generate script for asset</Button>
    </Stack>
    )
}

export default FightGeneratorGlobals;
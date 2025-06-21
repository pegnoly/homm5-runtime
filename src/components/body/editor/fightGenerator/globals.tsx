import { Button, Stack, Text } from "@mantine/core";
import { useFightAssetActions, useFightAssetId, useFightAssetName } from "./store";

function FightGeneratorGlobals() {
    const actions = useFightAssetActions();
    const assetId = useFightAssetId();
    const assetName = useFightAssetName();

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
        <Button radius={0} size="xs" disabled={assetId == undefined}>Generate script for asset</Button>
    </Stack>
    )
}

export default FightGeneratorGlobals;
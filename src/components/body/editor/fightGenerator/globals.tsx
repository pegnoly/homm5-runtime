import { Button, LoadingOverlay, Stack, Text } from "@mantine/core";
import { useFightAssetId, useFightAssetName } from "./store";
import { invoke } from "@tauri-apps/api/core";
import { useCurrentStackActions, useCurrentStackId } from "./elements/stacksGenerator/store";
import { FightAssetStackModel } from "./elements/stacksGenerator/types";
import { useState } from "react";

function FightGeneratorGlobals() {
    const actions = useCurrentStackActions();
    const assetId = useFightAssetId();
    const assetName = useFightAssetName();
    const stackId = useCurrentStackId();

    const [onSync, setOnSync] = useState<boolean>(false);

    async function startGeneration() {
        await invoke("generate_current_hero_script", {assetId: assetId});
    }

    async function pullFromSheet() {
        setOnSync(true);
        await invoke<FightAssetStackModel[]>("pull_from_sheet", {assetId: assetId})
            .then((data) => {
                setOnSync(false);
                if (stackId != undefined) {
                    const updatedCurrentStack = data.find(s => s.id == stackId)!;
                    actions.loadAsset(updatedCurrentStack);
                }
            })
    }

    async function pushToSheet() {
        setOnSync(true);
        await invoke("push_to_sheet", {assetId: assetId})
            .then(() => {
                setOnSync(false);
            })
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
        <>
            <LoadingOverlay visible={onSync}/>
            <Button onClick={pullFromSheet} radius={0} size="xs" disabled={assetId == undefined}>Pull asset from sheet</Button>
            <Button onClick={pushToSheet} radius={0} size="xs" disabled={assetId == undefined}>Push asset to sheet</Button>
        </>
    </Stack>
    )
}

export default FightGeneratorGlobals;
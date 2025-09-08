import { SimpleGrid } from "@mantine/core";
import { FightAssetSimple } from "../types";
import { useFightAssetActions } from "../store";
import { UUID } from "crypto";
import FightAssetListItem from "./assetListItem";

function FightAssetsList({assets, onAssetDelete}: {
    assets: FightAssetSimple[],
    onAssetDelete: (value: UUID) => void
}) {
    const actions = useFightAssetActions();
    actions.unloadAsset();

    return (
        <>
            <SimpleGrid cols={{xl: 4, sm: 3}}>{assets.map((asset, index) => (
                <FightAssetListItem key={index} asset={asset} onDelete={onAssetDelete}/>
            ))}
            </SimpleGrid>                        
        </>
    )
}

export default FightAssetsList;
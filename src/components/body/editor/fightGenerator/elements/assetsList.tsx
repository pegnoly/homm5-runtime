import { SimpleGrid } from "@mantine/core";
import { FightAssetSimple } from "../types";
import { FightAssetStore } from "../store";
import FightAssetListItem from "./assetListItem";

function FightAssetsList({assets, onAssetDeleted}: {
    assets: FightAssetSimple[],
    onAssetDeleted: (value: number) => void
}) {
    const actions = FightAssetStore.useActions();
    actions.unloadAsset();

    return (
        <>
            <SimpleGrid cols={{xl: 4, sm: 3}}>{assets.map((asset, index) => (
                <FightAssetListItem key={index} asset={asset} onDelete={onAssetDeleted}/>
            ))}
            </SimpleGrid>                        
        </>
    )
}

export default FightAssetsList;
import { Card, SimpleGrid } from "@mantine/core";
import { FightAssetSimple } from "../types";
import { Link } from "react-router";
import { EditorState } from "../../../../../stores/EditorStateStore";
import { useFightAssetActions } from "../store";

function FightAssetsList(params: {
    assets: FightAssetSimple[]
}) {
    const actions = useFightAssetActions();
    actions.unloadAsset();

    return (
    <SimpleGrid cols={{xl: 4, sm: 3}}>{params.assets.map((asset, index) => (
        <Link to={`/editor/${EditorState.FightGenerator}/focused/${asset.id}`} state={{assetName: asset.name}} key={index}>
            <Card radius={0} withBorder>
                {asset.name}
            </Card>
        </Link>
    ))}
    </SimpleGrid>
    )
}

export default FightAssetsList;
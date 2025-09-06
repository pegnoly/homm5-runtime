import { ActionIcon, Card, LoadingOverlay, Overlay, SimpleGrid } from "@mantine/core";
import { FightAssetSimple } from "../types";
import { Link } from "react-router";
import { EditorState } from "../../../../../stores/EditorStateStore";
import { useFightAssetActions } from "../store";
import { IconX } from "@tabler/icons-react";
import { useMutation } from "@tanstack/react-query";
import { UUID } from "crypto";
import { FightGeneratorApi } from "../api";

function FightAssetsList({assets, onDelete}: {
    assets: FightAssetSimple[],
    onDelete: (value: UUID) => void
}) {
    const actions = useFightAssetActions();
    actions.unloadAsset();

    const mutation = useMutation({
        mutationFn: async(id: UUID) => {
            return FightGeneratorApi.deleteAsset(id);
        },
        onSuccess(_data, variables, _context) {
            onDelete(variables)
        },
    })

    return (
        <>
            <SimpleGrid cols={{xl: 4, sm: 3}}>{assets.map((asset, index) => (
                <Card radius={0} withBorder>
                    <ActionIcon 
                        radius={0} 
                        size="xs" 
                        bg="red" 
                        style={{ display: 'flex', justifySelf: 'end', position: 'absolute', top: '5%', right: '2%'}}
                        onClick={() => mutation.mutate(asset.id)}
                    >
                        <IconX/>
                    </ActionIcon>
                    <Link to={`/editor/${EditorState.FightGenerator}/focused/${asset.id}`} state={{assetName: asset.name}} key={index} style={{textDecoration: 'none'}}>
                        {asset.name}
                    </Link>
                </Card>
            ))}
            </SimpleGrid>                        
        </>
    )
}

export default FightAssetsList;
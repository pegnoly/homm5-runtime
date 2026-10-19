import {Card} from "@mantine/core";
import {FightAssetSimple} from "../types";
import {useContextMenu} from "@/hooks/useContextMenu";
import {IconTrash} from "@tabler/icons-react";
import {EditorState} from "@/stores/EditorStateStore";
import {Link} from "react-router";
import {useMutation} from "@tanstack/react-query";
import {FightGeneratorApi} from "../api";
import {AssetContexMenu, AssetMenuItem} from "./assetMenu";

enum FightAssetContextMenuItem {
    Delete,
}

function GenerateFightAssetContextMenu(asset: FightAssetSimple, onInteract: (id: number, type: FightAssetContextMenuItem) => void): AssetMenuItem[] {
    return [
        {label: 'Delete', icon: <IconTrash/>, onClick: () => onInteract(asset.id, FightAssetContextMenuItem.Delete)}
    ];
}


function FightAssetListItem({ asset, onDelete } : {
    asset: FightAssetSimple, 
    onDelete: (id: number) => void,
}) {
    const contextMenu = useContextMenu();

    const deleteMutation = useMutation({
        mutationFn: async(id: number) => {
            return FightGeneratorApi.deleteAsset(id);
        },
        onSuccess(_data, variables, _context) {
            onDelete(variables)
        },
    });

    const menuInteractionCallback = async(id: number, type: FightAssetContextMenuItem) => {
        switch (type) {
            case FightAssetContextMenuItem.Delete: deleteMutation.mutate(id);
                break;
            default:
                break;
        }
    }

    const menuItems = GenerateFightAssetContextMenu(asset, menuInteractionCallback);

    return (
    <>
        <Card radius={0} withBorder onContextMenu={(e) => contextMenu.handleContextMenu(e)}>
            <Link to={`/editor/${EditorState.FightGenerator}/focused/${asset.id}`} state={{assetName: asset.name}} style={{textDecoration: 'none'}}>
                {asset.name}
            </Link>
        </Card>
        <AssetContexMenu 
            items={menuItems} 
            visible={contextMenu.visible}
            x={contextMenu.x}
            y={contextMenu.y}
            onClose={contextMenu.hideContextMenu}    
        />
    </>
    )
}

export default FightAssetListItem;
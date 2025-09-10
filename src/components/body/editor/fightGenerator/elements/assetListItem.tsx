import { Card } from "@mantine/core";
import { FightAssetSimple } from "../types";
import { useContextMenu } from "@/hooks/useContextMenu";
import { UUID } from "crypto";
import { IconNewSection, IconTrash } from "@tabler/icons-react";
import { EditorState } from "@/stores/EditorStateStore";
import { Link } from "react-router";
import { useMutation } from "@tanstack/react-query";
import { FightGeneratorApi } from "../api";
import { AssetContexMenu, AssetMenuItem } from "./assetMenu";
import { invoke } from "@tauri-apps/api/core";

enum FightAssetContextMenuItem {
    Delete,
    CreateSheet
}

function GenerateFightAssetContextMenu(asset: FightAssetSimple, onInteract: (id: UUID, type: FightAssetContextMenuItem) => void): AssetMenuItem[] {
    var items: AssetMenuItem[] = [{label: 'Delete', icon: <IconTrash/>, onClick: () => onInteract(asset.id, FightAssetContextMenuItem.Delete)}];
    if (asset.sheet_id == null) {
        items.push({label: 'Create sheet', icon: <IconNewSection/>, onClick: () => onInteract(asset.id, FightAssetContextMenuItem.CreateSheet)})
    }
    return items;
}


function FightAssetListItem({asset, onDelete, onSheetCreated} : { 
    asset: FightAssetSimple, 
    onDelete: (id: UUID) => void,
    onSheetCreated: (id: UUID, sheetId: number) => void 
}) {
    const contextMenu = useContextMenu();

    const deleteMutation = useMutation({
        mutationFn: async(id: UUID) => {
            return FightGeneratorApi.deleteAsset(id);
        },
        onSuccess(_data, variables, _context) {
            onDelete(variables)
        },
    });

    const createSheetMutation = useMutation({
        mutationFn: async(id: UUID) => {
            return invoke<number>("create_sheet_for_existing_asset", {assetId: id});
        },
        onSuccess(data, variables, _context) {
            console.log("On success: ", data);
            onSheetCreated(variables, data);
        },
    })

    const menuInteractionCallback = (id: UUID, type: FightAssetContextMenuItem) => {
        switch (type) {
            case FightAssetContextMenuItem.Delete: deleteMutation.mutate(id);
                break;
            case FightAssetContextMenuItem.CreateSheet: createSheetMutation.mutate(id);
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
                <>
                {
                    asset.sheet_id == null ? <span style={{color: 'red', fontSize: 10, paddingLeft: '5%'}}>[No sheet]</span> : null
                }
                </>
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
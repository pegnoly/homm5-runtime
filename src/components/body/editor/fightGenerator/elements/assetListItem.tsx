import { Card, LoadingOverlay } from "@mantine/core";
import { FightAssetSimple } from "../types";
import { useContextMenu } from "@/hooks/useContextMenu";
import { UUID } from "crypto";
import { IconGitPullRequest, IconNewSection, IconStackPush, IconTrash } from "@tabler/icons-react";
import { EditorState } from "@/stores/EditorStateStore";
import { Link } from "react-router";
import { useMutation } from "@tanstack/react-query";
import { FightGeneratorApi } from "../api";
import { AssetContexMenu, AssetMenuItem } from "./assetMenu";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

enum FightAssetContextMenuItem {
    Delete,
    CreateSheet,
    PullFromSheet,
    PushToSheet
}

function GenerateFightAssetContextMenu(asset: FightAssetSimple, onInteract: (id: UUID, type: FightAssetContextMenuItem) => void): AssetMenuItem[] {
    var items: AssetMenuItem[] = [
        {label: 'Delete', icon: <IconTrash/>, onClick: () => onInteract(asset.id, FightAssetContextMenuItem.Delete)}
    ];
    if (asset.sheet_id == null) {
        items.push({label: 'Create sheet', icon: <IconNewSection/>, onClick: () => onInteract(asset.id, FightAssetContextMenuItem.CreateSheet)})
    } else {
        items.push({label: 'Pull from sheet', icon: <IconGitPullRequest/>, onClick: () => onInteract(asset.id, FightAssetContextMenuItem.PullFromSheet)})
        items.push({label: 'Push to sheet', icon: <IconStackPush/>, onClick: () => onInteract(asset.id, FightAssetContextMenuItem.PullFromSheet)})
    }
    return items;
}


function FightAssetListItem({asset, onDelete, onSheetCreated} : { 
    asset: FightAssetSimple, 
    onDelete: (id: UUID) => void,
    onSheetCreated: (id: UUID, sheetId: number) => void 
}) {
    const contextMenu = useContextMenu();
    const [isUpdating, setIsUpdating] = useState<boolean>(false);

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
            setIsUpdating(false);
            onSheetCreated(variables, data);
        },
    })

    const menuInteractionCallback = async(id: UUID, type: FightAssetContextMenuItem) => {
        switch (type) {
            case FightAssetContextMenuItem.Delete: deleteMutation.mutate(id);
                break;
            case FightAssetContextMenuItem.CreateSheet: {
                setIsUpdating(true);
                createSheetMutation.mutate(id);
            }
                break;
            case FightAssetContextMenuItem.PullFromSheet: {
                setIsUpdating(true);
                await invoke("pull_from_sheet", {assetId: id}).then(() => setIsUpdating(false));
                break;
            }
            case FightAssetContextMenuItem.PushToSheet: {
                setIsUpdating(true);
                await invoke("push_to_sheet", {assetId: id}).then(() => setIsUpdating(false));
                break;
            }
            default:
                break;
        }
    }

    const menuItems = GenerateFightAssetContextMenu(asset, menuInteractionCallback);

    return (
    <>
        <Card radius={0} withBorder onContextMenu={(e) => contextMenu.handleContextMenu(e)}>
            <LoadingOverlay visible={isUpdating} loaderProps={{color: 'grape', type: "bars"}}/>
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
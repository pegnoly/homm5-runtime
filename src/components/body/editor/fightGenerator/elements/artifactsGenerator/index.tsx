import { useMutation, useQuery } from "@tanstack/react-query"
import { FightGeneratorApi } from "../../api"
import { useArtifactsSheetId, useCurrentArtifactsActions, useCurrentArtifactsAssetId } from "./store"
import ArtifactsAssetCreator from "./creator"
import { FightAssetArtifactsModel } from "./types"
import styles from "../styles.module.css";
import ArtifactsAssetCostsData from "./costs"
import RequiredArtifactsList from "./required"
import OptionalArtifactsList from "./optional"
import { UUID } from "crypto"
import { Button } from "@mantine/core"
import { invoke } from "@tauri-apps/api/core"
import { useEffect } from "react"

function useArtifactsAsset(assetId: UUID) {
    return useQuery({
        queryKey: ["artifacts_asset", assetId],
        queryFn: async() => {
            return FightGeneratorApi.tryLoadArtifactAsset(assetId)
        }
    })
}

function FightAssetArtifactsGenerator({assetId}: {assetId: UUID}) {
    const actions = useCurrentArtifactsActions();
    const artifactsAssetId = useCurrentArtifactsAssetId();
    const sheetId = useArtifactsSheetId();

    async function assetCreated(value: FightAssetArtifactsModel) {
        actions.loadAsset(value);
    }
    
    const associateSheetMutation = useMutation({
        mutationFn: async() => {
            return invoke<FightAssetArtifactsModel>("add_artifacts_data_to_asset_sheet", {assetId: assetId, artAssetId: artifactsAssetId});
        },
        onSuccess(data, _variables, _context) {
            actions.loadAsset(data);
        },
    })

    return (
        <>
            <div className={styles.artifacts_panel}>
                {
                    artifactsAssetId == undefined ?
                    <ArtifactsAssetCreator assetId={assetId} onCreated={assetCreated}/> :
                    <div style={{width: '100%', display: 'flex', flexDirection: 'row'}}>
                        <div style={{width: '40%', paddingLeft: '10%', display: 'flex', flexDirection: 'column', gap: '15%'}}>
                            {
                                sheetId != null ? null : <Button radius={0} bg="red" onClick={() => associateSheetMutation.mutate()}>No art sheet, click to add</Button>
                            }
                            <ArtifactsAssetCostsData/>
                        </div>
                        <div style={{width: "25%"}}>
                            <div style={{width: '100%'}}>
                                <RequiredArtifactsList/>
                            </div>
                        </div>
                        <div style={{width: "35%"}}>
                            <div style={{width: '100%'}}>
                                <OptionalArtifactsList/>
                            </div>
                        </div>
                    </div>
                }
            </div>
            <ArtifactAssetLoader id={assetId}/>
        </>
    )
}

function ArtifactAssetLoader({id}: {id: UUID}) {
    const actions = useCurrentArtifactsActions();
    const { data } = useArtifactsAsset(id);

    useEffect(() => {
        if (data !== undefined) {
            actions.loadAsset(data);
        }
    }, [data]);

    return null;
}

export default FightAssetArtifactsGenerator;
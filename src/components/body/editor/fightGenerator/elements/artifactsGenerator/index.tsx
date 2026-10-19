import { useQuery } from "@tanstack/react-query"
import { FightGeneratorApi } from "../../api"
import {
    FightAssetArtifactsStore
} from "./store"
import ArtifactsAssetCreator from "./creator"
import { FightAssetArtifactsModel } from "./types"
import styles from "../styles.module.css";
import ArtifactsAssetCostsData from "./costs"
import RequiredArtifactsList from "./required"
import OptionalArtifactsList from "./optional"
import { useEffect } from "react"

function useArtifactsAsset(assetId: number) {
    return useQuery({
        queryKey: ["artifacts_asset", assetId],
        queryFn: async() => {
            return FightGeneratorApi.tryLoadArtifactAsset(assetId)
        }
    })
}

function FightAssetArtifactsGenerator({assetId}: {assetId: number}) {
    const actions = FightAssetArtifactsStore.useActions();
    const artifactsAssetId = FightAssetArtifactsStore.useAssetId();

    async function assetCreated(value: FightAssetArtifactsModel) {
        actions.loadAsset(value);
    }

    return (
        <>
            <div className={styles.artifacts_panel}>
                {
                    artifactsAssetId == undefined ?
                    <ArtifactsAssetCreator assetId={assetId} onCreated={assetCreated}/> :
                    <div style={{width: '100%', display: 'flex', flexDirection: 'row'}}>
                        <div style={{width: '40%', paddingLeft: '10%', display: 'flex', flexDirection: 'column', gap: '15%'}}>
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

function ArtifactAssetLoader({id}: {id: number}) {
    const actions = FightAssetArtifactsStore.useActions();
    const { data } = useArtifactsAsset(id);

    useEffect(() => {
        if (data !== undefined) {
            actions.loadAsset(data);
        }
    }, [data]);

    return null;
}

export default FightAssetArtifactsGenerator;
import { useQuery } from "@tanstack/react-query"
import { FightGeneratorApi } from "../../api"
import { useCurrentArtifactsActions, useCurrentArtifactsAssetId } from "./store"
import ArtifactsAssetCreator from "./creator"
import { FightAssetArtifactsModel } from "./types"
import styles from "../styles.module.css";
import ArtifactsAssetCostsData from "./costs"
import RequiredArtifactsList from "./required"
import OptionalArtifactsList from "./optional"

function useArtifactsAsset(assetId: number) {
    return useQuery({
        queryKey: ["artifacts_asset", assetId],
        queryFn: async() => {
            return FightGeneratorApi.tryLoadArtifactAsset(assetId)
        }
    })
}

function FightAssetArtifactsGenerator(params: {
    assetId: number
}) {
    const actions = useCurrentArtifactsActions();
    const artifactsAssetId = useCurrentArtifactsAssetId();

    const { data } = useArtifactsAsset(params.assetId);
    if (data != undefined) {
        actions.loadAsset(data);
    }

    async function assetCreated(value: FightAssetArtifactsModel) {
        actions.loadAsset(value);
    }

    return (
    <div className={styles.artifacts_panel}>
        {
            artifactsAssetId == undefined ?
            <ArtifactsAssetCreator assetId={params.assetId} assetCreatedCallback={assetCreated}/> :
            <div style={{width: '100%', display: 'flex', flexDirection: 'row'}}>
                <div style={{width: '40%', paddingLeft: '10%'}}>
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
    )
}

export default FightAssetArtifactsGenerator;
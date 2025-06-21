import { AssetGenerationType } from "../../types";
import DifficultyValues from "../common/difficultyValues";
import { useArtifactsBasePowers, useArtifactsGenerationType, useArtifactsPowerGrow, useCurrentArtifactsActions, useCurrentArtifactsAssetId } from "./store";

function ArtifactsAssetCostsData() {
    const artifactsAssetId = useCurrentArtifactsAssetId();
    const basePowers = useArtifactsBasePowers();
    const powersGrow = useArtifactsPowerGrow();
    const generationType = useArtifactsGenerationType();
    const actions = useCurrentArtifactsActions();

    return <div style={{display: 'flex', flexDirection: 'row', width: '100%', gap: '10%'}}>
        <div style={{width: '45%', display: 'flex'}}>
            <DifficultyValues
                name="Artifacts base costs"
                containerId={artifactsAssetId!}
                tauriFunction="update_artifacts_base_cost"
                updateCallback={actions.setBasePowers}
                values={basePowers!}
            />
        </div>
        {
            generationType == AssetGenerationType.Dynamic ?
            <div style={{width: '45%', display: 'flex'}}>
            <DifficultyValues
                name="Artifacts costs grow"
                containerId={artifactsAssetId!}
                tauriFunction="update_artifacts_cost_grow"
                updateCallback={actions.setBasePowers}
                values={powersGrow!}
            />
            </div> :
            null
        }
    </div>
}

export default ArtifactsAssetCostsData;
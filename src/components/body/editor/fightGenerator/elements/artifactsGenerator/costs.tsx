import { AssetGenerationType } from "../../types";
import DifficultyValues from "../common/difficultyValues";
import {
    FightAssetArtifactsStore
} from "./store";
import AverageArtifactsCostTooltip from "./tooltips/averageArtifactsCost";

function ArtifactsAssetCostsData() {
    const artifactsAssetId = FightAssetArtifactsStore.useAssetId();
    const basePowers = FightAssetArtifactsStore.useBasePowers();
    const powersGrow = FightAssetArtifactsStore.usePowersGrow();
    const generationType = FightAssetArtifactsStore.useGenerationType();
    const actions = FightAssetArtifactsStore.useActions();

    return <div style={{display: 'flex', flexDirection: 'row', width: '100%', gap: '10%'}}>
        <div style={{width: '45%', display: 'flex'}}>
            <DifficultyValues
                name="Базовые велью артефактов"
                containerId={artifactsAssetId!}
                tauriFunction="update_artifacts_base_cost"
                updateCallback={actions.setBasePowers}
                values={basePowers!}
                tooltipComponent={{component: AverageArtifactsCostTooltip}}
            />
        </div>
        {
            generationType == AssetGenerationType.Dynamic ?
            <div style={{width: '45%', display: 'flex'}}>
            <DifficultyValues
                name="Приросты велью артефактов"
                containerId={artifactsAssetId!}
                tauriFunction="update_artifacts_cost_grow"
                updateCallback={actions.setBasePowers}
                values={powersGrow!}
                tooltipComponent={{component: AverageArtifactsCostTooltip}}
            />
            </div> :
            null
        }
    </div>
}

export default ArtifactsAssetCostsData;
import DifficultyValues from "./difficultyValues";
import { AssetGenerationType, DifficultyMappedValue, HeroAssetArtifactsModel } from "./types";

function HeroAssetArtifactsCostConfigurator(params: {model: HeroAssetArtifactsModel, updateModelCallback: (value: HeroAssetArtifactsModel) => void}) {

    async function updateBasePowersValue(newValue: DifficultyMappedValue) {
        params.updateModelCallback({...params.model!, base_powers: newValue});
    }

    async function updatePowerGrowValue(newValue: DifficultyMappedValue) {
        params.updateModelCallback({...params.model!, powers_grow: newValue});
    }

    return <div style={{display: 'flex', flexDirection: 'row', gap: 50}}>
        <DifficultyValues
            name="Artifacts base costs per difficulty"
            containerId={params.model.id}
            tauriFunction="update_artifacts_base_cost"
            updateCallback={updateBasePowersValue}
            values={params.model.base_powers}
        />
        {
            params.model.generation_type == AssetGenerationType.Dynamic ?
            <DifficultyValues
                name="Artifacts costs grow per difficulty"
                containerId={params.model.id}
                tauriFunction="update_artifacts_cost_grow"
                updateCallback={updatePowerGrowValue}
                values={params.model.powers_grow!}
            /> :
            null
        }
    </div>
}

export default HeroAssetArtifactsCostConfigurator;
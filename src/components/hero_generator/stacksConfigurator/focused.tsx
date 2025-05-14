import { useEffect, useState } from "react";
import { HeroAssetStackModel } from "./main";
import { invoke } from "@tauri-apps/api/core";
import DifficultyValues from "../artsConfigurator/difficultyValues";
import { AssetGenerationType, DifficultyMappedValue } from "../artsConfigurator/types";
import HeroAssetStackGenerationRules from "./rules";

function HeroAssetFocusedStack(params: {stackId: number}) {
    const [stackData, setStackData] = useState<HeroAssetStackModel | null>(null);

    useEffect(() => {
        loadStackData();
    }, []);

    const loadStackData = async () => {
        await invoke<HeroAssetStackModel | null>("load_stack", {stackId: params.stackId})
            .then((value) => setStackData(value));
    }

    async function updateStackBasePowers(value: DifficultyMappedValue) {
        setStackData({...stackData!, base_powers: value});
    }

    async function updateStackPowersGrow(value: DifficultyMappedValue) {
        setStackData({...stackData!, powers_grow: value});
    }

    return <>
        {
            stackData != null ?
            <div>
                <div style={{display: 'flex', flexDirection: 'row', gap: 50, paddingTop: '5%'}}>
                    <DifficultyValues
                        name="Stack base powers"
                        tauriFunction="update_stack_base_powers"
                        values={stackData.base_powers}
                        updateCallback={updateStackBasePowers}
                        containerId={params.stackId}
                    />
                    {
                        stackData.generation_type == AssetGenerationType.Dynamic ?
                        <DifficultyValues
                            name="Stack powers grow"
                            tauriFunction="update_stack_powers_grow"
                            values={stackData.powers_grow!}
                            updateCallback={updateStackPowersGrow}
                            containerId={params.stackId}
                        /> :
                        null
                    }
                </div>
                <div style={{paddingTop: '10%'}}>
                    <HeroAssetStackGenerationRules model={stackData} updateCallback={setStackData}/>
                </div>
            </div> :
            null
        }
    </>
}

export default HeroAssetFocusedStack;
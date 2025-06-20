import { useEffect } from "react";
import { HeroAssetStackModel } from "./main";
import { invoke } from "@tauri-apps/api/core";
import HeroAssetStackCountConfigurator from "./count";
import HeroAssetStackUnitConfigurator from "./unit";
import useHeroGeneratorStore from "../../../stores/FightGeneratorStore";
import { useShallow } from "zustand/shallow";

function HeroAssetFocusedStack(params: {stackId: number}) {
    const [stackAsset, setStackAsset] = useHeroGeneratorStore(useShallow((state) => [state.currentStackAsset, state.setCurrentStackAsset]));

    useEffect(() => {
        loadStackData();
    }, [params.stackId]);

    const loadStackData = async () => {
        await invoke<HeroAssetStackModel | null>("load_stack", {stackId: params.stackId})
            .then((value) => setStackAsset(value!));
    }

    return <>
        {
            stackAsset != null ?
            <div style={{width: '100%', height: '100%'}}>
                <div style={{width: '100%', height: '49%', display: 'flex', alignContent: 'space-between'}}>
                    <HeroAssetStackCountConfigurator model={stackAsset} updateModelCallback={setStackAsset}/>
                </div>
                <div style={{width: '100%', height: '49%', paddingTop: '2%'}}>
                    <HeroAssetStackUnitConfigurator model={stackAsset} updateCallback={setStackAsset}/>
                </div>
            </div> :
            // <div>
            //     <div style={{display: 'flex', flexDirection: 'row', gap: 50, paddingTop: '5%'}}>
            //         <DifficultyValues
            //             name="Stack base powers"
            //             tauriFunction="update_stack_base_powers"
            //             values={stackData.base_powers}
            //             updateCallback={updateStackBasePowers}
            //             containerId={params.stackId}
            //         />
            //         {
            //             stackData.power_based_generation_type == AssetGenerationType.Dynamic ?
            //             <DifficultyValues
            //                 name="Stack powers grow"
            //                 tauriFunction="update_stack_powers_grow"
            //                 values={stackData.powers_grow!}
            //                 updateCallback={updateStackPowersGrow}
            //                 containerId={params.stackId}
            //             /> :
            //             null
            //         }
            //     </div>
            //     <div style={{paddingTop: '10%'}}>
            //         <HeroAssetStackGenerationRules model={stackData} updateCallback={setStackData}/>
            //     </div>
            // </div> :
            null
        }
    </>
}

export default HeroAssetFocusedStack;
import { useEffect, useState } from "react";
import { HeroAssetArtifactsModel } from "./types";
import { invoke } from "@tauri-apps/api/core";
import HeroAssetArtifactsInitializator from "./initializator";
import HeroAssetArtifactsCostConfigurator from "./costs";
import HeroAssetArtifactsLists from "./lists";
import HeroAssetArtifactsConfiguratorHeader from "./header";

function HeroAssetArtifactsConfigurator(params: {assetId: number}) {
    const [artifactsData, setArtifactsData] = useState<HeroAssetArtifactsModel | null>(null);

    useEffect(() => {
        loadArtifactsData();
    }, []);

    const loadArtifactsData = async () => {
        await invoke<HeroAssetArtifactsModel | null>("try_load_artifacts_data_for_asset", {assetId: params.assetId})
            .then((value) => setArtifactsData(value));
    }

    async function artifactsDataInitialized(value: HeroAssetArtifactsModel) {
        setArtifactsData(value);
    }

    return <div style={{width: '100%', display: 'flex', flexDirection: 'column', gap: 25}}>
        {
            artifactsData == null ? 
            <HeroAssetArtifactsInitializator assetId={params.assetId} initializedCallback={artifactsDataInitialized}/> :
            <div style={{height: '100%'}}>
                <div style={{height: '15%'}}>
                    <HeroAssetArtifactsConfiguratorHeader/>
                </div>
                <div style={{height: '82%', paddingTop: '3%'}}> 
                    <HeroAssetArtifactsCostConfigurator model={artifactsData} updateModelCallback={setArtifactsData}/>
                    <HeroAssetArtifactsLists model={artifactsData} updateCallback={setArtifactsData}/>
                </div>
            </div> 
        }
    </div>
}

export default HeroAssetArtifactsConfigurator;
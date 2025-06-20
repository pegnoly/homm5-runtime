import { Button } from "antd";
import useHeroGeneratorStore from "../../stores/FightGeneratorStore";
import { invoke } from "@tauri-apps/api/core";

function HeroGeneratorGlobals() {
    const currentAsset = useHeroGeneratorStore((state) => state.currentAssetId);

    return <Button 
        disabled={!currentAsset} 
        onClick={() => invoke("generate_current_hero_script", {assetId: currentAsset})}
    >Generate script for current hero</Button>
}

export default HeroGeneratorGlobals;
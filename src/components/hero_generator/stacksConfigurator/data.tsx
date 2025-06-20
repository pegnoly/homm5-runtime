import { Typography } from "antd";
import useHeroGeneratorStore from "../../../stores/FightGeneratorStore";
import { countGenerationTypeNames, StackCountGenerationType, unitGenerationTypeNames } from "./main";
import { AssetGenerationType } from "../artsConfigurator/types";

function HeroAssetCurrentStackData() {
    const currentAsset = useHeroGeneratorStore((state) => state.currentStackAsset);

    return <>
    {
        currentAsset != null ?
        <Typography.Text style={{fontFamily: 'cursive', fontSize: 16, fontWeight: 'bolder', fontStretch: 'expanded'}}>
            <span>
                {` unit generation -`}
            </span>
            <span style={{color: 'green'}}>
                {` ${unitGenerationTypeNames.get(currentAsset?.type_generation_mode!)}`}
            </span>
            <span style={{color: 'black'}}>
                <br></br>
                {`count generation -`}
            </span>
            <span style={{color: 'green'}}>
                {` ${countGenerationTypeNames.get(currentAsset?.count_generation_mode!)}`}
                <br></br>
            </span>
            {
                currentAsset!.count_generation_mode == StackCountGenerationType.PowerBased ?
                <>
                    <span style={{color: 'black'}}>
                        {`power generation - `}
                    </span>
                    <span style={{color: 'green'}}>
                        {currentAsset!.power_based_generation_type == AssetGenerationType.Static ? "Static" : "Dynamic"}
                    </span>
                </> :
                null
            }
        </Typography.Text> :
        null
    }
    </>
}

export default HeroAssetCurrentStackData;
import { AssetGenerationType } from "../../types";
import { useCountGenerationMode, useCurrentStackId, usePowerBasetGenerationType, useTypeGenerationMode } from "./store";
import { countGenerationTypeNames, StackCountGenerationType, unitGenerationTypeNames } from "./types";
import { Text } from "@mantine/core";

function FightAssetCurrentStackData() {
    const currentStackId = useCurrentStackId();
    const typeGenerationMode = useTypeGenerationMode();
    const countGenerationMode = useCountGenerationMode();
    const powerBasedGenerationMode = usePowerBasetGenerationType();

    return <>
    {
        currentStackId != undefined ?
        <div style={{display: 'flex'}}>
            <Text style={{fontFamily: 'cursive', fontSize: 14, fontWeight: 'bolder', fontStretch: 'expanded'}}>
                <span>
                    {` unit generation -`}
                </span>
                <span style={{color: 'green'}}>
                    {` ${unitGenerationTypeNames.get(typeGenerationMode!)}`}
                </span>
                <span style={{color: 'black'}}>
                    <br></br>
                    {`count generation -`}
                </span>
                <span style={{color: 'green'}}>
                    {` ${countGenerationTypeNames.get(countGenerationMode!)}`}
                    <br></br>
                </span>
                {
                    countGenerationMode == StackCountGenerationType.PowerBased ?
                    <>
                        <span style={{color: 'black'}}>
                            {`power generation - `}
                        </span>
                        <span style={{color: 'green'}}>
                            {powerBasedGenerationMode == AssetGenerationType.Static ? "Static" : "Dynamic"}
                        </span>
                    </> :
                    null
                }
            </Text>
        </div> :
        null
    }
    </>
}

export default FightAssetCurrentStackData;
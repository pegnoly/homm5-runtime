import { useParams } from "react-router";
import FightAssetStackGenerator from "../../../fightGenerator/elements/stacksGenerator";
import { UUID } from "crypto";

function BankVariantFocused() {
    const { id } = useParams();

    return (
    <>
        <FightAssetStackGenerator assetId={id as UUID}/>
    </>
    )
}

export default BankVariantFocused;
import { useParams } from "react-router";
import FightAssetStackGenerator from "../../../fightGenerator/elements/stacksGenerator";
function BankVariantFocused() {
    const { id } = useParams();

    return (
    <>
        <FightAssetStackGenerator assetId={parseInt(id!)}/>
    </>
    )
}

export default BankVariantFocused;
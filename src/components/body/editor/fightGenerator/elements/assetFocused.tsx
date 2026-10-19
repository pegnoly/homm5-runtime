import { useLocation, useParams } from "react-router";
import styles from './styles.module.css'
import FightAssetStackGenerator from "./stacksGenerator";
import { FightAssetStore } from "../store";
import FightAssetArtifactsGenerator from "@/components/body/editor/fightGenerator/elements/artifactsGenerator";

function FightAssetFocused() {
    const { id } = useParams();
    const location = useLocation();
    const { assetName } = location.state || {};

    const actions = FightAssetStore.useActions();

    if (assetName != undefined && id != undefined) {
        actions.setCurrentAssetId(parseInt(id));
        actions.setCurrentAssetName(assetName as string);
    }

    return (
    <div className={styles.focused_panel}>
        <FightAssetStackGenerator assetId={parseInt(id!)}/>
        <FightAssetArtifactsGenerator assetId={parseInt(id!)}/>
    </div>
    )
}

export default FightAssetFocused;
import { useLocation, useParams } from "react-router";
import styles from './styles.module.css'
import FightAssetStackGenerator from "./stacksGenerator";
import FightAssetArtifactsGenerator from "./artifactsGenerator";
import { useFightAssetActions } from "../store";

function FightAssetFocused() {
    const { id } = useParams();
    const location = useLocation();
    const { assetName } = location.state || {};

    const actions = useFightAssetActions();

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
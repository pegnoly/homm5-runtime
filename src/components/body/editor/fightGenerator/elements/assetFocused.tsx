import { useLocation, useParams } from "react-router";
import styles from './styles.module.css'
import FightAssetStackGenerator from "./stacksGenerator";
import FightAssetArtifactsGenerator from "./artifactsGenerator";
import { useFightAssetActions } from "../store";
import { UUID } from "crypto";

function FightAssetFocused() {
    const { id } = useParams();
    const location = useLocation();
    const { assetName } = location.state || {};

    const actions = useFightAssetActions();

    if (assetName != undefined && id != undefined) {
        actions.setCurrentAssetId(id as UUID);
        actions.setCurrentAssetName(assetName as string);
    }

    return (
    <div className={styles.focused_panel}>
        <FightAssetStackGenerator assetId={id as UUID}/>
        <FightAssetArtifactsGenerator assetId={id as UUID}/>
    </div>
    )
}

export default FightAssetFocused;
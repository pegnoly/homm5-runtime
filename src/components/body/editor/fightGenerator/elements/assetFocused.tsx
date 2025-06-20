import { useParams } from "react-router";
import styles from './styles.module.css'
import FightAssetStackGenerator from "./stacksGenerator";

function FightAssetFocused() {
    const { id } = useParams();

    return (
    <div className={styles.focused_panel}>
        <FightAssetStackGenerator assetId={parseInt(id!)}/>
    </div>
    )
}

export default FightAssetFocused;
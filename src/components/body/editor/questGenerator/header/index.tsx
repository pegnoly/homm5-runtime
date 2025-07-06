import { useQuery } from "@tanstack/react-query";
import styles from "../styles.module.css";
import QuestCreator from "./creator";
import QuestSelector from "./selector";
import { QuestGeneratorApi } from "../api";
import { useQuestsActions } from "../store";
import { useCurrentMapId } from "../../../../../stores/common";

function useQuestsData(mapId: number) {
    return useQuery({
        queryKey: ["quests", mapId],
        queryFn: async() => {
            return QuestGeneratorApi.loadQuests();
        }
    })
}

function QuestGeneratorHeader() {
    const currentMapId = useCurrentMapId(); // weird but i need this now for compability
    const actions = useQuestsActions();

    const { data } = useQuestsData(currentMapId!);
    if (data != undefined) {
        console.log("Quests loaded: ", data)
        actions.loadQuests(data);
    }

    return (
    <div className={styles.header}>
        <div className={styles.manage_panel}>
            <QuestCreator/>
            <QuestSelector/>
        </div>
    </div>
    )
}

export default QuestGeneratorHeader;
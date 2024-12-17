import QuestInitializator from "./initialize"
import QuestScriptName from "./script"
import QuestDirectory from "./directory"
import QuestName from "./name"
import QuestDesc from "./desc"

function QuestData() {

    return <>   
        <QuestInitializator/>
        <QuestScriptName/>
        <QuestDirectory/>
        <QuestName/>
        <QuestDesc/>
    </>
}

export default QuestData;
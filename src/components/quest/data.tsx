import QuestInitializator from "./initialize"
import QuestScriptName from "./script"
import QuestDirectory from "./directory"
import QuestName from "./name"
import QuestDesc from "./desc"
import QuestProps from "./props"

function QuestData() {

    return <>   
        <QuestInitializator/>
        <QuestScriptName/>
        <QuestDirectory/>
        <QuestName/>
        <QuestDesc/>
        <QuestProps/>
    </>
}

export default QuestData;
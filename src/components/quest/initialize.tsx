import QuestCreationProvider from "../../contexts/questCreation";
import QuestCreator from "./creator";
import QuestLoader from "./loader";

function QuestInitializator() {
    return <div style={{display: 'flex', flexDirection: 'row', gap: 10, justifyContent: 'center'}}>
        <QuestCreationProvider>
            <QuestCreator/>
        </QuestCreationProvider>
        <QuestLoader/>
    </div>
}

export default QuestInitializator;
import CreatureNumberPropsEditor from "./numberProps";
import CreaturesInteractionEditor from "./creaturesInteraction";

function CreatureEditorBody() {
    return (
        <div style={{width: '100%', display: 'flex', flexDirection: 'row'}}>
            <CreatureNumberPropsEditor/>
            <CreaturesInteractionEditor/>
        </div>
    )
}

export default CreatureEditorBody;
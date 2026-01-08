import CreatureNumberPropsEditor from "./numberProps";
import CreaturesInteractionEditor from "./creaturesInteraction";
import CreatureCostEditor from "./cost";

function CreatureEditorBody() {
    return (
        <div style={{width: '100%', display: 'flex', flexDirection: 'row', gap: '5%'}}>
            <CreatureNumberPropsEditor/>
            <CreaturesInteractionEditor/>
            <div style={{width: '40%', display: 'flex', flexDirection: 'column', paddingTop: '2%'}}>
                <CreatureCostEditor/>
            </div>
        </div>
    )
}

export default CreatureEditorBody;
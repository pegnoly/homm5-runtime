import DialogCreator from "./Creator";
import DialogLoader from "./Loader";
import SpeakerCreator from "./speaker/SpeakerCreator";

function DialogInitializator() {

    return <div style={{display: 'flex', flexDirection: 'row', justifyContent: 'center', gap: 10}}>
        <DialogCreator/>
        <DialogLoader/>
        <SpeakerCreator/>
    </div>
}

export default DialogInitializator;
import styles from '../../styles.module.css';
import CreatureEditorBody from './body';
import CreatureEditorSidebar from "./sidebar";
import {Route, Routes} from "react-router";

function CreatureEditorLayout() {

    return (
        <div className={styles.editor_layout}>
            <div style={{width: '100%', height: '100%', display: "flex", flexDirection: 'row', padding: '1%', gap: '2%'}}>
                <div style={{width: '22%', height: '100%'}}>
                    <CreatureEditorSidebar/>
                </div>
                <div style={{width: '77%', height: '100%'}}>
                    <Routes>
                        <Route path='focused/:id/*' element={<CreatureEditorBody/>} />
                    </Routes>
                </div>
            </div>
        </div>
    )
}

export default CreatureEditorLayout;
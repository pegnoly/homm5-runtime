import { Route, Routes } from 'react-router';
import Sidebar from './sidebar';
import classes from './styles.module.css'
import EditorMain from './editor';
import EditorDefault from './editor/default';

function Body() {
    return (
    <div className={classes.main}>
        <Sidebar/>
        <Routes>
            <Route path='/' element={<EditorDefault/>}/>
            <Route path='/editor/:state/*' element={<EditorMain/>}/>
        </Routes>
    </div>
    )
}

export default Body;
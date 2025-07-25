import { Route, Routes } from 'react-router';
import styles from '../../styles.module.css';
import BanksList from './list';
import BankFocused from './focused';

function BankGeneratorLayout() {
    return (
    <div className={styles.editor_layout}>
        <Routes>
            <Route path='/' element={<BanksList/>}/>
            <Route path="/focused/:id/*" element={<BankFocused/>}/>
        </Routes>
    </div>
    )
}

export default BankGeneratorLayout;
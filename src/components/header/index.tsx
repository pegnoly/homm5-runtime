import HeaderCore from './core';
import MapSelector from './mapSelector';
import RepackersPanel from './repacker';
import classes from './styles.module.css'

function Header() {

    return (
    <div className={classes.main}>
        <HeaderCore/>
        <RepackersPanel/>
        <MapSelector/>
    </div>
    )
}

export default Header;
import HeaderCore from './core';
import MapSelector from './mapSelector';
import classes from './styles.module.css'
import {Button} from "@mantine/core";
import {invoke} from "@tauri-apps/api/core";

function Header() {

    return (
    <div className={classes.main}>
        <HeaderCore/>
        {/*<RepackersPanel/>*/}
        <MapSelector/>
        <Button radius={0} onClick={() => invoke("test_creature_sync")}>
            Sync creatures
        </Button>
    </div>
    )
}

export default Header;
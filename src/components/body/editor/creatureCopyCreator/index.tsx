import { Button, ButtonGroup, List, Text } from "@mantine/core";
import styles from "../../styles.module.css";
import CreatableCreatureItem from "./model";
import CreatureGenerationParams from "./generationParams";
import CreatureCopyCreator from "./store";
import CreatureGenerationInitializator from "./initializator";

function CreatureCopyCreatorLayout() {

    const models = CreatureCopyCreator.useModels();
    const currentId = CreatureCopyCreator.useCurrentId();
    const actions = CreatureCopyCreator.useActions();

    return (
    <div className={styles.editor_layout}>
        <div style={{width: '100%', height: '100%', display: 'flex', flexDirection: 'column'}}>
            <div style={{height: '10%', width: '100%', display: 'flex', flexDirection: 'row', justifyContent: 'space-between'}}>
                <ButtonGroup>
                    <CreatureGenerationInitializator/>
                    <Button radius={0} bg="dark" disabled={currentId == undefined}>Save session</Button>
                    <Button radius={0} bg="cyan" disabled={currentId == undefined}>Load session</Button>
                    <Button 
                        disabled={currentId == undefined}
                        radius={0} 
                        onClick={() => {{
                            actions.addModel(currentId! + 1);
                        }}}
                    >Add</Button>
                </ButtonGroup>
                <Text>{`Generation session active with start id ${currentId}`}</Text>
            </div>
            <div style={{height: '90%', width: '100%'}}>
                <div style={{width: '100%', height: '100%', display: 'flex', flexDirection: 'row'}}>
                    <div style={{width: '75%', height: '90%', overflow: 'auto'}}>
                        <List>{models.map((item, index) => (
                            <div key={index}>
                                <CreatableCreatureItem model={item}/>
                            </div>
                        ))}</List>
                    </div>
                    <div style={{width: '25%', height: '100%'}}>
                        <CreatureGenerationParams/>
                    </div>
                </div>
            </div>
        </div>
    </div>
    )
}

export default CreatureCopyCreatorLayout;
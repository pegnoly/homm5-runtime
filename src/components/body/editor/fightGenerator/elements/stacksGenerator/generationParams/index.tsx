import { useConcreteCreatures, useCurrentStackActions, useCurrentStackId, useTypeGenerationMode } from "../store";
import styles from '../../styles.module.css'
import { StackUnitGenerationType } from "../types";
import TiersSelector from "./tiersSelector";
import TownSelector from "./townSelector";
import AssetStackRuleSelector from "./ruleSelector";
import FightAssetStackStatsData from "./statSelector";
import { Accordion, AccordionControl, AccordionItem, AccordionPanel, ComboboxItem, Group, MultiSelect, OptionsFilter, Select, Stack } from "@mantine/core";
import { useState } from "react";
import { TownType } from "../../../types";
import useGameDataStore from "../../../../../../../stores/GameDataStore";
import { useMutation } from "@tanstack/react-query";
import { FightGeneratorApi } from "../../../api";

enum ParamsSelectionMode {
    Towns = "Towns",
    Tiers = "Tiers",
    Rules = "Rules"
}

function FightAssetStackParamsData() {
    const unitGenerationMode = useTypeGenerationMode();
    const currentStackId = useCurrentStackId();

    return (
    <div className={styles.stack_params_panel}>
        {
            currentStackId == undefined ? ( null ) :
            (
                unitGenerationMode == StackUnitGenerationType.TierSlotBased ?
                <TierSlotBasedGenerationData/> :
                <ConcreteUnitsSelection/>
            )
        }
    </div> 
    )
}

function TierSlotBasedGenerationData() {
    const currentStackId = useCurrentStackId();
    return (
        <>
        {
            currentStackId != undefined ?
            <div style={{width: '100%', display: 'flex', flexDirection: 'row'}}>
                <div 
                style={{
                    display: 'flex', flexDirection: 'column', gap: '2%', width: '50%', overflow: 'auto', alignContent: 'center', justifyItems: 'center', justifySelf: 'center'
                }}>
                    <Accordion defaultValue={ParamsSelectionMode.Towns}>
                        <AccordionItem key={0} value={ParamsSelectionMode.Towns}>
                            <AccordionControl>
                                Towns selection
                            </AccordionControl>
                            <AccordionPanel>
                                <TownSelector/>
                            </AccordionPanel>
                        </AccordionItem>
                        <AccordionItem key={1} value={ParamsSelectionMode.Tiers}>
                            <AccordionControl>
                                Tiers selection
                            </AccordionControl>
                            <AccordionPanel>
                                <TiersSelector/>
                            </AccordionPanel>
                        </AccordionItem>
                        <AccordionItem key={2} value={ParamsSelectionMode.Rules}>
                            <AccordionControl>
                                Rules selection
                            </AccordionControl>
                            <AccordionPanel>
                                <AssetStackRuleSelector/>
                            </AccordionPanel>
                        </AccordionItem>
                    </Accordion>
                </div>
                <div style={{width: '50%'}}>
                    <FightAssetStackStatsData/>
                </div>
            </div>
            :
            null
        }
        </>
    )
}

function ConcreteUnitsSelection() {
    const [currentTown, setCurrentTown] = useState<TownType | null>(null);
    const [currentTier, setCurrentTier] = useState<number | null>(null);
    
    return (
    <Stack>
        <Group>
            <ConcreteCreatureTownSelector currentTown={currentTown} townSelectedCallback={setCurrentTown}/>
            <ConcreteCreatureTierSelector currentTier={currentTier} tierSelectedCallback={setCurrentTier}/>
        </Group>
        <CreatureSelector tier={currentTier} town={currentTown}/>
    </Stack>
    )
}

function ConcreteCreatureTownSelector(params: {
    currentTown: TownType | null,
    townSelectedCallback: (value: TownType) => void
}) {

    return (
    <Select
        size="sm"
        label="Select concrete creature town"
        value={params.currentTown}
        onChange={(value) => params.townSelectedCallback(value as TownType)}
        data={[
            {value: TownType.TownAcademy, label: "Academy"}, 
            {value: TownType.TownDungeon, label: "Dungeon"},
            {value: TownType.TownHeaven, label: "Heaven"},
            {value: TownType.TownInferno, label: "Inferno"},
            {value: TownType.TownFortress, label: "Fortress"},
            {value: TownType.TownPreserve, label: "Preserve"},
            {value: TownType.TownNecromancy, label: "Necromancy"},
            {value: TownType.TownStronghold, label: "Stronghold"},
            {value: TownType.TownNoType, label: "Neutral"},
        ]}
    />
    )
}

function ConcreteCreatureTierSelector(params: {
    currentTier: number | null,
    tierSelectedCallback: (value: number) => void
}) {
    return (
    <Select
        label="Select concrete creature tier"
        value={params.currentTier?.toString()}
        onChange={(value) => params.tierSelectedCallback(parseInt(value!))}
        data={[
            {value: "1", label: "Tier 1"},
            {value: "2", label: "Tier 2"},
            {value: "3", label: "Tier 3"},
            {value: "4", label: "Tier 4"},
            {value: "5", label: "Tier 5"},
            {value: "6", label: "Tier 6"},
            {value: "7", label: "Tier 7"},
        ]}
    />
    )
}

export type UpdateConcreteCreaturesPayload = {
    stackId: number,
    creatures: number []
}

function CreatureSelector(params: {
    town: TownType | null,
    tier: number | null
}) {
    const currentStackId = useCurrentStackId();
    const concreteCreatures = useConcreteCreatures();
    const actions = useCurrentStackActions();
    const creatures = useGameDataStore(state => state.creatures);

    const mutation = useMutation({
        mutationFn: async(payload: UpdateConcreteCreaturesPayload) => {
            return FightGeneratorApi.updateStackConcreteCreatures(payload)
        },
        onSuccess(_data, variables, _context) {
            actions.setConcreteCreatures(variables.creatures);
        },
    });

    const optionsFilter: OptionsFilter = ({ options }) => {
        return (options as ComboboxItem[]).filter((option) => {
            const creature = creatures.find(c => c.id == parseInt(option.value));
            return creature?.town == params.town && creature?.tier == params.tier
        });
    };

    return (
    <MultiSelect
        radius={0}
        pr="sm"
        onChange={(values) => mutation.mutate({stackId: currentStackId!, creatures: values.map(v => parseInt(v))})}
        disabled={!params.town || !params.tier || mutation.isPending}
        // searchable
        filter={optionsFilter}
        value={creatures.filter(i => concreteCreatures?.ids.includes(i.id)).map(i => i.id.toString())}
        data={creatures.map(c => ({
            value: c.id.toString(), label: c.name
        }))}
    />
    )
}

export default FightAssetStackParamsData;
import {
    FightStackGeneratorStore,
} from "../store";
import styles from '../../styles.module.css'
import { SelectionType } from "../types";
import TiersSelector from "./tiersSelector";
import TownSelector from "./townSelector";
import AssetStackRuleSelector from "./ruleSelector";
import FightAssetStackStatsData from "./statSelector";
import {
    Accordion,
    AccordionControl,
    AccordionItem,
    AccordionPanel,
    ComboboxItem,
    Group,
    MultiSelect,
    OptionsFilter,
    Select,
    Stack
} from "@mantine/core";
import {useState} from "react";
import {TownTypeExtended} from "../../../types";
import useGameDataStore from "../../../../../../../stores/GameDataStore";
import {useMutation} from "@tanstack/react-query";
import {FightGeneratorApi} from "../../../api";

enum ParamsSelectionMode {
    Towns = "Towns",
    Tiers = "Tiers",
    Rules = "Rules"
}

function FightAssetStackParamsData() {
    const selectionType = FightStackGeneratorStore.useSelectionType();
    const id = FightStackGeneratorStore.useCurrentId();

    return (
    <div className={styles.stack_params_panel}>
        {
            id == undefined ? null :
            (
                selectionType == SelectionType.TierSlotBased ?
                <TierSlotBasedGenerationData/> :
                <SpecificUnitsSelection/>
            )
        }
    </div> 
    )
}

function TierSlotBasedGenerationData() {
    const id = FightStackGeneratorStore.useCurrentId();
    return (
        <>
        {
            id != undefined ?
            <div style={{width: '100%', display: 'flex', flexDirection: 'row'}}>
                <div 
                style={{
                    display: 'flex', flexDirection: 'column', gap: '2%', width: '50%', overflow: 'auto', alignContent: 'center', justifyItems: 'center', justifySelf: 'center'
                }}>
                    <Accordion defaultValue={ParamsSelectionMode.Towns}>
                        <AccordionItem key={0} value={ParamsSelectionMode.Towns}>
                            <AccordionControl>
                                Выбор городов юнитов
                            </AccordionControl>
                            <AccordionPanel>
                                <TownSelector/>
                            </AccordionPanel>
                        </AccordionItem>
                        <AccordionItem key={1} value={ParamsSelectionMode.Tiers}>
                            <AccordionControl>
                                Выбор уровней юнитов
                            </AccordionControl>
                            <AccordionPanel>
                                <TiersSelector/>
                            </AccordionPanel>
                        </AccordionItem>
                        <AccordionItem key={2} value={ParamsSelectionMode.Rules}>
                            <AccordionControl>
                                Выбор правил генерации
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

function SpecificUnitsSelection() {
    const [currentTown, setCurrentTown] = useState<TownTypeExtended | null>(null);
    const [currentTier, setCurrentTier] = useState<number | null>(null);
    
    return (
    <Stack>
        <Group>
            <SpecificCreatureTownSelector currentTown={currentTown} townSelectedCallback={setCurrentTown}/>
            <SpecificCreatureTierSelector currentTier={currentTier} tierSelectedCallback={setCurrentTier}/>
        </Group>
        <CreatureSelector tier={currentTier} town={currentTown}/>
    </Stack>
    )
}

function SpecificCreatureTownSelector(params: {
    currentTown: TownTypeExtended | null,
    townSelectedCallback: (value: TownTypeExtended) => void
}) {

    return (
    <Select
        searchable
        size="sm"
        label="Выберите город юнитов"
        value={params.currentTown}
        onChange={(value) => params.townSelectedCallback(value as TownTypeExtended)}
        data={[
            {value: TownTypeExtended.TownAcademy, label: "Academy"},
            {value: TownTypeExtended.TownDungeon, label: "Dungeon"},
            {value: TownTypeExtended.TownHeaven, label: "Heaven"},
            {value: TownTypeExtended.TownInferno, label: "Inferno"},
            {value: TownTypeExtended.TownFortress, label: "Fortress"},
            {value: TownTypeExtended.TownPreserve, label: "Preserve"},
            {value: TownTypeExtended.TownNecromancy, label: "Necromancy"},
            {value: TownTypeExtended.TownStronghold, label: "Stronghold"},
            {value: TownTypeExtended.TownBastion, label: "Bastion"},
            {value: TownTypeExtended.TownSanctuary, label: "Sanctuary"},
            {value: TownTypeExtended.TownRenegades, label: "Renegades"},
            {value: TownTypeExtended.TownNoType, label: "Neutral"},
        ]}
    />
    )
}

function SpecificCreatureTierSelector(params: {
    currentTier: number | null,
    tierSelectedCallback: (value: number) => void
}) {
    return (
    <Select
        label="Выберите уровень юнитов"
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
    town: TownTypeExtended | null,
    tier: number | null
}) {
    const id = FightStackGeneratorStore.useCurrentId();
    const specificCreatures = FightStackGeneratorStore.useSpecificCreatures();
    const actions = FightStackGeneratorStore.useActions();
    const creatures = useGameDataStore(state => state.creatures);

    const mutation = useMutation({
        mutationFn: async(payload: UpdateConcreteCreaturesPayload) => {
            return FightGeneratorApi.updateStackConcreteCreatures(payload)
        },
        onSuccess(_data, variables, _context) {
            actions.setSpecificCreatures(variables.creatures);
        },
    });

    const optionsFilter: OptionsFilter = ({ options }) => {
        return (options as ComboboxItem[]).filter((option) => {
            const creature = creatures.find(c => c.id == parseInt(option.value));
            return creature?.town_extended == params.town && creature?.tier == params.tier
        });
    };

    return (
    <MultiSelect
        radius={0}
        pr="sm"
        onChange={(values) => mutation.mutate({stackId: id!, creatures: values.map(v => parseInt(v))})}
        disabled={!params.town || !params.tier || mutation.isPending}
        // searchable
        filter={optionsFilter}
        value={creatures.filter(i => specificCreatures?.ids.includes(i.id)).map(i => i.id.toString())}
        data={creatures.map(c => ({
            value: c.id.toString(), label: c.inner_name ? c.inner_name : c.name
        }))}
    />
    )
}

export default FightAssetStackParamsData;
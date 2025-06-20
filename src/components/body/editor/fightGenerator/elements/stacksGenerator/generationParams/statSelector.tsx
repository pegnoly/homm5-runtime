import { Accordion, AccordionControl, AccordionItem, AccordionPanel, ActionIcon, Button, Center, Group, MultiSelect, SegmentedControl, Text } from "@mantine/core";
import { useCurrentStackActions, useCurrentStackId, useStatParams, useTypeGenerationMode } from "../store";
import { StackUnitGenerationType, StatGenerationRule, StatGenerationType } from "../types";
import { useMutation, useQuery } from "@tanstack/react-query";
import { FightGeneratorApi } from "../../../api";
import { IconRowRemove, IconTagPlus } from "@tabler/icons-react";

function useStatElementsQuery(stackId: number) {
    return useQuery({
        queryKey: ["stack_stat_elements", stackId],
        queryFn: async() => {
            return FightGeneratorApi.loadStatParamElements(stackId);
        }
    })
}

function FightAssetStackStatsData() {
    const currentStackId = useCurrentStackId();
    const typeGenerationMode = useTypeGenerationMode();
    const actions = useCurrentStackActions();

    if (typeGenerationMode != StackUnitGenerationType.TierSlotBased || currentStackId == undefined) {
        return null;
    }

    const { data } = useStatElementsQuery(currentStackId);

    if (data != undefined) {
        actions.loadStatsParams(data!);
    }

    return (
    <>
        <StatParamsCreator/>
        <StatParamsList/>
    </>
    )
}

function StatParamsCreator() {
    const currentStackId = useCurrentStackId();
    const actions = useCurrentStackActions();

    const addParamMutation = useMutation({
        mutationFn: async(stackId: number) => {
            return FightGeneratorApi.createStatParamElement(stackId);
        },
        onSuccess(data, _variables, _context) {
            actions.addStatParam(data);
        },
    })
    
    return (
    <Group justify="center">
        <Text>Create stat param</Text>
        <Button onClick={() => addParamMutation.mutate(currentStackId!)}>
            <IconTagPlus/>
        </Button>
    </Group>
    )
}

function StatParamsList() 
{
    const statParams = useStatParams();
    const actions = useCurrentStackActions();

    const mutation = useMutation({
        mutationFn: async(elementId: number) => {
            return FightGeneratorApi.removeStatParamElement(elementId);
        },
        onSuccess(_data, variables, _context) {
            actions.removeStatParam(variables);
        },
    })

    return (
    <div style={{display: 'flex', flexDirection: 'column', padding: '5%', overflow: 'auto', height: '75%'}}>
        <Accordion 
            defaultValue={(statParams == undefined || statParams?.length == 0) ? null : statParams![0].id.toString()}>{statParams?.map((param, index) => (
            <AccordionItem key={param.id} value={param.id.toString()}>
                <Center>
                    <AccordionControl>{`Stat param ${index}`}</AccordionControl>
                    <ActionIcon onClick={() => mutation.mutate(param.id)}>
                        <IconRowRemove/>
                    </ActionIcon>
                </Center>
                <AccordionPanel>
                    <StatParamsConfigurator elementId={param.id.toString()}/>
                </AccordionPanel>
            </AccordionItem>
        ))}</Accordion>
    </div>
    )
}

function StatParamsConfigurator(params: {
    elementId: string
}) {
    return (
    <>
        <RuleSelector elementId={params.elementId}/>
        <StatsSelector elementId={params.elementId}/>
    </>
    )
}

function RuleSelector(params: {
    elementId: string
}) {
    const statParams = useStatParams();
    const actions = useCurrentStackActions();
    
    const mutation = useMutation({
        mutationFn: async(rule: StatGenerationRule) => {
            return FightGeneratorApi.updateStatParamElementRule(parseInt(params.elementId), rule);
        },
        onSuccess(_data, variables, _context) {
            actions.updateStatParamElementRule(parseInt(params.elementId), variables)
        },
    })

    return (
    <SegmentedControl
        size="xs"
        value={statParams?.find(s => s.id == parseInt(params.elementId))?.rule}
        onChange={(value) => mutation.mutate(value as StatGenerationRule)}
        data={[
            { value: StatGenerationRule.MaxBy, label: "Max by" },
            { value: StatGenerationRule.MinBy, label: "Min by" },
        ]}
    /> 
    )
}

function StatsSelector(params: {
    elementId: string
}) {
    const statParams = useStatParams();
    const actions = useCurrentStackActions();

    const mutation = useMutation({
        mutationFn: async(stats: StatGenerationType []) => {
            return FightGeneratorApi.updateStatParamElementStats(parseInt(params.elementId), stats);
        },
        onSuccess(_data, variables, _context) {
            actions.updateStatParamElementStats(parseInt(params.elementId), variables)
        },
    })

    return (
        <MultiSelect
            disabled={mutation.isPending}
            radius={0}
            miw={100}
            size="xs"
            label="Select rules"
            value={statParams?.find(s => s.id == parseInt(params.elementId))?.stats.values}
            onChange={(value) => {
                mutation.mutate(value.map(v => v as StatGenerationType));
            }}
            data={[
                {value: StatGenerationType.Hitpoints, label: "Hitpoints"},
                {value: StatGenerationType.Attack, label: "Attack"},
                {value: StatGenerationType.Defence, label: "Defence"},
                {value: StatGenerationType.Initiative, label: "Initiative"},
                {value: StatGenerationType.Speed, label: "Speed"},
            ]}
        />
    )
}

export default FightAssetStackStatsData;
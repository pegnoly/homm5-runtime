import { useDisclosure } from "@mantine/hooks";
import { AssetGenerationType } from "../../types";
import { useCountGenerationMode, useCurrentStackActions, useCurrentStackId, usePowerBasetGenerationType, useTypeGenerationMode } from "./store";
import { countGenerationTypeNames, StackCountGenerationType, StackUnitGenerationType, unitGenerationTypeNames } from "./types";
import { ActionIcon, Button, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, Select, Stack, Text, Tooltip } from "@mantine/core";
import { IconEdit } from "@tabler/icons-react";
import { useState } from "react";
import { useMutation } from "@tanstack/react-query";
import { FightGeneratorApi } from "../../api";

function FightAssetCurrentStackData() {
    const currentStackId = useCurrentStackId();
    const typeGenerationMode = useTypeGenerationMode();
    const countGenerationMode = useCountGenerationMode();
    const powerBasedGenerationMode = usePowerBasetGenerationType();

    return <>
    {
        currentStackId != undefined ?
        <div style={{display: 'flex', flexDirection: 'row', gap: 10}}>
            <DataDisplayer unitGenerationType={typeGenerationMode!} countGenerationType={countGenerationMode!} countGenerationMode={powerBasedGenerationMode!}/>
            <DataUpdater unitGenerationType={typeGenerationMode!} countGenerationType={countGenerationMode!} countGenerationMode={powerBasedGenerationMode!}/>
        </div> :
        null
    }
    </>
}

function DataDisplayer({unitGenerationType, countGenerationType, countGenerationMode}: {
    unitGenerationType: StackUnitGenerationType,
    countGenerationType: StackCountGenerationType,
    countGenerationMode: AssetGenerationType
}) {

    return <div style={{display: 'flex'}}>
        <Text style={{fontFamily: 'cursive', fontSize: 14, fontWeight: 'bolder', fontStretch: 'expanded'}}>
            <span>
                {` unit generation -`}
            </span>
            <span style={{color: 'green'}}>
                {` ${unitGenerationTypeNames.get(unitGenerationType!)}`}
            </span>
            <span style={{color: 'black'}}>
                <br></br>
                {`count generation -`}
            </span>
            <span style={{color: 'green'}}>
                {` ${countGenerationTypeNames.get(countGenerationType!)}`}
                <br></br>
            </span>
            {
                countGenerationType == StackCountGenerationType.PowerBased ?
                <>
                    <span style={{color: 'black'}}>
                        {`power generation - `}
                    </span>
                    <span style={{color: 'green'}}>
                        {countGenerationMode == AssetGenerationType.Static ? "Static" : "Dynamic"}
                    </span>
                </> :
                null
            }
        </Text>
    </div>
}

export type UpdateStackDataPayload = {
    stackId: number,
    unitGenerationType: StackUnitGenerationType,
    countGenerationType: StackCountGenerationType,
    countGenerationMode: AssetGenerationType
}

function DataUpdater({unitGenerationType, countGenerationType, countGenerationMode}: {
    unitGenerationType: StackUnitGenerationType,
    countGenerationType: StackCountGenerationType,
    countGenerationMode: AssetGenerationType
}) {
    const [opened, {open, close}] = useDisclosure(false);
    const [selectedUnitGenerationType, setSelectedUnitGenerationType] = useState<StackUnitGenerationType | null>(unitGenerationType);
    const [selectedCountGenerationType, setSelectedCountGenerationType] = useState<StackCountGenerationType | null>(countGenerationType);
    const [selectedType, setSelectedType] = useState<AssetGenerationType | null>(countGenerationMode);

    const id = useCurrentStackId();
    const actions = useCurrentStackActions();

    const mutation = useMutation({
        mutationFn: async(payload: UpdateStackDataPayload) => {
            return FightGeneratorApi.updateStackData(payload);
        },
        onSuccess(_data, variables, _context) {
            close();
            actions.updateBaseData(variables)
        },
    });

    return (
    <>
        <Tooltip label="Use to update generation rules">
            <ActionIcon radius={0} onClick={open}>
                <IconEdit/>
            </ActionIcon>
        </Tooltip>
        <ModalRoot opened={opened} onClose={close}>
            <ModalOverlay/>
            <ModalContent>
                <ModalHeader>
                    <ModalTitle>Stack data update</ModalTitle>
                    <ModalCloseButton/>
                </ModalHeader>
                <ModalBody>
                    <Stack>
                        <Select
                            value={selectedUnitGenerationType}
                            onChange={(value) => setSelectedUnitGenerationType(value as StackUnitGenerationType)}
                            placeholder="Generation mode of stack unit type"
                            label="Select mode"
                            data={[
                                {label: "Generate concrete units in stack", value: StackUnitGenerationType.ConcreteUnit},
                                {label: "Generate unit from given towns and tiers", value: StackUnitGenerationType.TierSlotBased}
                            ]}
                        />
                        <Select
                            value={selectedCountGenerationType}
                            onChange={(value) => setSelectedCountGenerationType(value as StackCountGenerationType)}
                            placeholder="Generation mode of stack unit count type"
                            label="Select mode"
                            data={[
                                {label: "Use raw values for unit count", value: StackCountGenerationType.Raw},
                                {label: "Generate counts using unit's power", value: StackCountGenerationType.PowerBased}
                            ]}
                        />
                        <Select
                            // disabled={!selectedCountGenerationType || selectedCountGenerationType == StackCountGenerationType.Raw}
                            value={selectedType} 
                            onChange={(value) => setSelectedType(value as AssetGenerationType)} 
                            placeholder="Define does this stack have dynamic generation or not"
                            label="Define generation"
                            data={[
                                {label: "Static(stack doesn't grow over time)", value: AssetGenerationType.Static},
                                {label: "Dynamic(stack grows over time)", value: AssetGenerationType.Dynamic}
                            ]}
                        />
                    </Stack>
                    <Group justify="end">
                        <Button 
                            disabled={!selectedCountGenerationType || !selectedUnitGenerationType || !selectedType} 
                            onClick={() => mutation.mutate({
                                stackId: id!, 
                                countGenerationType: selectedCountGenerationType!,
                                unitGenerationType: selectedUnitGenerationType!,
                                countGenerationMode: selectedType!
                            })}
                        >Create</Button>
                    </Group>
                </ModalBody>
            </ModalContent>
        </ModalRoot>
    </>
    )
}

export default FightAssetCurrentStackData;
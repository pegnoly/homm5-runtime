import { useDisclosure } from "@mantine/hooks";
import { useState } from "react";
import { StackCountGenerationType, StackUnitGenerationType } from "./types";
import { AssetGenerationType } from "../../types";
import { Button, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, Select, Stack, UnstyledButton } from "@mantine/core";
import { IconSquareRoundedPlus2 } from "@tabler/icons-react";
import { useMutation } from "@tanstack/react-query";
import { FightGeneratorApi } from "../../api";

export type CreateStackPayload = {
    assetId: string,
    typeGenerationMode: StackUnitGenerationType,
    countGenerationMode: StackCountGenerationType,
    generationType: AssetGenerationType
}

function FightAssetStackCreator(params: {
    assetId: string,
    disabled: boolean,
    stackCreatedCallback: (value: number) => void
}) {
    const [opened, {open, close}] = useDisclosure(false);
    const [selectedUnitGenerationType, setSelectedUnitGenerationType] = useState<StackUnitGenerationType | null>(null);
    const [selectedCountGenerationType, setSelectedCountGenerationType] = useState<StackCountGenerationType | null>(null);
    const [selectedType, setSelectedType] = useState<AssetGenerationType | null>(null);

    const mutation = useMutation({
        mutationFn: async(payload: CreateStackPayload) => {
            return FightGeneratorApi.createStack(payload);
        },
        onSuccess(data, _variables, _context) {
            close();
            params.stackCreatedCallback(data);
        },
    })

    return <>
        <UnstyledButton disabled={params.disabled} onClick={open} size="md">
            <IconSquareRoundedPlus2/>
        </UnstyledButton>
        <ModalRoot centered opened={opened} onClose={close}>
            <ModalOverlay/>
            <ModalContent>
                <ModalHeader>
                    <ModalTitle>Fight asset stack creation</ModalTitle>
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
                                assetId: params.assetId, 
                                countGenerationMode: selectedCountGenerationType!,
                                typeGenerationMode: selectedUnitGenerationType!,
                                generationType: selectedType!
                            })}
                        >Create</Button>
                    </Group>
                </ModalBody>
            </ModalContent>
        </ModalRoot>
    </>
}

export default FightAssetStackCreator;
import { useDisclosure } from "@mantine/hooks";
import { useState } from "react";
import { CalculationType, SelectionType } from "./types";
import { AssetGenerationType } from "../../types";
import { ActionIcon, Button, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, Select, Stack, Tooltip } from "@mantine/core";
import { IconSquareRoundedPlus2 } from "@tabler/icons-react";
import { useMutation } from "@tanstack/react-query";
import { FightGeneratorApi } from "../../api";

export type CreateStackPayload = {
    assetId: number,
    selectionType: SelectionType,
    calculationRule: CalculationType,
    updateType: AssetGenerationType
}

function FightAssetStackCreator(params: {
    assetId: number,
    disabled: boolean,
    stackCreatedCallback: (value: number) => void
}) {
    const [opened, {open, close}] = useDisclosure(false);
    const [selectionType, setSelectionType] = useState<SelectionType | null>(null);
    const [calculationType, setCalculationType] = useState<CalculationType | null>(null);
    const [updateType, setUpdateType] = useState<AssetGenerationType | null>(null);

    const mutation = useMutation({
        mutationFn: async(payload: CreateStackPayload) => {
            return FightGeneratorApi.createStack(payload);
        },
        onError(error, _variables, _context) {
            console.log("Error: ", error);
        },
        onSuccess(data, _variables, _context) {
            close();
            params.stackCreatedCallback(data);
        },
    })

    return <>
        <Tooltip label="Используйте, чтобы добавить отряд">
            <ActionIcon disabled={params.disabled} onClick={open} size="md" bg="green">
                <IconSquareRoundedPlus2/>
            </ActionIcon>
        </Tooltip>
        <ModalRoot centered opened={opened} onClose={close}>
            <ModalOverlay/>
            <ModalContent>
                <ModalHeader>
                    <ModalTitle>Создание нового отряда</ModalTitle>
                    <ModalCloseButton/>
                </ModalHeader>
                <ModalBody>
                    <Stack>
                        <Select
                            value={selectionType}
                            onChange={(value) => setSelectionType(value as SelectionType)}
                            placeholder="Тип выбора юнитов в стеке"
                            label="Укажите нужный тип"
                            data={[
                                {label: "Выбор конкретных юнитов вручную", value: SelectionType.Manual},
                                {label: "Генерация случайных из указанных городов и уровней", value: SelectionType.TierSlotBased}
                            ]}
                        />
                        <Select
                            value={calculationType}
                            onChange={(value) => setCalculationType(value as CalculationType)}
                            placeholder="Тип определения численности отряда"
                            label="Укажите нужный тип"
                            data={[
                                {label: "Конкретное число юнитов", value: CalculationType.Raw},
                                {label: "Генерация на основе силы юнитов", value: CalculationType.PowerBased}
                            ]}
                        />
                        <Select
                            // disabled={!selectedCountGenerationType || selectedCountGenerationType == StackCountGenerationType.Raw}
                            value={updateType}
                            onChange={(value) => setUpdateType(value as AssetGenerationType)}
                            placeholder="Тип увеличения силы отряда"
                            label="Укажите нужный тип"
                            data={[
                                {label: "Статический(отряд не растет со временем)", value: AssetGenerationType.Static},
                                {label: "Динамический(отряд растет со временем)", value: AssetGenerationType.Dynamic}
                            ]}
                        />
                    </Stack>
                    <Group justify="end">
                        <Button
                            radius={0}
                            disabled={!selectionType || !calculationType || !updateType}
                            onClick={() => mutation.mutate({
                                assetId: params.assetId, 
                                selectionType: selectionType!,
                                calculationRule: calculationType!,
                                updateType: updateType!
                            })}
                        >Создать</Button>
                    </Group>
                </ModalBody>
            </ModalContent>
        </ModalRoot>
    </>
}

export default FightAssetStackCreator;
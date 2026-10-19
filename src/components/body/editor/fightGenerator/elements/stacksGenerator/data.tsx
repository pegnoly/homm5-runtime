import { useDisclosure } from "@mantine/hooks";
import { AssetGenerationType } from "../../types";
import {
    FightStackGeneratorStore
} from "./store";
import {
    CalculationType,
    SelectionType,
    unicCalculationTypeNames,
    unitSelectionTypeNames
} from "./types";
import { ActionIcon, Button, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, Select, Stack, Text, Tooltip } from "@mantine/core";
import { IconEdit } from "@tabler/icons-react";
import { useState } from "react";
import { useMutation } from "@tanstack/react-query";
import { FightGeneratorApi } from "../../api";

function FightAssetCurrentStackData() {
    const id = FightStackGeneratorStore.useCurrentId();
    const selectionType = FightStackGeneratorStore.useSelectionType();
    const calculationType = FightStackGeneratorStore.useCalculationType();
    const updateType = FightStackGeneratorStore.useUpdateType();

    return <>
    {
        id != undefined ?
        <div style={{display: 'flex', flexDirection: 'row', gap: 10}}>
            <DataDisplayer selection={selectionType!} calculation={calculationType!} update={updateType!}/>
            <DataUpdater selection={selectionType!} calculation={calculationType!} update={updateType!}/>
        </div> :
        null
    }
    </>
}

function DataDisplayer({selection, calculation, update}: {
    selection: SelectionType,
    calculation: CalculationType,
    update: AssetGenerationType
}) {

    return <div style={{display: 'flex'}}>
        <Text style={{fontFamily: 'cursive', fontSize: 14, fontWeight: 'bolder', fontStretch: 'expanded'}}>
            <span>
                {` Тип выбора -`}
            </span>
            <span style={{color: 'green'}}>
                {` ${unitSelectionTypeNames.get(selection!)}`}
            </span>
            <span style={{color: 'black'}}>
                <br></br>
                {`рассчет численности -`}
            </span>
            <span style={{color: 'green'}}>
                {` ${unicCalculationTypeNames.get(calculation!)}`}
                <br></br>
            </span>
            <span style={{color: 'black'}}>
                {`прирост - `}
            </span>
            <span style={{color: 'green'}}>
                {update == AssetGenerationType.Static ? "Нет" : "Да"}
            </span>
        </Text>
    </div>
}

export type UpdateStackDataPayload = {
    stackId: number,
    selectionRule: SelectionType,
    calculationRule: CalculationType,
    updateType: AssetGenerationType
}

function DataUpdater({selection, calculation, update}: {
    selection: SelectionType,
    calculation: CalculationType,
    update: AssetGenerationType
}) {
    const [opened, {open, close}] = useDisclosure(false);
    const [selectionType, setSelectionType] = useState<SelectionType | null>(selection);
    const [calculationType, setCalculationType] = useState<CalculationType | null>(calculation);
    const [updateType, setUpdateType] = useState<AssetGenerationType | null>(update);

    const id = FightStackGeneratorStore.useCurrentId();
    const actions = FightStackGeneratorStore.useActions();

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
        <Tooltip label="Используйте, чтобы обновить правила генерации">
            <ActionIcon radius={0} onClick={open}>
                <IconEdit/>
            </ActionIcon>
        </Tooltip>
        <ModalRoot opened={opened} onClose={close}>
            <ModalOverlay/>
            <ModalContent>
                <ModalHeader>
                    <ModalTitle>Обновление логики генерации стека</ModalTitle>
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
                            disabled={!selectionType || !calculationType || !updateType}
                            onClick={() => mutation.mutate({
                                stackId: id!, 
                                selectionRule: selectionType!,
                                calculationRule: calculationType!,
                                updateType: updateType!
                            })}
                        >Обновить</Button>
                    </Group>
                </ModalBody>
            </ModalContent>
        </ModalRoot>
    </>
    )
}

export default FightAssetCurrentStackData;
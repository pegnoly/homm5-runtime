import { Button, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, Stack, TextInput } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useState } from "react";
import { BankDifficultyType, BankVariant } from "../types";
import { BankMainStore } from "../store";
import { BankDifficultyStore } from "./difficulty/store";
import { useMutation } from "@tanstack/react-query";
import { BankGeneratorApi } from "../api";

function BankVariantCreator({onCreated}: {onCreated: (value: BankVariant) => void}) {
    const bankId = BankMainStore.useId();
    const difficulty = BankDifficultyStore.useType();

    const [opened, {open, close}] = useDisclosure(false);
    const [label, setLabel] = useState<string>("");

    const mutation = useMutation({
        mutationFn: async(data: {bankId: number, label: string, difficulty: BankDifficultyType}) => {
            return BankGeneratorApi.createVariant(data.bankId, data.label, data.difficulty);
        },
        onSuccess(data, _variables, _context) {
            close();
            onCreated(data);
        },
    })

    return (
    <>
        <Button radius={0} onClick={open}>Create variant</Button>
        <ModalRoot opened={opened} centered onClose={close}>
            <ModalOverlay/>
            <ModalContent>
                <ModalHeader>
                    <ModalTitle>Variant creation</ModalTitle>
                    <ModalCloseButton/>
                </ModalHeader>
                <ModalBody>
                    <Stack>
                        <TextInput
                            label="Enter label"
                            value={label}
                            onChange={(e) => setLabel(e.currentTarget.value)}
                        />
                        <Group justify="end">
                            <Button
                                disabled={label.length == 0} 
                                radius={0}
                                onClick={() => mutation.mutate({bankId: bankId!, label: label, difficulty: difficulty!})}
                            >Create</Button>
                        </Group>
                    </Stack>
                </ModalBody>
            </ModalContent>
        </ModalRoot>
    </>
    )
}

export default BankVariantCreator;
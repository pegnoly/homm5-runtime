import { Button, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, NumberInput, Stack, TextInput } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useState } from "react";
import CreatureCopyCreator from "./store";

function CreatureGenerationInitializator() {
    const actions = CreatureCopyCreator.useActions();

    const [opened, {open, close}] = useDisclosure(false);
    const [id, setId] = useState<number | undefined>(undefined);
    const [name, setName] = useState<string | undefined>(undefined);

    async function create() {
        actions.initializeSession(id!, name!);
        actions.addModel(id!);
        setId(undefined);
        setName(undefined);
        close();
    }

    return (
    <>
        <Button radius={0} onClick={open}>
            Init session
        </Button>
        <ModalRoot opened={opened} onClose={close} centered>
            <ModalOverlay/>
            <ModalContent>
                <ModalHeader>
                    <ModalTitle>Initialize generation</ModalTitle>
                    <ModalCloseButton/>
                </ModalHeader>
                <ModalBody>
                    <Stack>
                        <TextInput
                            label="Enter the name of session"
                            value={name}
                            onChange={(e) => setName(e.currentTarget.value)}
                        />
                        <NumberInput
                            label="Enter creature id to start from"
                            value={id}
                            onChange={(value) => setId(value as number)}
                        />
                        <Group justify="end">
                            <Button 
                                radius={0} 
                                type="submit"
                                disabled={id == undefined || name == undefined}
                                onClick={create}
                            >Create</Button>
                        </Group>
                    </Stack>
                </ModalBody>
            </ModalContent>
        </ModalRoot>
    </>
    )
}

export default CreatureGenerationInitializator;
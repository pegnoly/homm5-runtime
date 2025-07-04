import { Button, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, NumberInput, Stack } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useState } from "react";
import CreatureCopyCreator from "./store";

function CreatureGenerationInitializator() {
    const actions = CreatureCopyCreator.useActions();

    const [opened, {open, close}] = useDisclosure(false);
    const [id, setId] = useState<number | undefined>(undefined);

    async function create() {
        actions.initializeSession(id!);
        actions.addModel(id!);
        setId(undefined)
        close()
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
                        <NumberInput
                            label="Enter creature id to start from"
                            value={id}
                            onChange={(value) => setId(value as number)}
                        />
                        <Group justify="end">
                            <Button 
                                radius={0} 
                                type="submit"
                                disabled={id == undefined}
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
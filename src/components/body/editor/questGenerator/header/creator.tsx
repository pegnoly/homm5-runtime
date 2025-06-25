import { 
    Button, 
    Group, 
    ModalBody, 
    ModalCloseButton, 
    ModalContent, 
    ModalHeader, 
    ModalOverlay, 
    ModalRoot, 
    ModalTitle, 
    Stack, 
    Text, 
    TextInput, 
    Tooltip 
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks"
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useState } from "react";
import { useQuests, useQuestsActions } from "../store";
import { useMutation } from "@tanstack/react-query";
import { QuestGeneratorApi } from "../api";

export type CreateQuestPayload = {
    directory: string,
    name: string,
    scriptName: string
}

function QuestCreator() {
    const [opened, {open, close}] = useDisclosure(false);

    const actions = useQuestsActions();
    const quests = useQuests();

    const [name, setName] = useState<string | undefined>(undefined);
    const [scriptName, setScriptName] = useState<string | undefined>(undefined);
    const [directory, setDirectory] = useState<string | undefined>(undefined);

    async function pickDirectory() {
        invoke("pick_quest_directory", {initial: true})
    }

    listen<string>("quest_directory_picked", (event) => {
        setDirectory(event.payload)
    })

    const mutation = useMutation({
        mutationFn: async(payload: CreateQuestPayload) => {
            return QuestGeneratorApi.createQuest(payload);
        },
        onSuccess(data, _variables, _context) {
            close();
            actions.loadQuests([...quests!, data]);
            actions.loadCurrentQuest(data);
            actions.setCurrentProgressNumber(0);
        },
    })

    return (
    <>
        <Button radius={0} onClick={open}>Create new quest</Button>
        <ModalRoot opened={opened} onClose={close} centered>
            <ModalOverlay/>
            <ModalContent>
                <ModalHeader>
                    <ModalTitle>Quest creation</ModalTitle>
                    <ModalCloseButton/>
                </ModalHeader>
                <ModalBody>
                    <Stack>
                        <TextInput
                            label="Quest name"
                            placeholder="Enter name"
                            value={name}
                            onChange={(e) => setName(e.currentTarget.value)}
                        />
                        <TextInput
                            label="Quest script name"
                            placeholder="Enter name"
                            value={scriptName}
                            onChange={(e) => setScriptName(e.currentTarget.value)}
                        />
                        <Button radius={0} onClick={pickDirectory}>Pick directory of quest</Button>
                        <Tooltip label={directory}>
                            <Text lineClamp={1}>{directory}</Text>
                        </Tooltip>
                        <Group justify="end">
                            <Button 
                                onClick={() => mutation.mutate({name: name!, scriptName: scriptName!, directory: directory!})} 
                                radius={0} 
                                type="submit"
                            >Create</Button>
                        </Group>
                    </Stack>
                </ModalBody>
            </ModalContent>
        </ModalRoot>
    </>
    )
}

export default QuestCreator;
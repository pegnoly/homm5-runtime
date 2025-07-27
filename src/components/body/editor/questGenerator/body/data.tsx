import { ActionIcon, Button, Checkbox, Group, Stack, Text, Textarea, Tooltip } from "@mantine/core"
import EditableProperty from "../../../../common/editableProperty"
import { useCurrentQuestDesc, useCurrentQuestDirectory, useCurrentQuestId, useCurrentQuestIsActive, useCurrentQuestIsSecondary, useCurrentQuestName, useCurrentQuestScriptName, useQuestsActions } from "../store"
import { useMutation } from "@tanstack/react-query"
import { QuestGeneratorApi } from "../api"
import { IconEdit } from "@tabler/icons-react"
import { invoke } from "@tauri-apps/api/core"
import { listen } from "@tauri-apps/api/event"
import { useState } from "react"

export type UpdateQuestPayloadBase = {
    questId: number
}

function QuestGeneratorDataBlock() {
    const currentId = useCurrentQuestId();
    const currentName = useCurrentQuestName();
    const currentScriptName = useCurrentQuestScriptName();
    const currentDirectory = useCurrentQuestDirectory();
    const currentDesc = useCurrentQuestDesc();
    const isActive = useCurrentQuestIsActive();
    const isSecondary = useCurrentQuestIsSecondary();
    const actions = useQuestsActions();

    console.log("Current desc:", currentDesc);

    const [localDesc, setLocalDesc] = useState<string>(currentDesc!);
    const [descEditable, setDescEditable] = useState<boolean>(false);

    console.log("Local desc:", localDesc);

    async function tryUpdateDirectory() {
        await invoke("pick_quest_directory", {initial: false})
    }

    listen<string>("quest_directory_updated", async(event) => {
        await QuestGeneratorApi.updateQuestDirectory({directory: event.payload, questId: currentId!});
        actions.setCurrentQuestDirectory(event.payload);
    })

    const updateNameMutation = useMutation({
        mutationFn: async(payload: UpdateQuestPayloadBase & {name: string}) => {
            return QuestGeneratorApi.updateQuestName(payload);
        },
        onSuccess(_data, variables, _context) {
            actions.setCurrentQuestName(variables.name);
        },
    });

    const updateScriptNameMutation = useMutation({
        mutationFn: async(payload: UpdateQuestPayloadBase & {scriptName: string}) => {
            return QuestGeneratorApi.updateQuestScriptName(payload);
        },
        onSuccess(_data, variables, _context) {
            actions.setCurrentQuestScriptName(variables.scriptName);
        },
    });

    const updateDescMutation = useMutation({
        mutationFn: async(payload: UpdateQuestPayloadBase & {desc: string}) => {
            return QuestGeneratorApi.updateQuestDesc(payload);
        },
        onSuccess(_data, variables, _context) {
            actions.setCurrentQuestDesc(variables.desc)
        },
    });

    const updateIsActiveMutation = useMutation({
        mutationFn: async(payload: UpdateQuestPayloadBase & {isActive: boolean}) => {
            return QuestGeneratorApi.updateQuestIsActive(payload);
        },
        onSuccess(_data, variables, _context) {
            actions.setCurrentQuestIsActive(variables.isActive)
        },
    });

    const updateIsSecondaryMutation = useMutation({
        mutationFn: async(payload: UpdateQuestPayloadBase & {isSecondary: boolean}) => {
            return QuestGeneratorApi.updateQuestIsSecondary(payload);
        },
        onSuccess(_data, variables, _context) {
            actions.setCurrentQuestIsSecondary(variables.isSecondary)
        },
    });

    return (
    <div 
        style={{
            pointerEvents: currentId == undefined ? 'none' : 'auto',
            width: '100%',
            height: '100%',
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            paddingLeft: '5%',
            paddingRight: '5%'
        }}
    >
        <Stack>
            <EditableProperty 
                label="Quest name" 
                initialValue={currentName!} 
                onSave={(value) => updateNameMutation.mutate({questId: currentId!, name: value})}
            />
            <EditableProperty 
                label="Quest script name" 
                initialValue={currentScriptName!} 
                onSave={(value) => updateScriptNameMutation.mutate({questId: currentId!, scriptName: value})}
            />
            <Group>
                <Tooltip label={currentDirectory}>
                    <Text style={{width: '80%'}} lineClamp={1}>{currentDirectory}</Text>
                </Tooltip>
                <ActionIcon onClick={tryUpdateDirectory}>
                    <IconEdit/>
                </ActionIcon>
            </Group>
            <Textarea
                rows={10}
                disabled={!descEditable}
                label="Quest description"
                value={localDesc}
                onChange={(e) => setLocalDesc(e.currentTarget.value)}
            />
            <Group justify="end">
                <div hidden={descEditable}>
                    <Button onClick={() => setDescEditable(true)} radius={0} size="xs">Edit desc</Button>
                </div>
                <div hidden={!descEditable}>
                    <Button 
                        radius={0} 
                        size="xs"
                        onClick={() => {
                            setDescEditable(false);
                            updateDescMutation.mutate({desc: localDesc!, questId: currentId!});
                        }}
                    >Save desc</Button>
                </div>
            </Group>
            <Group justify="space-between">
                <Checkbox 
                    label="Is active quest" 
                    checked={isActive}
                    onChange={(e) => updateIsActiveMutation.mutate({questId: currentId!, isActive: e.currentTarget.checked})}
                />
                <Checkbox 
                    label="Is secondary quest"
                    checked={isSecondary}
                    onChange={(e) => updateIsSecondaryMutation.mutate({questId: currentId!, isSecondary: e.currentTarget.checked})}
                />
            </Group>
        </Stack>
    </div>
    )
}

export default QuestGeneratorDataBlock;
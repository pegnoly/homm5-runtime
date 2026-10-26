import {
    ActionIcon,
    Badge,
    Button,
    Card,
    Group,
    Select,
    Stack,
    Text,
    TextInput,
    Tooltip
} from "@mantine/core";
import { IconEdit, IconPlus, IconTrash, IconX } from "@tabler/icons-react";
import { useMutation } from "@tanstack/react-query";
import { useEffect, useState } from "react";
import { QuestGeneratorApi } from "../api";
import { QuestGeneratorStore } from "../store";
import { QuestUpdateSource, QuestUpdateSourceVariable, QuestUpdateSourceVariableType } from "../types";

export type AddQuestUpdateSourcePayload = {
    questId: number,
    source: string,
    variables: QuestUpdateSourceVariable[],
    comment: string
}

export type UpdateQuestUpdateSourcePayload = {
    questId: number,
    source: string,
    variables: QuestUpdateSourceVariable[],
    comment: string
}

export type DeleteQuestUpdateSourcePayload = {
    questId: number,
    source: string
}

const VARIABLE_TYPE_LABELS: Record<QuestUpdateSourceVariableType, string> = {
    string: "Строка",
    number: "Число",
    bool: "Логическое",
    any: "Любое"
}

const VARIABLE_TYPE_OPTIONS = [
    { value: "string", label: VARIABLE_TYPE_LABELS.string },
    { value: "number", label: VARIABLE_TYPE_LABELS.number },
    { value: "bool", label: VARIABLE_TYPE_LABELS.bool },
    { value: "any", label: VARIABLE_TYPE_LABELS.any }
]

function QuestGeneratorUpdateSourcesBlock() {
    const currentId = QuestGeneratorStore.useQuestId();

    return (
    <div
        style={{
            pointerEvents: currentId == undefined ? 'none' : 'auto',
            width: '100%',
            height: '100%',
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'stretch',
            paddingLeft: '5%',
            paddingRight: '5%',
            overflowY: 'auto'
        }}
    >
        {
            currentId == undefined ? null :
            <UpdateSourcesEditor questId={currentId}/>
        }
    </div>
    )
}

function UpdateSourcesEditor({questId}: {questId: number}) {
    const updateSources = QuestGeneratorStore.useQuestUpdateSources();
    const sources = updateSources?.sources ?? [];

    return (
    <Stack gap="sm">
        <AddUpdateSourceForm questId={questId} sources={sources}/>
        <Text fw={700}>Источники обновления</Text>
        {
            sources.length == 0 ?
            <Text c="dimmed" size="sm">Источники не добавлены</Text> :
            sources.map(item => (
                <UpdateSourceItem key={item.source} questId={questId} item={item}/>
            ))
        }
    </Stack>
    )
}

function AddUpdateSourceForm({questId, sources}: {questId: number, sources: QuestUpdateSource[]}) {
    const actions = QuestGeneratorStore.useActions();

    const [source, setSource] = useState<string>("");
    const [variables, setVariables] = useState<QuestUpdateSourceVariable[]>([]);
    const [comment, setComment] = useState<string>("");

    const addMutation = useMutation({
        mutationFn: async(payload: AddQuestUpdateSourcePayload) => {
            return QuestGeneratorApi.addUpdateSource(payload);
        },
        onSuccess(_data, payload, _context) {
            actions.addCurrentQuestUpdateSource({
                source: payload.source,
                variables: payload.variables,
                comment: payload.comment
            });
            setSource("");
            setVariables([]);
            setComment("");
        },
    });

    const isEmpty = source.trim().length == 0;
    const isDuplicate = sources.some(item => item.source == source);

    return (
    <Card withBorder radius={0} padding="sm">
        <Stack gap="xs">
            <Text fw={700}>Добавить источник обновления</Text>
            <TextInput
                label="Источник"
                placeholder="Название источника"
                value={source}
                onChange={(e) => setSource(e.currentTarget.value)}
                error={isDuplicate ? "Источник с таким названием уже добавлен" : undefined}
            />
            <UpdateSourceVariablesEditor value={variables} onChange={setVariables}/>
            <TextInput
                label="Комментарий"
                placeholder="Комментарий к источнику"
                value={comment}
                onChange={(e) => setComment(e.currentTarget.value)}
            />
            <Group justify="end">
                <Button
                    radius={0}
                    size="xs"
                    leftSection={<IconPlus size={16}/>}
                    disabled={isEmpty || isDuplicate}
                    onClick={() => addMutation.mutate({
                        questId,
                        source,
                        variables,
                        comment
                    })}
                >Добавить</Button>
            </Group>
        </Stack>
    </Card>
    )
}

function UpdateSourceItem({questId, item}: {questId: number, item: QuestUpdateSource}) {
    const actions = QuestGeneratorStore.useActions();

    const [editing, setEditing] = useState<boolean>(false);
    const [variables, setVariables] = useState<QuestUpdateSourceVariable[]>(item.variables);
    const [comment, setComment] = useState<string>(item.comment);

    useEffect(() => {
        setVariables(item.variables);
        setComment(item.comment);
    }, [item.variables, item.comment]);

    const updateMutation = useMutation({
        mutationFn: async(payload: UpdateQuestUpdateSourcePayload) => {
            return QuestGeneratorApi.updateUpdateSource(payload);
        },
        onSuccess(_data, payload, _context) {
            actions.updateCurrentQuestUpdateSource(payload.source, {
                source: payload.source,
                variables: payload.variables,
                comment: payload.comment
            });
            setEditing(false);
        },
    });

    const deleteMutation = useMutation({
        mutationFn: async(payload: DeleteQuestUpdateSourcePayload) => {
            return QuestGeneratorApi.deleteUpdateSource(payload);
        },
        onSuccess(_data, payload, _context) {
            actions.deleteCurrentQuestUpdateSource(payload.source);
        },
    });

    return (
    <Card withBorder radius={0} padding="sm">
        {
            editing ?
            <Stack gap="xs">
                <TextInput label="Источник" value={item.source} disabled/>
                <UpdateSourceVariablesEditor value={variables} onChange={setVariables}/>
                <TextInput
                    label="Комментарий"
                    value={comment}
                    onChange={(e) => setComment(e.currentTarget.value)}
                />
                <Group justify="end">
                    <Button
                        radius={0}
                        size="xs"
                        onClick={() => updateMutation.mutate({
                            questId,
                            source: item.source,
                            variables,
                            comment
                        })}
                    >Сохранить</Button>
                    <Button
                        radius={0}
                        size="xs"
                        variant="default"
                        onClick={() => {
                            setVariables(item.variables);
                            setComment(item.comment);
                            setEditing(false);
                        }}
                    >Отмена</Button>
                </Group>
            </Stack> :
            <Stack gap="xs">
                <Group justify="space-between" wrap="nowrap">
                    <Text fw={600} lineClamp={1}>{item.source}</Text>
                    <Group gap={4} wrap="nowrap">
                        <Tooltip label="Редактировать источник">
                            <ActionIcon onClick={() => setEditing(true)}>
                                <IconEdit size={16}/>
                            </ActionIcon>
                        </Tooltip>
                        <Tooltip label="Удалить источник">
                            <ActionIcon
                                color="red"
                                onClick={() => deleteMutation.mutate({questId, source: item.source})}
                            >
                                <IconTrash size={16}/>
                            </ActionIcon>
                        </Tooltip>
                    </Group>
                </Group>
                {
                    item.variables.length == 0 ? null :
                    <Group gap={4}>
                        {
                            item.variables.map((variable, index) => (
                                <Badge key={index} radius={0} size="sm">
                                    {`${variable.name}: ${VARIABLE_TYPE_LABELS[variable.type]}`}
                                </Badge>
                            ))
                        }
                    </Group>
                }
                {
                    item.comment.length == 0 ? null :
                    <Text size="sm" c="dimmed">{item.comment}</Text>
                }
            </Stack>
        }
    </Card>
    )
}

function UpdateSourceVariablesEditor(
    {value, onChange}: {
        value: QuestUpdateSourceVariable[],
        onChange: (value: QuestUpdateSourceVariable[]) => void
    }
) {
    const [name, setName] = useState<string>("");
    const [variableType, setVariableType] = useState<string | null>(null);

    const trimmedName = name.trim();
    const isEmpty = trimmedName.length == 0;
    const isDuplicate = value.some(item => item.name == trimmedName);

    function addVariable() {
        if (isEmpty || isDuplicate) {
            return;
        }

        onChange([
            ...value,
            {
                name: trimmedName,
                type: (variableType ?? "any") as QuestUpdateSourceVariableType
            }
        ]);
        setName("");
        setVariableType(null);
    }

    return (
    <Stack gap={4}>
        <Text size="sm" fw={500}>Переменные</Text>
        <Group gap="xs" align="end" wrap="nowrap">
            <TextInput
                style={{flex: 1}}
                size="xs"
                placeholder="Имя переменной"
                value={name}
                error={isDuplicate ? "Переменная уже добавлена" : undefined}
                onChange={(e) => setName(e.currentTarget.value)}
                onKeyDown={(e) => {
                    if (e.key == "Enter") {
                        addVariable();
                    }
                }}
            />
            <Select
                size="xs"
                w={140}
                placeholder="Любое"
                data={VARIABLE_TYPE_OPTIONS}
                value={variableType}
                onChange={setVariableType}
                clearable
            />
            <ActionIcon
                size="lg"
                disabled={isEmpty || isDuplicate}
                onClick={addVariable}
            >
                <IconPlus size={16}/>
            </ActionIcon>
        </Group>
        {
            value.length == 0 ? null :
            <Group gap={4}>
                {
                    value.map((variable, index) => (
                        <Badge
                            key={index}
                            radius={0}
                            size="sm"
                            rightSection={
                                <ActionIcon
                                    size="xs"
                                    variant="transparent"
                                    color="gray"
                                    onClick={() => onChange(value.filter((_, i) => i != index))}
                                >
                                    <IconX size={12}/>
                                </ActionIcon>
                            }
                        >
                            {`${variable.name}: ${VARIABLE_TYPE_LABELS[variable.type]}`}
                        </Badge>
                    ))
                }
            </Group>
        }
    </Stack>
    )
}

export default QuestGeneratorUpdateSourcesBlock;

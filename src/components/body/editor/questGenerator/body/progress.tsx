import { useMutation, useQuery } from "@tanstack/react-query"
import {
    QuestGeneratorStore
} from "../store"
import { QuestGeneratorApi } from "../api";
import {
    Button,
    ButtonGroup,
    Checkbox,
    Group, NumberInput,
    SegmentedControl,
    Stack,
    Text,
    Textarea
} from "@mantine/core";
import {QuestProgressType} from "@/components/body/editor/questGenerator/types.ts";
import {useRef, useState} from "react";

export type SaveQuestProgressPayload = {
    id: number,
    progressType: QuestProgressType,
    concatenate: boolean
}

export type CreateQuestProgressPayload = {
    questId: number,
    number: number,
    progressType: QuestProgressType
}

enum ProgressType {
    Default = "DEFAULT",
    OneOf = "ONEOF"
}

function QuestGeneratorProgressBlock() {
    const currentQuestId = QuestGeneratorStore.useQuestId();
    return (
    <>
        <CurrentProgress/>
        {
            currentQuestId == undefined ?
            null :
            <ProgressLoader/>
        }
    </>
    )
}

function CurrentProgress() {
    const currentQuestId = QuestGeneratorStore.useQuestId();
    const currentProgressId = QuestGeneratorStore.useProgressId();
    const currentProgressText = QuestGeneratorStore.useProgressText();
    const currentProgressOneOf = QuestGeneratorStore.useProgressOneOf();
    const currentProgressConcatenate = QuestGeneratorStore.useProgressIsConcatenate();
    const currentProgressNumber = QuestGeneratorStore.useProgressNumber();
    const actions = QuestGeneratorStore.useActions();

    const textAreaRef = useRef<HTMLTextAreaElement>(null);

    const [_, setCaretPosition] = useState<number>(0);
    const [currentProgressType, setCurrentProgressType] = useState<ProgressType | undefined>(
        currentProgressText != undefined ? ProgressType.Default : currentProgressOneOf != undefined ? ProgressType.OneOf : undefined
    );

    const saveProgressMutation = useMutation({
        mutationFn: async() => {
            return QuestGeneratorApi.saveProgress({
                id: currentProgressId!,
                progressType: currentProgressType == ProgressType.Default ? { type: "Default", data: currentProgressText! } : { type: "OneOf", data: currentProgressOneOf! },
                concatenate: currentProgressConcatenate!
            });
        },

        onError: (error, _variables, _context) => {
            console.log("Failed to save progress: ", error);
        }
    })

    const createProgressMutation = useMutation({
        mutationFn: async(payload: CreateQuestProgressPayload) => {
            return QuestGeneratorApi.createProgress(payload);
        },

        onSuccess: (data) => {
            actions.loadCurrentProgress(data)
        },

        onError: (error, _variables, _context) => {
            console.log("Creation failed with an error: ", error);
        }
    })

    const handleCursorUpdate = () => {
        if (textAreaRef.current) {
            setCaretPosition(textAreaRef.current.selectionStart)
        }
    }

    const insertTextAtCaret = (text: string) => {
        if (!textAreaRef.current || currentProgressOneOf == undefined) {
            return;
        }

        const start = textAreaRef.current.selectionStart;
        const end = textAreaRef.current.selectionEnd;
        const newText = currentProgressOneOf.text.substring(0, start) + text + currentProgressOneOf.text.substring(end);
        actions.setCurrentProgressOneOf({...currentProgressOneOf!, text: newText});
    }


    return (
    <>
        {
            currentProgressId == undefined ?
                <div style={{display: "flex", alignContent: "center", justifyContent: "center"}}>
                    <Button
                        size="xl" radius={0}
                        onClick={() => createProgressMutation.mutate({ questId: currentQuestId!, number: 0, progressType: { type: "Default", data: "" }})}
                    >Создать нулевой прогресс</Button>
                </div> :
                <div
                    style={{
                        width: '100%',
                        height: '85%',
                        display: 'flex',
                        flexDirection: 'column',
                        alignItems: 'stretch',
                        justifyContent: 'stretch'
                    }}>
                    <div style={{paddingLeft: '10%', paddingRight: '10%'}}>
                        <Stack>
                            <Group>
                                <Text>{`Текущий прогресс: ${currentProgressNumber}`}</Text>
                                <ButtonGroup>
                                    <Button
                                        size="xs"
                                        disabled={currentProgressNumber == 0}
                                        radius={0}
                                        bg="cyan"
                                        onClick={() => actions.setCurrentProgressNumber(currentProgressNumber! - 1)}
                                    >Предыдущий</Button>
                                    <Button
                                        size="xs"
                                        radius={0}
                                        bg="grape"
                                        onClick={() => actions.setCurrentProgressNumber(currentProgressNumber! + 1)}
                                    >Следующий</Button>
                                </ButtonGroup>
                                <SegmentedControl
                                    data={[
                                        { value: ProgressType.Default, label: "Стандартный"},
                                        { value: ProgressType.OneOf, label: "x из N"}
                                    ]}
                                    onChange={(e) => setCurrentProgressType(e as ProgressType)}
                                    value={currentProgressType?.toString()}>
                                </SegmentedControl>
                                {
                                    currentProgressType != ProgressType.OneOf ? null : <OneOfProgressRenderer/>
                                }
                            </Group>
                            <Checkbox
                                label="Отображать после выполнения?"
                                labelPosition="left"
                                checked={currentProgressConcatenate}
                                onChange={(e) => actions.setCurrentProgressConcatenate(e.currentTarget.checked)}
                            />
                            <ButtonGroup>
                                <Button
                                    radius={0}
                                    size="xs"
                                    disabled={currentProgressType == ProgressType.Default}
                                    onClick={() => insertTextAtCaret("[current_value]")}
                                >
                                    Добавить метку текущего "x из N"
                                </Button>
                                <Button
                                    bg="cyan"
                                    radius={0}
                                    size="xs"
                                    disabled={currentProgressType == ProgressType.Default}
                                    onClick={() => insertTextAtCaret("[values_count]")}
                                >
                                    Добавить метку максимального "x из N"
                                </Button>
                            </ButtonGroup>
                            <Textarea
                                ref={textAreaRef}
                                rows={10}
                                minRows={10}
                                maxRows={12}
                                value={currentProgressType == ProgressType.Default ? (currentProgressText != undefined ? currentProgressText : "") : currentProgressOneOf?.text}
                                onChange={(e) => {
                                    currentProgressType == ProgressType.Default ? actions.setCurrentProgressText(e.currentTarget.value)
                                        : actions.setCurrentProgressOneOf(({...currentProgressOneOf!, text: e.currentTarget.value}));
                                }}
                                onMouseUp={handleCursorUpdate}
                                onKeyUp={handleCursorUpdate}
                            />
                            <Button
                                radius={0}
                                onClick={() => saveProgressMutation.mutate()}
                            >Сохранить прогресс</Button>
                        </Stack>
                    </div>
                </div>
        }
    </>)
}

function useProgress(questId: number, progressNumber: number) {
    return useQuery({
        queryKey: ["quest_progress", questId, progressNumber],
        queryFn: async() => {
            return QuestGeneratorApi.loadProgress(questId, progressNumber);
        }
    })
}

function ProgressLoader() {
    const currentQuestId = QuestGeneratorStore.useQuestId();
    const currentProgressNumber = QuestGeneratorStore.useProgressNumber();
    const actions = QuestGeneratorStore.useActions();

    const { data } = useProgress(currentQuestId!, currentProgressNumber!);
    if (data != undefined) {
        actions.loadCurrentProgress(data);
    } else {
        actions.resetCurrentProgress();
    }

    return null;
}

function OneOfProgressRenderer() {
    const currentProgressOneOf = QuestGeneratorStore.useProgressOneOf();
    const actions = QuestGeneratorStore.useActions();

    return (
    <>
        <Group>
            <NumberInput
                size="xs"
                radius={0}
                label="Начальное значение"
                value={currentProgressOneOf!.start_value}
                onChange={(e) => actions.setCurrentProgressOneOf({...currentProgressOneOf!, start_value: e as number})}
            />
            <NumberInput
                size="xs"
                radius={0}
                label="Всего"
                value={currentProgressOneOf!.count}
                onChange={(e) => actions.setCurrentProgressOneOf({...currentProgressOneOf!, count: e as number})}
            />
        </Group>
    </>
    );
}

export default QuestGeneratorProgressBlock;
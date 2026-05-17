import { useMutation, useQuery } from "@tanstack/react-query"
import {
    useCurrentProgressId, useCurrentProgressIsConcatenate, useCurrentProgressNumber,
    useCurrentProgressOneOf, useCurrentProgressText, useCurrentQuestId, useQuestsActions
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
import {useState} from "react";

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
    const currentQuestId = useCurrentQuestId();
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
    const currentQuestId = useCurrentQuestId();
    const currentProgressId = useCurrentProgressId();
    const currentProgressText = useCurrentProgressText();
    const currentProgressOneOf = useCurrentProgressOneOf();
    const currentProgressConcatenate = useCurrentProgressIsConcatenate();
    const currentProgressNumber = useCurrentProgressNumber();
    const actions = useQuestsActions();

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

    return (
    <>
        {
            currentProgressId == undefined ?
                <div style={{display: "flex", alignContent: "center", justifyContent: "center"}}>
                    <Button
                        size="xl" radius={0}
                        onClick={() => createProgressMutation.mutate({ questId: currentQuestId!, number: 0, progressType: { type: "Default", data: "" }})}
                    >Create initial progress</Button>
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
                                <Text>{`Current progress: ${currentProgressNumber}`}</Text>
                                <ButtonGroup>
                                    <Button
                                        size="xs"
                                        disabled={currentProgressNumber == 0}
                                        radius={0}
                                        bg="cyan"
                                        onClick={() => actions.setCurrentProgressNumber(currentProgressNumber! - 1)}
                                    >Previous progress</Button>
                                    <Button
                                        size="xs"
                                        radius={0}
                                        bg="grape"
                                        onClick={() => actions.setCurrentProgressNumber(currentProgressNumber! + 1)}
                                    >Next progress</Button>
                                </ButtonGroup>
                                <SegmentedControl
                                    data={[
                                        { value: ProgressType.Default, label: "Default"},
                                        { value: ProgressType.OneOf, label: "OneOf"}
                                    ]}
                                    onChange={(e) => setCurrentProgressType(e as ProgressType)}
                                    value={currentProgressType?.toString()}>
                                </SegmentedControl>
                                {
                                    currentProgressType != ProgressType.OneOf ? null : <OneOfProgressRenderer/>
                                }
                            </Group>
                            <Checkbox
                                label="Concatenate with previous?"
                                labelPosition="left"
                                checked={currentProgressConcatenate}
                                onChange={(e) => actions.setCurrentProgressConcatenate(e.currentTarget.checked)}
                            />
                            <Textarea
                                rows={10}
                                minRows={10}
                                maxRows={12}
                                value={currentProgressType == ProgressType.Default ? (currentProgressText != undefined ? currentProgressText : "") : currentProgressOneOf?.text}
                                onChange={(e) => {
                                    currentProgressType == ProgressType.Default ? actions.setCurrentProgressText(e.currentTarget.value)
                                        : actions.setCurrentProgressOneOf(({...currentProgressOneOf!, text: e.currentTarget.value}));
                                }}
                            />
                            <Button
                                radius={0}
                                onClick={() => saveProgressMutation.mutate()}
                            >Save progress</Button>
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
    const currentQuestId = useCurrentQuestId();
    const currentProgressNumber = useCurrentProgressNumber();
    const actions = useQuestsActions();

    const { data } = useProgress(currentQuestId!, currentProgressNumber!);
    if (data != undefined) {
        actions.loadCurrentProgress(data);
    } else {
        actions.resetCurrentProgress();
    }

    return null;
}

function OneOfProgressRenderer() {
    const currentProgressOneOf = useCurrentProgressOneOf();
    const actions = useQuestsActions();

    return (
    <>
        <Group>
            <NumberInput
                size="xs"
                radius={0}
                label="Start value"
                value={currentProgressOneOf!.start_value}
                onChange={(e) => actions.setCurrentProgressOneOf({...currentProgressOneOf!, start_value: e as number})}
            />
            <NumberInput
                size="xs"
                radius={0}
                label="Count"
                value={currentProgressOneOf!.count}
                onChange={(e) => actions.setCurrentProgressOneOf({...currentProgressOneOf!, count: e as number})}
            />
        </Group>
    </>
    );
}

export default QuestGeneratorProgressBlock;
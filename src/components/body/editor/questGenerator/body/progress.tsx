import { useMutation, useQuery } from "@tanstack/react-query"
import { useCurrentProgressId, useCurrentProgressIsConcatenate, useCurrentProgressNumber, useCurrentProgressText, useCurrentQuestId, useQuestsActions } from "../store"
import { QuestGeneratorApi } from "../api";
import { Button, ButtonGroup, Checkbox, Group, Stack, Text, Textarea } from "@mantine/core";

export type SaveQuestProgressPayload = {
    id: number,
    text: string,
    concatenate: boolean
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
    const currentProgressConcatenate = useCurrentProgressIsConcatenate();
    const currentProgressNumber = useCurrentProgressNumber();
    const actions = useQuestsActions();

    const mutation = useMutation({
        mutationFn: async(payload: SaveQuestProgressPayload) => {
            return QuestGeneratorApi.saveProgress(payload);
        }
    })

    return (
    <>
        <div 
            style={{
                pointerEvents: currentQuestId == undefined ? 'none' : 'auto', 
                width: '100%', 
                height: '85%', 
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'stretch',
                justifyContent: 'stretch'
            }}
        >
            <div style={{paddingTop: '3%', paddingLeft: '10%', paddingRight: '10%'}}>
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
                    </Group>
                    <Checkbox 
                        label="Concatenate with previous?" 
                        labelPosition="left" 
                        checked={currentProgressConcatenate}
                        onChange={(e) => actions.setCurrentProgressConcatenate(e.currentTarget.checked)}
                    />
                    <Textarea
                        rows={17}
                        minRows={17}
                        maxRows={19}
                        value={currentProgressText}
                        onChange={(e) => actions.setCurrentProgressText(e.currentTarget.value)}
                    />
                    <Button 
                        radius={0} 
                        onClick={() => mutation.mutate({id: currentProgressId!, text: currentProgressText!, concatenate: currentProgressConcatenate!})}
                    >Save progress</Button>
                </Stack>
            </div>
        </div>
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
    } 

    return null;
}

export default QuestGeneratorProgressBlock;
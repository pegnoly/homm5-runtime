import { useQuery } from "@tanstack/react-query";
import { useCurrentMapId } from "@/stores/common.ts";
import { DialogGeneratorApi } from "../api";
import { DialogGeneratorStore } from "../store";
import { Select } from "@mantine/core";

function useDialogsQuery(missionId: number) {
    return useQuery({
        queryKey: ["dialogs", missionId],
        queryFn: async() => {
            return DialogGeneratorApi.loadDialogs(missionId);
        }
    })
}

function DialogsSelector(params: {
    selectedCallback: (value: number) => void
}) {
    const dialogs = DialogGeneratorStore.useDialogs();
    const currentDialogId = DialogGeneratorStore.useCurrentDialogId();

    return (
    <>
        <DialogsLoader/>
        <Select
            label="Загрузить существующий диалог"
            placeholder="Выбрать"
            searchable
            radius={0}
            value={currentDialogId?.toString()}
            onChange={(value) => params.selectedCallback(parseInt(value!))}
            size="sm"
            data={dialogs?.map(dialog => ({
                value: dialog.id.toString(), label: dialog.name
            }))}
        />
    </>
    )
}

function DialogsLoader() {
    const currentMapId = useCurrentMapId();
    const actions = DialogGeneratorStore.useActions();
    const { data } = useDialogsQuery(currentMapId!);
    
    if (data != undefined) {
        actions.loadDialogs(data);
    } else {
        return null;
    }

    return null;
}

export default DialogsSelector;
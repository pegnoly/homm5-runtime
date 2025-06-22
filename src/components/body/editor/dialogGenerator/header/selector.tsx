import { useQuery } from "@tanstack/react-query";
import { useCurrentMapId } from "../../../../../stores/common";
import { DialogGeneratorApi } from "../api";
import { useCurrentDialogId, useDialogActions, useDialogs } from "../store";
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
    const dialogs = useDialogs();
    const currentDialogId = useCurrentDialogId();

    return (
    <>
        <DialogsLoader/>
        <Select
            label="Select existing dialog"
            placeholder="Pick dialog"
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
    const actions = useDialogActions();
    const { data } = useDialogsQuery(currentMapId!);
    
    if (data != undefined) {
        actions.loadDialogs(data);
    } else {
        return null;
    }

    return null;
}

export default DialogsSelector;
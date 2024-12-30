import { Checkbox } from "antd";
import { useCurrentQuestStore } from "../../../stores/QuestStore";
import { useShallow } from "zustand/shallow";
import { invoke } from "@tauri-apps/api/core";

function QuestProps() {
    return <div style={{paddingTop: '2%', display: 'flex', flexDirection: 'row', justifyContent: 'center'}}>
        <SecondarySelector/>
        <InitiallyActiveSelector/>
    </div>
}

function SecondarySelector() {

    const [id, secondary, setSecondary] = useCurrentQuestStore(useShallow((state) => [state.id, state.secondary, state.set_secondary]))

    async function updateIsSecondary(checked: boolean) {
        setSecondary(checked)
        await invoke("update_is_secondary", {questId: id, isSecondary: checked})
    }

    return <>
        <Checkbox checked={secondary} onChange={(e) => updateIsSecondary(e.target.checked)} title="Второстепенный">Второстепенный</Checkbox>
    </>
}

function InitiallyActiveSelector() {

    const [id, active, setActive] = useCurrentQuestStore(useShallow((state) => [state.id, state.active, state.set_active]))

    async function updateIsActive(checked: boolean) {
        setActive(checked)
        await invoke("update_is_active", {questId: id, isSecondary: checked})
    }

    return <>
        <Checkbox checked={active} onChange={(e) => updateIsActive(e.target.checked)} title="Изначально активен">Изначально активен</Checkbox>
    </>
}

export default QuestProps;
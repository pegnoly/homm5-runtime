import { Checkbox } from "antd";
import { useCurrentQuestStore } from "../../../stores/QuestStore";
import { useShallow } from "zustand/shallow";
import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";

function QuestProps() {
    return <div style={{paddingTop: '2%', display: 'flex', flexDirection: 'row', justifyContent: 'center'}}>
        <SecondarySelector/>
        <InitiallyActiveSelector/>
    </div>
}

function SecondarySelector() {

    const [id, secondary, setSecondary] = useCurrentQuestStore(useShallow((state) => [state.id, state.secondary, state.set_secondary]))

    useEffect(() => {
        if (id != null) {
            loadIsSecondary()
        }
    }, [id])

    const loadIsSecondary = async () => {
        await invoke<boolean>("load_quest_is_secondary", {questId: id}).then((res) => setSecondary(res))
    }

    async function updateIsSecondary(checked: boolean) {
        setSecondary(checked)
        await invoke("update_is_secondary", {questId: id, isSecondary: checked})
    }

    return <>
        <Checkbox checked={secondary} onChange={(e) => updateIsSecondary(e.target.checked)} title="Secondary">Secondary?</Checkbox>
    </>
}

function InitiallyActiveSelector() {

    const [id, active, setActive] = useCurrentQuestStore(useShallow((state) => [state.id, state.active, state.set_active]))

    useEffect(() => {
        if (id != null) {
            loadIsActive()
        }
    }, [id])

    const loadIsActive = async () => {
        await invoke<boolean>("load_quest_is_active", {questId: id}).then((res) => setActive(res))
    }

    async function updateIsActive(checked: boolean) {
        setActive(checked)
        await invoke("update_is_active", {questId: id, isSecondary: checked})
    }

    return <>
        <Checkbox checked={active} onChange={(e) => updateIsActive(e.target.checked)} title="Initially active">Initially active?</Checkbox>
    </>
}

export default QuestProps;
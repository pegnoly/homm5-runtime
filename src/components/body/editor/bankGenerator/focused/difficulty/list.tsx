import { SegmentedControl, Title } from "@mantine/core";
import { BankDifficultyStore } from "./store";
import { BankDifficultyType } from "../../types";

function BankDifficultiesList() {
    const type = BankDifficultyStore.useType();
    const actions = BankDifficultyStore.useActions()

    return (
    <div>
        <Title size="sm">Difficulties</Title>
        <SegmentedControl
            orientation="vertical"
            value={type}
            onChange={(value) => actions.updateType(value as BankDifficultyType)}
            data={[
                {value: BankDifficultyType.Easy, label: "Easy"},
                {value: BankDifficultyType.Medium, label: "Medium"},
                {value: BankDifficultyType.Hard, label: "Hard"},
                {value: BankDifficultyType.Critical, label: "Critical"},
                {value: BankDifficultyType.Boss, label: "Boss"}
            ]}
        />
    </div>
    )
}

export default BankDifficultiesList;
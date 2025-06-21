import { SegmentedControl } from "@mantine/core"

function FightAssetStackSelector(params: {
    stackIds: number [],
    currentSelectedStack: number | undefined,
    stackSelectedCallback: (value: number) => void
}) {
    return (
    <>
        <SegmentedControl
            radius={0}
            orientation="vertical"
            value={params.currentSelectedStack?.toString()}
            onChange={(value) => params.stackSelectedCallback(parseInt(value))}
            data={params.stackIds.map((stack, index) => ({label: (index + 1).toString(), value: stack.toString()}))}/>
    </>
    )
}

export default FightAssetStackSelector;
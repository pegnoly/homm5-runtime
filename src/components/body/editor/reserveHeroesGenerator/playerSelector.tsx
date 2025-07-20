import { SegmentedControl } from "@mantine/core"

function ReserveHeroesGeneratorPlayerSelector({current, onSelected}: {
    current: number,
    onSelected: (value: number) => void
}
) {

    return (
    <SegmentedControl
        radius={0}
        onChange={(value) => onSelected(parseInt(value))}
        value={current.toString()}
        data={[
            {value: "1", label: "1"},
            {value: "2", label: "2"},
            {value: "3", label: "3"},
            {value: "4", label: "4"},
            {value: "5", label: "5"},
            {value: "6", label: "6"},
            {value: "7", label: "7"},
            {value: "8", label: "8"}
        ]}
    ></SegmentedControl>
    )
}

export default ReserveHeroesGeneratorPlayerSelector;
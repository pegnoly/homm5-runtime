import {MultiSelect, NumberInput, TextInput} from "@mantine/core";
import { useState } from "react";
import {TownTypeExtended} from "@/components/body/editor/fightGenerator/types.ts";

export type CreatureSelectionRequest = {
    id?: number,
    nameFilter?: string,
    townsFilter?: TownTypeExtended[]
}

function CreatureToEditSelector({onSelectionUpdated}: { onSelectionUpdated: (request: CreatureSelectionRequest) => void }) {

    const [towns, setTowns] = useState<TownTypeExtended[]>([]);
    const [selectedId, setSelectedId] = useState<number|undefined>(undefined);
    const [nameFilter, setNameFilter] = useState<string>("");

    function townsUpdated(newTowns: TownTypeExtended[]) {
        setTowns(newTowns);
        onSelectionUpdated({
            id: selectedId,
            nameFilter: nameFilter,
            townsFilter: newTowns
        });
    }

    function nameFilterUpdated(newFilter: string) {
        setNameFilter(newFilter);
        onSelectionUpdated({
            id: selectedId,
            nameFilter: newFilter,
            townsFilter: towns
        });
    }

    function selectedIdUpdated(newId: number) {
        setSelectedId(newId);
        onSelectionUpdated({
            id: newId,
            nameFilter: nameFilter,
            townsFilter: towns
        });
    }

    return (
    <>
        <div style={{display: 'flex', flexDirection: 'column', justifyContent: 'space-around', alignItems: 'center', gap: '5%'}}>
            <div>
                <div style={{display: 'flex', flexDirection: 'row', gap: '2'}}>
                    <TextInput
                        radius={0}
                        label="Фильтр по имени"
                        size="xs"
                        value={nameFilter}
                        onChange={(value) => nameFilterUpdated(value.target.value)}
                    />
                    <NumberInput
                        radius={0}
                        size="xs"
                        allowNegative={false}
                        label="Фильтр по номеру"
                        value={selectedId}
                        onChange={(value) => {
                            if (typeof(value) === "string") {
                                selectedIdUpdated(parseInt(value))
                            } else {
                                selectedIdUpdated(value)
                            }
                        }}
                    />
                </div>
                <MultiSelect
                    radius={0}
                    miw={100}
                    size="xs"
                    label="Фильтр по городам"
                    value={towns}
                    onChange={(value) => {
                        townsUpdated(value.map(v => v as TownTypeExtended));
                    }}
                    data={[
                        {value: TownTypeExtended.TownAcademy, label: "Academy"},
                        {value: TownTypeExtended.TownDungeon, label: "Dungeon"},
                        {value: TownTypeExtended.TownHeaven, label: "Heaven"},
                        {value: TownTypeExtended.TownInferno, label: "Inferno"},
                        {value: TownTypeExtended.TownFortress, label: "Fortress"},
                        {value: TownTypeExtended.TownPreserve, label: "Preserve"},
                        {value: TownTypeExtended.TownNecromancy, label: "Necromancy"},
                        {value: TownTypeExtended.TownStronghold, label: "Stronghold"},
                        {value: TownTypeExtended.TownBastion, label: "Bastion"},
                        {value: TownTypeExtended.TownSanctuary, label: "Sanctuary"},
                        {value: TownTypeExtended.TownRenegades, label: "Renegades"},
                        {value: TownTypeExtended.TownNoType, label: "Neutral"},
                    ]}
                />
            </div>
        </div>
    </>
    )
}

export default CreatureToEditSelector;
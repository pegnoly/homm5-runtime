import { ArtifactSlotType } from "./types";
import { useState } from "react";
import ArtifactListItem from "./item";
import useGameDataStore from "../../../../../../stores/GameDataStore";
import { ActionIcon, ComboboxItem, Group, List, OptionsFilter, SegmentedControl, Select, Text } from "@mantine/core";
import { useCurrentArtifactsActions, useCurrentArtifactsAssetId, useOptionalArtifacts } from "./store";
import { useMutation } from "@tanstack/react-query";
import { FightGeneratorApi } from "../../api";
import { IconPlus } from "@tabler/icons-react";

export type AddOptionalArtifactPayload = {
    assetId: number,
    slot: ArtifactSlotType,
    artifactId: number
}

export type RemoveOptionalArtifactPayload = {
    assetId: number,
    slot: ArtifactSlotType,
    artifactId: number
}

function OptionalArtifactsList() {
    const artifacts = useGameDataStore((state) => state.artifacts);

    const artifactsAssetId = useCurrentArtifactsAssetId();
    const optionalArtifacts = useOptionalArtifacts();
    const actions = useCurrentArtifactsActions();

    const [slot, setSlot] = useState<ArtifactSlotType>(ArtifactSlotType.Primary);
    const [selectedId, setSelectedId] = useState<number | null>(null);
    
    const addMutation = useMutation({
        mutationFn: async(payload: AddOptionalArtifactPayload) => {
            return FightGeneratorApi.addOptionalArtifact(payload);
        },
        onSuccess(_data, variables, _context) {
            setSelectedId(null);
            actions.addOptionalArtifact(slot, variables.artifactId)
        },
    });

    const removeMutation = useMutation({
        mutationFn: async(payload: RemoveOptionalArtifactPayload) => {
            return FightGeneratorApi.removeOptionalArtifact(payload);
        },
        onSuccess(_data, variables, _context) {
            setSelectedId(null);
            actions.removeOptionalArtifact(slot, variables.artifactId)
        },
    });

    async function removeArtifact(artId: number) {
        removeMutation.mutate({assetId: artifactsAssetId!, slot: slot, artifactId: artId});
    }

    const optionsFilter: OptionsFilter = ({ options, search }) => {
        const splittedSearch = search.toLowerCase().trim().split(' ');
        return (options as ComboboxItem[]).filter((option) => {
            const words = option.label.toLowerCase().trim().split(' ');
            return splittedSearch.every((searchWord) => words.some((word) => word.includes(searchWord)));
        });
    };

    return <div style={{display: 'flex', flexDirection: 'column', paddingLeft: '5%'}}>
        <Text style={{fontSize: 15, color: 'darkorchid', fontWeight: 'bold', fontStretch: 'expanded'}}>Optional artifacts</Text>
        <div style={{width: '100%', height: '100%'}}>
            <div style={{display: 'flex', flexDirection: 'row', gap: '5%'}}>
                <SegmentedControl
                    size="xs"
                    value={slot}
                    onChange={(value) => {
                        setSelectedId(null);
                        setSlot(value as ArtifactSlotType);
                    }}
                    orientation="vertical"
                    data={[
                        {value: ArtifactSlotType.Chest, label: "Chest"},
                        {value: ArtifactSlotType.Neck, label: "Neck"},
                        {value: ArtifactSlotType.Miscslot1, label: "Pocket"},
                        {value: ArtifactSlotType.Head, label: "Head"},
                        {value: ArtifactSlotType.Primary, label: "Primary"},
                        {value: ArtifactSlotType.Secondary, label: "Secondary"},
                        {value: ArtifactSlotType.Shoulders, label: "Shoulders"},
                        {value: ArtifactSlotType.Feet, label: "Feet"},
                        {value: ArtifactSlotType.Finger, label: "Finger"},
                        {value: ArtifactSlotType.Inventory, label: "Inventory"},
                    ]}
                />
                <div style={{display: 'flex', flexDirection: 'column', justifyItems: 'self-start'}}>
                    <Group>
                        <Select 
                            size="xs"
                            searchable 
                            style={{width: 150}} 
                            value={selectedId?.toString()} 
                            onChange={(value) => setSelectedId(parseInt(value!))}
                            filter={optionsFilter}
                            data={artifacts.filter(art => art.slot == slot).filter(art => !optionalArtifacts?.values[slot]?.includes(art.id)).map(art => ({
                                value: art.id.toString(), label: art.name
                            }))}
                        />
                        <ActionIcon disabled={!selectedId} onClick={() => addMutation.mutate({assetId: artifactsAssetId!, slot: slot, artifactId: selectedId!})}>
                            <IconPlus/>
                        </ActionIcon>
                    </Group>
                    {
                        optionalArtifacts!.values[slot].length > 0 ?
                        <div>
                            <List>{optionalArtifacts!.values[slot].map((art, index) => (
                                <div key={index}>
                                    <ArtifactListItem 
                                        artifactId={art}
                                        artifactName={artifacts.find(a => a.id == art)?.name!}
                                        removeCallback={removeArtifact}
                                    />
                                </div>
                            ))}</List>
                        </div> :
                        null
                    }
                </div>
            </div>
        </div>
    </div> 
}

export default OptionalArtifactsList;
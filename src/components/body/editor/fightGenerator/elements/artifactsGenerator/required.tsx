import { useState } from "react";
import useGameDataStore from "../../../../../../stores/GameDataStore";
import { useCurrentArtifactsActions, useCurrentArtifactsAssetId, useRequiredArtifacts } from "./store";
import { useMutation } from "@tanstack/react-query";
import { FightGeneratorApi } from "../../api";
import { ActionIcon, ComboboxItem, Group, List, OptionsFilter, Select, Text } from "@mantine/core";
import ArtifactListItem from "./item";
import { IconPlus } from "@tabler/icons-react";

export type AddRequiredArtifactPayload = {
    assetId: number,
    artifactId: number
}

export type RemoveRequiredArtifactPayload = {
    assetId: number,
    artifactId: number
}

function RequiredArtifactsList() {
    const artifacts = useGameDataStore(state => state.artifacts);
    const artifactsAssetId = useCurrentArtifactsAssetId();
    const actions = useCurrentArtifactsActions();
    const requiredArtifacts = useRequiredArtifacts();

    const [selectedId, setSelectedId] = useState<number | null>(null);

    const addMutation = useMutation({
        mutationFn: async(payload: AddRequiredArtifactPayload) => {
            return FightGeneratorApi.addRequiredArtifact(payload);
        },
        onSuccess(_data, variables, _context) {
            setSelectedId(null);
            actions.addRequiredArtifact(variables.artifactId)
        },
    });

    const removeMutation = useMutation({
        mutationFn: async(payload: RemoveRequiredArtifactPayload) => {
            return FightGeneratorApi.removeRequiredArtifact(payload);
        },
        onSuccess(_data, variables, _context) {
            setSelectedId(null);
            actions.removeRequiredArtifact(variables.artifactId)
        },
    });

    async function removeArtifact(artId: number) {
        removeMutation.mutate({assetId: artifactsAssetId!, artifactId: artId})
    }

    const optionsFilter: OptionsFilter = ({ options, search }) => {
        const splittedSearch = search.toLowerCase().trim().split(' ');
        return (options as ComboboxItem[]).filter((option) => {
            const words = option.label.toLowerCase().trim().split(' ');
            return splittedSearch.every((searchWord) => words.some((word) => word.includes(searchWord)));
        });
    };

    return <div style={{display: 'flex', flexDirection: 'column', justifyItems: 'start', paddingLeft: '10%', overflow: 'auto'}}>
        <Text style={{fontSize: 15, color: 'darkorchid', fontWeight: 'bold', fontStretch: 'expanded'}}>Required artifacts</Text>
        <Group>
            <Select 
                size="xs"
                searchable 
                style={{width: 150}} 
                value={selectedId?.toString()} 
                onChange={(value) => setSelectedId(parseInt(value!))}
                filter={optionsFilter}
                data={artifacts.filter(art => !requiredArtifacts?.ids.includes(art.id)).map(art => ({
                    value: art.id.toString(), label: art.name
                }))}
            />
            <ActionIcon disabled={!selectedId} onClick={() => addMutation.mutate({assetId: artifactsAssetId!, artifactId: selectedId!})}>
                <IconPlus/>
            </ActionIcon>
        </Group>
        <List>{requiredArtifacts?.ids.map((art, index) => (
            <div style={{paddingTop: '1%'}} key={index}>
                <ArtifactListItem artifactId={art} artifactName={artifacts.find(a => a.id == art)?.name!} removeCallback={removeArtifact}/>
            </div>
        ))}</List>
    </div>
}

export default RequiredArtifactsList;
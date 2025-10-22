import { Button, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, Select } from "@mantine/core"
import { AssetGenerationType } from "../../types"
import { FightAssetArtifactsModel } from "./types"
import { useDisclosure } from "@mantine/hooks"
import { useState } from "react"
import { useMutation } from "@tanstack/react-query"
import { FightGeneratorApi } from "../../api"
import { UUID } from "crypto"

export type CreateArtifactsAssetPayload = {
    assetId: UUID,
    generationType: AssetGenerationType
}

function ArtifactsAssetCreator({assetId, onCreated}: {
    assetId: UUID,
    onCreated: (value: FightAssetArtifactsModel) => void
}) {
    const [opened, {open, close}] = useDisclosure(false);
    const [selectedType, setSelectedType] = useState<AssetGenerationType | null>(null);

    const mutation = useMutation({
        mutationFn: async(payload: CreateArtifactsAssetPayload) => {
            return FightGeneratorApi.createArtifactsAsset(payload)
        },
        onSuccess(data, _variables, _context) {
            console.log("Here: ", data);
            close();
            onCreated(data);
        },
    })

    return (
    <>
        <Button c="dark" radius={0} size="md" justify="start" maw={200} onClick={open}>Create artifacts asset</Button>
        <ModalRoot opened={opened} centered onClose={close}>
            <ModalOverlay/>
            <ModalContent>
                <ModalHeader>
                    <ModalTitle>Artifacts asset creation</ModalTitle>
                    <ModalCloseButton/>
                </ModalHeader>
                <ModalBody>
                    <Select
                        value={selectedType}
                        onChange={(value) => setSelectedType(value as AssetGenerationType)}
                        placeholder="Define do artifacts have dynamic generation or not"
                        label="Define generation"
                        data={[
                            {label: "Static(artifacts cost doesn't grow over time)", value: AssetGenerationType.Static},
                            {label: "Dynamic(artifacts cost grows over time)", value: AssetGenerationType.Dynamic}
                        ]}
                    />
                    <Group justify="end">
                        <Button 
                            disabled={!selectedType} 
                            onClick={() => mutation.mutate({assetId: assetId, generationType: selectedType!})} 
                            radius={0}
                        >
                            Create
                        </Button>
                    </Group>
                </ModalBody>
            </ModalContent>
        </ModalRoot>
    </>
    )
}

export default ArtifactsAssetCreator;
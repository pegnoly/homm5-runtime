import { Button, Group, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay, ModalRoot, ModalTitle, Select } from "@mantine/core"
import { AssetGenerationType } from "../../types"
import { FightAssetArtifactsModel } from "./types"
import { useDisclosure } from "@mantine/hooks"
import { useState } from "react"
import { useMutation } from "@tanstack/react-query"
import { FightGeneratorApi } from "../../api"

export type CreateArtifactsAssetPayload = {
    assetId: number,
    generationType: AssetGenerationType
}

function ArtifactsAssetCreator({assetId, onCreated}: {
    assetId: number,
    onCreated: (value: FightAssetArtifactsModel) => void
}) {
    const [opened, {open, close}] = useDisclosure(false);
    const [selectedType, setSelectedType] = useState<AssetGenerationType | null>(null);

    const mutation = useMutation({
        mutationFn: async(payload: CreateArtifactsAssetPayload) => {
            return FightGeneratorApi.createArtifactsAsset(payload)
        },
        onSuccess(data, _variables, _context) {
            close();
            onCreated(data);
        },
        onError: (e) => {
            console.log("Error: ", e);
        }
    })

    return (
    <>
        <Button c="dark" radius={0} size="md" justify="start" maw={250} onClick={open}>Создать набор артефактов</Button>
        <ModalRoot opened={opened} centered onClose={close}>
            <ModalOverlay/>
            <ModalContent>
                <ModalHeader>
                    <ModalTitle>Создание набора</ModalTitle>
                    <ModalCloseButton/>
                </ModalHeader>
                <ModalBody>
                    <Select
                        value={selectedType}
                        onChange={(value) => setSelectedType(value as AssetGenerationType)}
                        placeholder="Определить прирост велью артефактов"
                        label="Влияет на скейл велью от времени игры"
                        data={[
                            {label: "Не растет", value: AssetGenerationType.Static},
                            {label: "Растет", value: AssetGenerationType.Dynamic}
                        ]}
                    />
                    <Group justify="end">
                        <Button 
                            disabled={!selectedType} 
                            onClick={() => mutation.mutate({assetId: assetId, generationType: selectedType!})} 
                            radius={0}
                        >
                            Создать
                        </Button>
                    </Group>
                </ModalBody>
            </ModalContent>
        </ModalRoot>
    </>
    )
}

export default ArtifactsAssetCreator;
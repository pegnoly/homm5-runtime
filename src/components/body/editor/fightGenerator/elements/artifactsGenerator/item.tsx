import { ActionIcon, Text } from "@mantine/core";
import { IconTrash } from "@tabler/icons-react";

function ArtifactListItem(params: {
    artifactId: number,
    artifactName: string,
    removeCallback: (value: number) => void
}) {
    return <div style={{display: 'flex', flexDirection: 'row', gap: 10, width: '100%', justifyContent: 'space-around'}}>
        <Text style={{fontFamily: 'cursive', fontWeight: 'bolder', fontSize: 14, width: '70%'}}>{params.artifactName}</Text>
        <ActionIcon onClick={() => params.removeCallback(params.artifactId)}>
            <IconTrash/>
        </ActionIcon>
    </div>
}

export default ArtifactListItem;
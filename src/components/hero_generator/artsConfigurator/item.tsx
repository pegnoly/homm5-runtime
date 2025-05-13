import { Button, Typography } from "antd";
import { ArtifactModel } from "../../../stores/GameDataStore";
import { DeleteFilled } from "@ant-design/icons";

function ArtifactListItem(params: {artifact: ArtifactModel, removeArtifactCallback: (artId: number) => void}) {
    return <div style={{display: 'flex', flexDirection: 'row', gap: 15}}>
        <Typography.Text style={{fontFamily: 'cursive', fontWeight: 'bolder', fontSize: 15}}>{params.artifact.name}</Typography.Text>
        <Button onClick={() => params.removeArtifactCallback(params.artifact.id)} icon={<DeleteFilled/>}/>
    </div>
}

export default ArtifactListItem;
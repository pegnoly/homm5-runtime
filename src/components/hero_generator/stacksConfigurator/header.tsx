import { Segmented, Space, Typography } from "antd";
import HeroAssetStackCreator from "./creator";

function HeroAssetStacksConfiguratorHeader(params: {
    assetId: number,
    stacks: number[],
    currentStack: number | null,
    stackSelectedCallback: (value: number) => void,
    stackCreatedCallback: (value: number) => void
}) {
    // const [currentStack, setCurrentStack] = useHeroGeneratorStore(useShallow((state) => [state.currentStack, state.setCurrentStack]));
    return <>
        <Typography.Text style={{fontFamily: 'fantasy', fontSize: 20, color: 'darkorchid', fontStretch: 'expanded'}}>Stacks data</Typography.Text>
        <Space>
            <Segmented
                value={params.currentStack!}
                onChange={params.stackSelectedCallback}
                options={params.stacks.map((id, index) => ({value: id, label: (index + 1).toString()}))}
            />
            <HeroAssetStackCreator 
                disabled={params.stacks.length >= 7} 
                assetId={params.assetId} 
                createCallback={params.stackCreatedCallback}
            />
        </Space> 
    </>
}

export default HeroAssetStacksConfiguratorHeader;
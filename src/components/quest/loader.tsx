import { Button, Select } from "antd";

function QuestLoader() {
    
    return <div style={{display: 'flex', flexDirection: 'column', width: '45%', gap: 2}}>
        <Select style={{height: 25}}></Select>
        <Button style={{height: 25}}>Загрузить квест</Button>
    </div>
}

export default QuestLoader;
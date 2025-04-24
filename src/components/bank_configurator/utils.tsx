import { Typography } from "antd"

function BankStringProperty(params: {
    text: string,
    initialValue: any,
    updateCallback: (updated: string) => void
}) {
    return <div style={{display: 'flex', flexDirection: 'row', gap: 10}}>
        <Typography.Text style={{fontFamily: 'cursive', fontWeight: 'bold'}}>{`${params.text}: `}</Typography.Text>
        <Typography.Text 
            style={{fontFamily: 'cursive', fontWeight: 'bold', color: 'green'}}
            editable={{onChange: params.updateCallback}}
        >{params.initialValue}</Typography.Text>
    </div>
}

export default BankStringProperty;
import { Typography } from "antd"

function BankStringProperty(params: {
    text: string,
    initialValue: any,
    updateCallback: (updated: string) => void
}) {
    return <div style={{display: 'flex', flexDirection: 'row', gap: 10}}>
        <Typography.Text style={{fontFamily: 'cursive', fontWeight: 'bold', fontSize: 19}}>{`${params.text}: `}</Typography.Text>
        <Typography.Text 
            style={{fontFamily: 'cursive', fontWeight: 'bold', color: 'green', fontSize: 19}}
            editable={{onChange: params.updateCallback}}
        >{params.initialValue}</Typography.Text>
    </div>
}

export default BankStringProperty;
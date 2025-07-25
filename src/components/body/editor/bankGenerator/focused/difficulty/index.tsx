import BankDifficultiesList from "./list";
import BankDifficultySelected from "./selected";

function BankDifficultyInfo() {
    return (
    <div style={{width: '100%', height: '100%', display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'space-around'}}>
        <BankDifficultiesList/>
        <BankDifficultySelected/>
    </div>
    )
}

export default BankDifficultyInfo;
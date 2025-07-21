import { useState } from "react";
import { MagicSchool } from "../../types";
import { Select } from "@mantine/core";
import ReserveHeroSpellsSelector from "./selector";
import ReserveHeroSpellsList from "./list";

function ReserveHeroSpellsPanel() {
    const [school, setSchool] = useState<MagicSchool>(MagicSchool.Destructive);

    return (
    <div style={{width: '100%', height: '100%', display: 'flex', flexDirection: 'column', overflow: 'auto', paddingRight: '2%'}}>
        <Select
            size="xs"
            radius={0}
            label="Select school"
            value={school}
            onChange={(value) => setSchool(value as MagicSchool)}
            data={[
                {value: MagicSchool.Destructive, label: "Destructive"},
                {value: MagicSchool.Dark, label: "Dark"},
                {value: MagicSchool.Light, label: "Light"},
                {value: MagicSchool.Summoning, label: "Summoning"},
                {value: MagicSchool.Runic, label: "Runic"},
                {value: MagicSchool.Warcries, label: "Warcries"},
            ]}
        />
        <ReserveHeroSpellsSelector school={school}/>
        <ReserveHeroSpellsList/>
    </div>
    )
}

export default ReserveHeroSpellsPanel;
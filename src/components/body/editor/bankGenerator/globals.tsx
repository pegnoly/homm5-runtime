import { Button, Stack } from "@mantine/core";
import { invoke } from "@tauri-apps/api/core";

function BankGeneratorGlobals() {
    async function startGeneration() {
        await invoke("generate_banks_script");
    }

    return (
    <Stack>
        <Button onClick={startGeneration} radius={0} size="xs">Generate scripts for banks</Button>
    </Stack>
    )
}

export default BankGeneratorGlobals;
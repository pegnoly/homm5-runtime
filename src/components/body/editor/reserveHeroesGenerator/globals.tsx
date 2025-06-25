import { Button, Stack } from "@mantine/core";
import { invoke } from "@tauri-apps/api/core";

function ReserveHeroesGeneratorGlobals() {

    return (
    <Stack>
        <Button
            radius={0}
            size="xs"
            onClick={() => invoke("apply_modifications")}
        >Apply modifications
        </Button>
    </Stack>
    )
}

export default ReserveHeroesGeneratorGlobals;
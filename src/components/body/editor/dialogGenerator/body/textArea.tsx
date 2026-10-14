import { ButtonGroup, Textarea, UnstyledButton } from "@mantine/core";
import { DialogGeneratorStore } from "../store";
import { useRef, useState } from "react";

function DialogGeneratorTextArea() {
    const textAreaRef = useRef<HTMLTextAreaElement>(null);

    const [_, setCaretPosition] = useState<number>(0);

    const currentVariantText = DialogGeneratorStore.useCurrentVariantText();
    const actions = DialogGeneratorStore.useActions();

    const handleCursorUpdate = () => {
        if (textAreaRef.current) {
            setCaretPosition(textAreaRef.current.selectionStart)
        }
    }

    const insertTextAtCaret = (startText: string, endText: string) => {
        if (!textAreaRef.current || currentVariantText == undefined) {
            return;
        }

        const start = textAreaRef.current.selectionStart;
        const end = textAreaRef.current.selectionEnd;
        const newText = currentVariantText.substring(0, start) + startText + currentVariantText.substring(start, end) + endText + currentVariantText.substring(end);
        actions.setCurrentVariantText(newText);
        actions.setCurrentVariantSaved(false);
    }

    return (
        <div style={{ height: "100%", display: "flex", flexDirection: "column", gap: 0}}>
            <ButtonGroup>
                <UnstyledButton onClick={() => insertTextAtCaret("<b>", "</b>")}>
                    B
                </UnstyledButton>
            </ButtonGroup>
            <Textarea
                ref={textAreaRef}
                // style={{padding: "4%"}}
                radius={0}
                rows={20}
                minRows={20}
                maxRows={25}
                value={currentVariantText}
                onChange={(e) => {
                    actions.setCurrentVariantText(e.currentTarget.value);
                    actions.setCurrentVariantSaved(false);
                }}
                onMouseUp={handleCursorUpdate}
                onKeyUp={handleCursorUpdate}
                onKeyDown={(e) => {
                    if (e.key == "b" && e.ctrlKey) {
                        insertTextAtCaret("<b>", "</b>")
                    }
                }}
            />
        </div>
    )
}

export default DialogGeneratorTextArea;
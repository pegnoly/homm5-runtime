import { ComponentType, ReactNode, useEffect, useState } from 'react';
import { Text, TextInput, Button, Group, Box, MantineFontSize, NumberInput } from '@mantine/core';
import { IconCheck, IconPencilCheck } from '@tabler/icons-react';

export type EditablePropertyWrapperProps = {
  children: ReactNode,
  value?: string | number
}

export type EditablePropertyWrapper = {
  component: ComponentType<EditablePropertyWrapperProps>,
};

function EditableProperty({
  size = "sm", 
  type = "number", 
  label, 
  initialValue, 
  onSave, 
  tooltip
}: {
  size? : MantineFontSize,
  type?: "number" | "text",
  label: string,
  initialValue: string | number,
  onSave: (value: string | number) => void,
  tooltip?: EditablePropertyWrapper
}) {
  const [isEditing, setIsEditing] = useState<boolean>(false);
  const [value, setValue] = useState<string | number>(initialValue);

  useEffect(() => {
    setValue(initialValue);
  }, [initialValue])

  const handleSave = () => {
    setIsEditing(false);
    onSave(value);
  };

  return (
  <>
    <Box style={{display: 'flex', flexDirection: 'row', justifySelf: 'center'}}>
      {isEditing ? (
        <Group align="flex-end" gap="xs">
          {/* <Text>{params.label}</Text> */}
          {
            type == "number" ? 
            <NumberInput
              value={value}
              onChange={(e) => setValue(e)}
              onKeyDown={(e) => e.key === 'Enter' && handleSave()}
              // placeholder={params.placeholder}
              autoFocus
              w="50%"
            /> :
            <TextInput
              value={value}
              onChange={(e) => setValue(e.currentTarget.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleSave()}
              // placeholder={params.placeholder}
              autoFocus
              w="50%"
            />
          }
          <Button
            variant="subtle"
            onClick={handleSave}
            // leftSection={<IconCheck size={16} />}
          >
            <IconCheck size={12}/>
          </Button>
        </Group>
      ) : (
        (
          tooltip ? 
          <tooltip.component value={value}>
            <div style={{display: 'flex', flexDirection: 'row', gap: '2.5%', alignItems: 'center'}}>
              <Button
                variant="transparent"
                size='xs'
                onClick={() => setIsEditing(true)}
              >
                <IconPencilCheck/>
              </Button>
              <Text size={size} style={{fontWeight: 'bold', fontFamily: 'cursive'}}>{label}</Text>
              <Text>{value}</Text>
            </div>
          </tooltip.component> :
          <div style={{display: 'flex', flexDirection: 'row', gap: '2.5%', alignItems: 'center'}}>
            <Button
              variant="transparent"
              size='xs'
              onClick={() => setIsEditing(true)}
            >
              <IconPencilCheck/>
            </Button>
            <Text size={size} style={{fontWeight: 'bold', fontFamily: 'cursive'}}>{label}</Text>
            <Text>{value}</Text>
          </div>
        )
      )}
    </Box>
  </>
  )
}

export default EditableProperty;
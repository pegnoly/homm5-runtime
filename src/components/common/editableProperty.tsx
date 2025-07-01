import { ComponentType, ReactNode, useEffect, useState } from 'react';
import { Text, TextInput, Button, Group, Box } from '@mantine/core';
import { IconCheck, IconPencilCheck } from '@tabler/icons-react';

export type EditablePropertyWrapperProps = {
  children: ReactNode,
  value: string
}

export type EditablePropertyWrapper = {
  component: ComponentType<EditablePropertyWrapperProps>,
};

function EditableProperty(params: {
  type?: "input" | "textarea"
  label: string,
  initialValue: string,
  onSave: (value: string) => void,
  tooltip?: EditablePropertyWrapper
}) {
  const [isEditing, setIsEditing] = useState<boolean>(false);
  const [value, setValue] = useState<string>(params.initialValue);

  useEffect(() => {
    setValue(params.initialValue);
  }, [params.initialValue])

  const handleSave = () => {
    setIsEditing(false);
    params.onSave(value);
  };

  return (
  <>
    <Box style={{display: 'flex', flexDirection: 'row', justifySelf: 'center'}}>
      {isEditing ? (
        <Group align="flex-end" gap="xs">
          {/* <Text>{params.label}</Text> */}
          <TextInput
            value={value}
            onChange={(e) => setValue(e.currentTarget.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleSave()}
            // placeholder={params.placeholder}
            autoFocus
            w="50%"
          />
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
          params.tooltip ? 
          <params.tooltip.component value={value}>
            <div style={{display: 'flex', flexDirection: 'row', gap: '2.5%', alignItems: 'center'}}>
              <Button
                variant="transparent"
                size='xs'
                onClick={() => setIsEditing(true)}
              >
                <IconPencilCheck/>
              </Button>
              <Text style={{fontWeight: 'bold', fontFamily: 'cursive'}}>{params.label}</Text>
              <Text>{value}</Text>
            </div>
          </params.tooltip.component> :
          <div style={{display: 'flex', flexDirection: 'row', gap: '2.5%', alignItems: 'center'}}>
            <Button
              variant="transparent"
              size='xs'
              onClick={() => setIsEditing(true)}
            >
              <IconPencilCheck/>
            </Button>
            <Text style={{fontWeight: 'bold', fontFamily: 'cursive'}}>{params.label}</Text>
            <Text>{value}</Text>
          </div>
        )
      )}
    </Box>
  </>
  )
}

export default EditableProperty;
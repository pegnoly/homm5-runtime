// components/CustomContextMenu.tsx
import { Menu, Paper, Text } from '@mantine/core';
import { CSSProperties } from 'react';

export interface AssetMenuItem {
    label: string;
    icon?: React.ReactNode;
    onClick: () => void;
}

interface AssetContextMenuProps {
    x: number;
    y: number;
    visible: boolean;
    items: AssetMenuItem[];
    onClose: () => void;
}

export const AssetContexMenu = ({
    x,
    y,
    visible,
    items,
    onClose,
}: AssetContextMenuProps) => {
    if (!visible) return null;

    const menuStyle: CSSProperties = {
        position: 'fixed',
        top: y,
        left: x,
        zIndex: 1000,
    };

    return (
        <Paper
            style={menuStyle}
            shadow="md"
            p={4}
            withBorder
            onClick={(e) => e.stopPropagation()}
        >
            <Menu opened withinPortal={false} onClose={onClose}>
                <Menu.Target>
                    <div style={{ position: 'fixed', top: y, left: x, width: 0, height: 0 }} />
                </Menu.Target>
                    <Menu.Dropdown>
                        {items.map((item, index) => (
                        <Menu.Item
                            key={index}
                            leftSection={item.icon}
                            onClick={() => {
                                item.onClick();
                                onClose();
                            }}
                        >
                            <Text size="sm">{item.label}</Text>
                        </Menu.Item>
                        ))}
                    </Menu.Dropdown>
                </Menu>
        </Paper>
    );
};
// hooks/useContextMenu.ts
import { useState, useCallback, useEffect } from 'react';

interface ContextMenuState {
    x: number;
    y: number;
    visible: boolean;
}

export const useContextMenu = () => {
    const [state, setState] = useState<ContextMenuState>({
        x: 0,
        y: 0,
        visible: false,
    });

    const handleContextMenu = useCallback((e: React.MouseEvent) => {
        e.preventDefault();
        setState({
            x: e.clientX,
            y: e.clientY,
            visible: true,
        });
    }, []);

    const hideContextMenu = useCallback(() => {
        setState(prev => ({ ...prev, visible: false }));
    }, []);

    // Close menu when clicking elsewhere
    useEffect(() => {
        const handleClick = () => {
        if (state.visible) hideContextMenu();
        };

        document.addEventListener('click', handleClick);
        return () => document.removeEventListener('click', handleClick);
    }, [state.visible, hideContextMenu]);

    return {
        ...state,
        handleContextMenu,
        hideContextMenu,
    };
};
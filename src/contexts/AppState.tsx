import { useState, createContext, PropsWithChildren, useContext, useEffect } from "react";

export enum AppState {
    NotReady,
    Ready
}


type State = {
    current_state: AppState
}

type AppStateContextProps = {
    state: State;
    setState: (state: State) => void;
};

export const AppStateContext = createContext<AppStateContextProps | undefined>(undefined);

const AppStateProvider = ({children} : PropsWithChildren<{}>) => {
    const [state, setState] = useState<AppStateContextProps['state']>({
        current_state: AppState.NotReady
    });

    useEffect(() => {
        if (state.current_state == AppState.NotReady) {
            setState({...state, current_state: AppState.Ready})
        }
    }, [state])

    return(
        <AppStateContext.Provider value={{state, setState}}>
            {children}
        </AppStateContext.Provider>
    )
}

export const useAppStateContext = () => {
    const context = useContext(AppStateContext);
    return context;
}

export default AppStateProvider;
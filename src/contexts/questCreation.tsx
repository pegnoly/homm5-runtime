import { useState, createContext, PropsWithChildren, useContext } from "react";

type QuestCreationData = {
    directory: string,
    script_name: string,
    name: string
}

type QuestCreationContextProps = {
    state: QuestCreationData;
    setState: (state: QuestCreationData) => void;
};

export const QuestCreationContext = createContext<QuestCreationContextProps | undefined>(undefined);

const QuestCreationProvider = ({children} : PropsWithChildren<{}>) => {
    const [state, setState] = useState<QuestCreationContextProps['state']>({
        directory: "",
        script_name: "",
        name: ""
    });

    return(
        <QuestCreationContext.Provider value={{state, setState}}>
            {children}
        </QuestCreationContext.Provider>
    )
}

export const useQuestCreationContext = () => {
    const context = useContext(QuestCreationContext);
    return context;
}

export default QuestCreationProvider;
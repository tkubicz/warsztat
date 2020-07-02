import React from 'react';
import { AppState } from './AppState/AppState';

const AppStateContext = React.createContext<AppState>(AppState.createForContext());

export const Provider = AppStateContext.Provider;

export const useAppStateContext = (): AppState => {
    return React.useContext(AppStateContext);
};

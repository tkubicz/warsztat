import * as React from 'react';
import * as ReactDOM from 'react-dom';
import { App } from './App/App';
import { Provider } from './Provider';
import { AppState } from './AppState/AppState';

const appState = new AppState();

ReactDOM.render(
    <Provider value={appState}>
        <App />
    </Provider>
,
    document.getElementById('root')
);
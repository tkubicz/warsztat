import * as React from 'react';
import { observer } from 'mobx-react';
import { useAppStateContext } from '../Provider';
import styled from '@emotion/styled';
import { TabTiny } from './TabTiny/TabTiny';
import { TabChat } from './TabChat/TabChat';

interface DivTabButtonPropsType {
    isActive: boolean,
}

const DivTabContainer = styled('div')`
    display: flex;
    border: 1px solid gray;
    background-color: #e0e0e0;
    padding: 5px;
    margin: 5px;
`;

const DivTabButton = styled('div')<DivTabButtonPropsType>`
    ${props => props.isActive ? 'color: red;' : 'opacity: 0.5;'}
    margin-right: 10px;
    cursor: pointer;
`;

const DivBodyWrapper = styled('div')`
    display: flex;
    border: 1px solid gray;
    background-color: #e0e0e0;
    padding: 5px;
    margin: 5px;
`;

const renderTabButton = (label: string, redirect: () => void, isActive: boolean) => {
    return (
        <DivTabButton onClick={redirect} isActive={isActive}>
            { label }
        </DivTabButton>
    )
};

const RenderTabs = observer(() => {
    const appState = useAppStateContext();

    const tab = appState.tab;

    return (
        <DivTabContainer>
            { renderTabButton('TinyUrl', appState.goToTiny, tab === 'tinyurl') }
            { renderTabButton('Chat', appState.goToChat, tab === 'chat') }
        </DivTabContainer>
    )
});

const RenderBody = observer(() => {
    const appState = useAppStateContext();

    const tab = appState.tab;
    
    if (tab === 'tinyurl') {
        return <TabTiny />;
    }

    if (tab === 'chat') {
        return <TabChat />;
    }

    return null;
});

export const App = observer(() => {
    //const appState = useAppStateContext();

    return (
        <div>
            <RenderTabs />
            <DivBodyWrapper>
                <RenderBody />
            </DivBodyWrapper>
        </div>
    );
});


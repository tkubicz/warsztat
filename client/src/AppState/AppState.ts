import { observable } from 'mobx';

export class AppState {
    @observable tab: 'tinyurl' | 'chat' = 'tinyurl';

    goToTiny = () => {
        this.tab = 'tinyurl';
    }

    goToChat = () => {
        this.tab = 'chat';
    }

    static createForContext(): AppState {
        return new AppState();
    }
}
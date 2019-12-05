import {Injectable} from "@angular/core";
import {NavigationCancel, NavigationEnd, NavigationError, NavigationStart, Router} from "@angular/router";

@Injectable({
    providedIn: "root",
})
export class LoadingBarService {
    private openRequests = 0;
    private observers: any = [];

    constructor(private routerService: Router) {

        this.routerService.events.subscribe(event => {
            switch (true) {
                case event instanceof NavigationStart: {
                    this.incrementCounter();
                    break;
                }
                case event instanceof NavigationEnd:
                case event instanceof NavigationCancel:
                case event instanceof NavigationError: {
                    this.decrementCounter();
                    break;
                }
                default:
                    break;
            }
        });
    }

    subscribe(callback: any): void {
        this.observers.push(callback);
    }

    isLoading(): boolean {
        return this.openRequests > 0;
    }

    incrementCounter(): void {
        ++this.openRequests;
        if (this.openRequests === 1)
            this.notifyObservers();
    }

    decrementCounter(): void {
        --this.openRequests;
        if (this.openRequests <= 0) {
            this.openRequests = 0;
            this.notifyObservers();
        }
    }

    private notifyObservers(): void {
        this.observers.forEach(callback => callback.call(callback, this.isLoading()));
    }


}

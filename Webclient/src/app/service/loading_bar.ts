import {Injectable} from "@angular/core";
import {NavigationCancel, NavigationEnd, NavigationError, NavigationStart, Router} from "@angular/router";
import {ObserverPattern} from "../template/class_template/observer_pattern";

@Injectable({
    providedIn: "root",
})
export class LoadingBarService extends ObserverPattern {
    private openRequests = 0;

    constructor(private routerService: Router) {
        super();
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

    isLoading(): boolean {
        return this.openRequests > 0;
    }

    incrementCounter(): void {
        ++this.openRequests;
        if (this.openRequests === 1)
            this.notify();
    }

    decrementCounter(): void {
        --this.openRequests;
        if (this.openRequests <= 0) {
            this.openRequests = 0;
            this.notify();
        }
    }

    private notify(): void {
        super._notify(callback => callback.call(callback, this.isLoading()));
    }


}

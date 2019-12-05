import {Severity} from "../../domain_value/severity";
import {Notification} from "../../material/notification";

export class ObserverPattern {
    observers: any = [];

    notify(on_callback: any): void {
        this.observers.forEach(callback => on_callback(callback));
    }

    subscribe(callback: any): void {
        this.observers.push(callback);
    }
}

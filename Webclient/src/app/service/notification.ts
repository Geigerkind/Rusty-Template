import {Injectable} from "@angular/core";
import {Severity} from "../domain_value/severity";
import {Notification} from "../material/notification";
import {ObserverPattern} from "../template/class_template/observer_pattern";

@Injectable({
    providedIn: "root",
})
export class NotificationService extends ObserverPattern {
    notify(severity: Severity, message: string): void {
        super._notify(callback => callback.call(callback, new Notification(severity, message)));
    }
}

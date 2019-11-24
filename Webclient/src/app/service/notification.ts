import { Injectable } from "@angular/core";
import { Severity } from "../domain_value/severity";
import { Notification } from "../material/notification";

@Injectable({
  providedIn: "root",
 })
export class NotificationService {
  observers: any = [];

  notify(severity: Severity, message: string): void {
    this.observers.forEach(callback => callback.call(callback, new Notification(severity, message)));
  }

  subscribe(callback: any): void {
    this.observers.push(callback);
  }
}

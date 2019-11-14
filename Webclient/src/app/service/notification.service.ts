import { Injectable } from "@angular/core";
import { Severity } from "../domainvalue/severity";
import { NotificationInformation } from "../material/notification_information";

@Injectable({
  providedIn: "root",
 })
export class NotificationService {
  observers: any = [];

  notify(severity: Severity, message: string): void {
    this.observers.forEach(callback => callback.call(callback, new NotificationInformation(severity, message)));
  }

  subscribe(callback: any): void {
    this.observers.push(callback);
  }
}

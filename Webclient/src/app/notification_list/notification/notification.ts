import { Component, Input, Output, EventEmitter } from "@angular/core";
import { NotificationInformation } from "src/app/material/notification_information";

@Component({
  selector: "Notification",
  templateUrl: "./notification.html",
  styleUrls: ["./notification.scss"]
})
export class Notification {
  @Input() index: number;
  @Input() context: NotificationInformation;
  @Output() closed: EventEmitter<number> = new EventEmitter();

  close(): void {
    this.closed.emit(this.index);
  }
}

import { Component } from "@angular/core";
import { NotificationInformation } from "../../material/notification_information";
import { NotificationService } from "../../service/notification.service";

@Component({
  selector: "NotificationList",
  templateUrl: "./notification_list.html",
  styleUrls: ["./notification_list.scss"]
})
export class NotificationList {
  static TIMEOUT = 3000;
  notifications: Array<NotificationInformation> = [];

  constructor(private notificationService: NotificationService) {
    this.notificationService.subscribe(notification => this.addNotification(notification));
  }

  addNotification(notification: NotificationInformation): void  {
    this.notifications.push(notification);
    setTimeout(item => {
      const index = this.notifications.indexOf(item);
      if (index >= 0)
        this.notifications.splice(index, 1);
    }, NotificationList.TIMEOUT, notification);
  }

  closeNotification(index: number): void {
    this.notifications.splice(index, 1);
  }
}

import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { NotificationList } from "./component/notification_list/notification_list";
import { Notification } from "./component/notification/notification";

@NgModule({
  declarations: [
    NotificationList,
    Notification
  ],
  imports: [
    CommonModule,
    TranslateModule
  ],
  exports: [NotificationList]
})
export class NotificationListModule {}

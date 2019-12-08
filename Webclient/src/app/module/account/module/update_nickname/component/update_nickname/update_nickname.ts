import {Component} from "@angular/core";
import {UpdateNicknameService} from "../../service/update_nickname";
import {Severity} from "../../../../../../domain_value/severity";
import {NotificationService} from "../../../../../../service/notification";

@Component({
    selector: "UpdateNickname",
    templateUrl: "./update_nickname.html",
    styleUrls: ["./update_nickname.scss"]
})
export class UpdateNicknameComponent {
    nickname: string = '';
    disableSubmit: boolean = false;

    constructor(private updateNicknameService: UpdateNicknameService,
                private notificationService: NotificationService) {
    }

    on_submit(): void {
        this.disableSubmit = true;
        this.updateNicknameService.update(this.nickname, () => this.on_success(), () => this.on_failure());
    }

    on_success(): void {
        this.disableSubmit = false;
        this.notificationService.propagate(Severity.Success, 'serverResponses.200');
    }

    on_failure(): void {
        this.disableSubmit = false;
    }
}

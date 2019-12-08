import {Component} from "@angular/core";
import {FormFailure} from "../../../../../../material/form_failure";
import {NotificationService} from "../../../../../../service/notification";
import {Severity} from "../../../../../../domain_value/severity";
import {APIFailure} from "../../../../../../domain_value/api_failure";
import {UpdatePasswordService} from "../../service/update_password";
import {SettingsService} from "../../../../../../service/settings";

@Component({
    selector: "UpdatePassword",
    templateUrl: "./update_password.html",
    styleUrls: ["./update_password.scss"]
})
export class UpdatePasswordComponent {
    password: string = '';
    formFailure: FormFailure = FormFailure.empty();
    disableSubmit: boolean = false;

    constructor(private updatePasswordService: UpdatePasswordService,
                private notificationService: NotificationService,
                private settingsService: SettingsService) {
    }

    on_submit(): void {
        this.disableSubmit = true;
        this.updatePasswordService.update(this.password, (api_token) => this.on_success(api_token), (api_failure) => this.on_failure(api_failure));
    }

    on_success(api_token: any): void {
        this.disableSubmit = false;
        this.settingsService.set("API_TOKEN", api_token.token);
        this.notificationService.propagate(Severity.Success, 'serverResponses.200');
    }

    on_failure(api_failure: APIFailure): void {
        this.formFailure = FormFailure.from(api_failure, 523, 524);
        this.disableSubmit = false;
    }
}

import {Component} from "@angular/core";
import {FormFailure} from "../../../../../../material/form_failure";
import {NotificationService} from "../../../../../../service/notification";
import {Severity} from "../../../../../../domain_value/severity";
import {APIFailure} from "../../../../../../domain_value/api_failure";
import {UpdateMailService} from "../../service/update_mail";

@Component({
    selector: "UpdateMail",
    templateUrl: "./update_mail.html",
    styleUrls: ["./update_mail.scss"]
})
export class UpdateMailComponent {
    mail: string = '';
    formFailure: FormFailure = FormFailure.empty();
    disableSubmit: boolean = false;

    constructor(private updateMailService: UpdateMailService,
                private notificationService: NotificationService) {
    }

    on_submit(): void {
        this.disableSubmit = true;
        this.updateMailService.update(this.mail, () => this.on_success(), (api_failure) => this.on_failure(api_failure));
    }

    on_success(): void {
        this.disableSubmit = false;
        this.notificationService.propagate(Severity.Info, 'serverResponses.mail_confirm');
    }

    on_failure(api_failure: APIFailure): void {
        this.formFailure = FormFailure.from(api_failure, 521, 525, 528);
        this.disableSubmit = false;
    }
}

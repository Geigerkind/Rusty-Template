import {Component} from "@angular/core";
import {Severity} from "../../../../../../domain_value/severity";
import {NotificationService} from "../../../../../../service/notification";
import {DeleteAccountService} from "../../service/delete_account";

@Component({
    selector: "DeleteAccount",
    templateUrl: "./delete_account.html",
    styleUrls: ["./delete_account.scss"]
})
export class DeleteAccountComponent {
    disableSubmit: boolean = false;

    constructor(private deleteAccountService: DeleteAccountService,
                private notificationService: NotificationService) {
    }

    deleteAccount(): void {
        this.disableSubmit = true;
        this.deleteAccountService.delete(() => this.on_success(), () => this.on_failure());
    }

    on_success(): void {
        this.disableSubmit = false;
        this.notificationService.propagate(Severity.Success, 'serverResponses.200');
    }

    on_failure(): void {
        this.disableSubmit = false;
    }
}

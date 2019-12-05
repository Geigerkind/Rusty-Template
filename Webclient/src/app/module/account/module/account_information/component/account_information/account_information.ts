import {Component} from "@angular/core";
import {AccountInformation} from "../../../../domain_value/account_information";

@Component({
    selector: "AccountInformation",
    templateUrl: "./account_information.html",
    styleUrls: ["./account_information.scss"]
})
export class AccountInformationComponent {
    accountInformation: AccountInformation = new AccountInformation(1, "somemail@provider.com", false, "Peter Lusting");
}

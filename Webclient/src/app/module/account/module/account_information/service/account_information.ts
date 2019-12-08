import {Injectable} from "@angular/core";
import {APIService} from "../../../../../service/api";
import {AccountInformation} from "../../../domain_value/account_information";

@Injectable({
    providedIn: "root",
})
export class AccountInformationService {
    private static readonly URL_GET: string = '/account/get';

    constructor(private apiService: APIService) {
    }

    get(on_success: (AccountInformation) => void): void {
        this.apiService
            .get_auth<AccountInformation>(AccountInformationService.URL_GET, on_success);
    }
}

import {Injectable} from "@angular/core";
import {APIService} from "../../../../../service/api";

@Injectable({
    providedIn: "root",
})
export class DeleteAccountService {
    private static readonly URL_ISSUE_DELETE: string = '/account/delete';

    constructor(private apiService: APIService) {
    }

    delete(on_success: any, on_failure:any): void {
        this.apiService.delete_auth(DeleteAccountService.URL_ISSUE_DELETE, '', on_success, on_failure);
    }
}

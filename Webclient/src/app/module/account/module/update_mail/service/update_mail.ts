import {Injectable} from "@angular/core";
import {APIService} from "../../../../../service/api";

@Injectable({
    providedIn: "root",
})
export class UpdateMailService {
    private static readonly URL_UPDATE_MAIL: string = '/account/update/mail';

    constructor(private apiService: APIService) {
    }

    update(mail: string, on_success: any, on_failure: any): void {
        this.apiService.post_auth(UpdateMailService.URL_UPDATE_MAIL, mail, on_success, on_failure);
    }

}

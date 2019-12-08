import {Injectable} from "@angular/core";
import {APIService} from "../../../../../service/api";

@Injectable({
    providedIn: "root",
})
export class UpdatePasswordService {
    private static readonly URL_UPDATE_PASSWORD: string = '/account/update/password';

    constructor(private apiService: APIService) {
    }

    update(password: string, on_success: any, on_failure: any): void {
        this.apiService.post_auth(UpdatePasswordService.URL_UPDATE_PASSWORD, password, on_success, on_failure);
    }
}

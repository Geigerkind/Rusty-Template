import {Injectable} from "@angular/core";
import {APIService} from "../../../../../service/api";

@Injectable({
    providedIn: "root",
})
export class UpdateNicknameService {
    private static readonly URL_UPDATE_NICKNAME: string = '/account/update/nickname';

    constructor(private apiService: APIService) {
    }

    update(nickname: string, on_success: any, on_failure: any): void {
        this.apiService.post_auth(UpdateNicknameService.URL_UPDATE_NICKNAME, nickname, on_success, on_failure);
    }

}

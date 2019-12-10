import {Injectable} from "@angular/core";
import {APIService} from "../../../../../service/api";
import {CreateToken} from "../dto/create_token";

@Injectable({
    providedIn: "root",
})
export class APITokensService {
    private static readonly URL_GET_TOKENS: string = '/account/token/get';
    private static readonly URL_ADD_TOKEN: string = '/account/token/create';
    private static readonly URL_DELETE_TOKEN: string = '/account/token/delete';

    constructor(private apiService: APIService) {
    }

    get(on_success: any): void {
        this.apiService.get_auth(APITokensService.URL_GET_TOKENS, on_success);
    }

    add_token(create_token: CreateToken, on_success: any, on_failure: any): void {
        this.apiService.post_auth(APITokensService.URL_ADD_TOKEN, create_token, on_success, on_failure);
    }

    delete_token(token_id: number, on_success: any, on_failure: any): void {
        this.apiService.delete_auth(APITokensService.URL_DELETE_TOKEN, token_id, on_success, on_failure);
    }
}

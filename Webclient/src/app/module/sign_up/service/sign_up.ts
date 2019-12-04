import {Injectable} from "@angular/core";
import {APIService} from "../../../service/api";
import {SignUpForm} from "../dto/sign_up_form";
import {SettingsService} from "../../../service/settings";
import {Router} from "@angular/router";

@Injectable({
    providedIn: "root",
})
export class SignUpService {
    static readonly URL_SIGN_UP: string = "/account/create";

    constructor(private apiService: APIService,
                private settingsService: SettingsService,
                private routingService: Router) {
    }

    signUp(signUpForm: SignUpForm, on_response: any): void {
        this.apiService.post<any, SignUpForm>(SignUpService.URL_SIGN_UP, signUpForm, (resp) => {
            this.settingsService.set("API_TOKEN", resp.token, 30);
            this.routingService.navigate(["/account"]);
            on_response.call(on_response);
        }, on_response);
    }
}

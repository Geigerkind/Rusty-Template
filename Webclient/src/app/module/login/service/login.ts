import {Injectable} from "@angular/core";
import {APIService} from "../../../service/api";
import {LoginForm} from "../dto/login_form";
import {SettingsService} from "../../../service/settings";
import {Router} from "@angular/router";

@Injectable({
    providedIn: "root",
})
export class LoginService {
    static readonly URL_LOGIN: string = "/account/login";

    constructor(private apiService: APIService,
                private settingsService: SettingsService,
                private routingService: Router) {
    }

    signIn(loginForm: LoginForm): void {
        this.apiService.post<any, LoginForm>(LoginService.URL_LOGIN, (resp) => {
            this.settingsService.set("API_TOKEN", resp.token, 30);
            this.routingService.navigate(["/account"]);
        }, loginForm);
    }
}

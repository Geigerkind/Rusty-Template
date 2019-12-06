import {Component} from "@angular/core";
import {LoginService} from "../../service/login";
import {LoginForm} from "../../dto/login_form";

@Component({
    selector: "Login",
    templateUrl: "./login.html",
    styleUrls: ["./login.scss"]
})
export class LoginComponent {
    invalidateMail = false;
    invalidatePassword = false;
    disableSubmit = false;
    model: LoginForm = {
        mail: "",
        password: ""
    };

    constructor(private loginService: LoginService) {
    }

    onSubmit(): void {
        if (!this.disableSubmit) {
            this.disableSubmit = true;
            this.loginService.signIn(this.model, this.on_success(), callback => this.on_failure(callback));
        }
    }

    private on_success(): void {
        this.disableSubmit = false;
    }

    private on_failure(reason): void {
        if (reason.status === 599) {
            this.invalidateMail = true;
            this.invalidatePassword = true;
        }
        this.disableSubmit = false;
    }
}

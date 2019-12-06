import {Component} from "@angular/core";
import {LoginService} from "../../service/login";
import {LoginForm} from "../../dto/login_form";

@Component({
    selector: "Login",
    templateUrl: "./login.html",
    styleUrls: ["./login.scss"]
})
export class LoginComponent {
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
            this.loginService.signIn(this.model, this.on_success, this.on_failure);
        }
    }

    private on_success(): void {
        this.disableSubmit = false;
    }

    private on_failure(reason): void {
        console.log(reason);
        this.disableSubmit = false;
    }
}

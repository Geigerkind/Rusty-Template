import {Component} from "@angular/core";
import {LoginService} from "../../service/login";
import {LoginForm} from "../../dto/login_form";

@Component({
    selector: "Login",
    templateUrl: "./login.html",
    styleUrls: ["./login.scss"]
})
export class LoginComponent {
    model: LoginForm = {
        mail: "",
        password: ""
    };

    constructor(private loginService: LoginService) {
    }

    onSubmit(): void {
        this.loginService.signIn(this.model);
    }
}

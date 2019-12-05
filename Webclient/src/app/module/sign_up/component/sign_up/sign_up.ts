import {Component} from "@angular/core";
import {SignUpService} from "../../service/sign_up";
import {SignUpForm} from "../../dto/sign_up_form";

@Component({
    selector: "SignUp",
    templateUrl: "./sign_up.html",
    styleUrls: ["./sign_up.scss"]
})
export class SignUpComponent {
    disableSubmit = false;
    model: SignUpForm = {
        nickname: "",
        credentials: {
            mail: "",
            password: ""
        }
    };

    constructor(private signUpService: SignUpService) {
    }

    onSubmit(): void {
        if (!this.disableSubmit) {
            this.disableSubmit = true;
            this.signUpService.signUp(this.model, () => this.disableSubmit = false);
        }
    }
}

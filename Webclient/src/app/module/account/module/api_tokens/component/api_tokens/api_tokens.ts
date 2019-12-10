import {Component} from "@angular/core";
import {FormFailure} from "../../../../../../material/form_failure";
import {APITokensService} from "../../service/api_tokens";
import {APIToken} from "../../../../domain_value/api_token";
import {CreateToken} from "../../dto/create_token";
import {APIFailure} from "../../../../../../domain_value/api_failure";
import {NotificationService} from "../../../../../../service/notification";
import {Severity} from "../../../../../../domain_value/severity";

@Component({
    selector: "APITokens",
    templateUrl: "./api_tokens.html",
    styleUrls: ["./api_tokens.scss"]
})
export class APITokensComponent {
    disabledSubmit: boolean = false;
    formFailureDate: FormFailure = FormFailure.empty();
    formFailurePurpose: FormFailure = FormFailure.empty();
    purpose: string = '';
    exp_date: Date = new Date(new Date().getTime() + 24 * 60 * 60 * 1000);
    tokenList: Array<[APIToken, boolean]> = [];

    constructor(private apiTokensService: APITokensService,
                private notificationService: NotificationService) {
        this.get_tokens();
    }

    on_submit(): void {
        if (this.disabledSubmit)
            return;
        this.disabledSubmit = true;
        const create_token: CreateToken = {
            purpose: this.purpose,
            exp_date: Math.floor(this.exp_date.getTime() / 1000)
        };

        this.apiTokensService.add_token(create_token, (api_token) => this.add_token_success(api_token),
            (api_failure) => this.add_token_failure(api_failure));
    }

    delete_token(token_pair: [APIToken, boolean]) {
        token_pair[1] = true;
        this.apiTokensService.delete_token(token_pair[0].id, () => {
            this.notificationService.propagate(Severity.Success, "serverResponses.200");
            this.get_tokens();
            token_pair[1] = false;
        }, () => token_pair[1] = false);
    }

    private get_tokens(): void {
        this.apiTokensService.get((tokens) => this.retrieve_tokens(tokens));
    }

    private add_token_success(api_token: APIToken): void {
        this.tokenList.push([api_token, false]);
        this.notificationService.propagate(Severity.Success, "serverResponses.200");
        this.disabledSubmit = false;
    }

    private add_token_failure(api_failure: APIFailure): void {
        this.disabledSubmit = false;
        this.formFailureDate = FormFailure.from(api_failure, 531, 532);
        this.formFailurePurpose = FormFailure.from(api_failure, 533);
    }

    private retrieve_tokens(api_tokens: Array<APIToken>): void {
        this.tokenList = api_tokens.map(token => [token, false]);
    }
}

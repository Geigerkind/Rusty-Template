import {Component} from "@angular/core";

@Component({
    selector: "APITokens",
    templateUrl: "./api_tokens.html",
    styleUrls: ["./api_tokens.scss"]
})
export class APITokensComponent {
    private tokenList: Array<Array<string>> = [
        ["api token str", "purpose 1"],
        ["api token str", "purpose 2"],
        ["api token str", "purpose 3"]
    ];
}

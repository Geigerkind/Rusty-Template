import {Injectable} from "@angular/core";
import {HttpClient, HttpErrorResponse, HttpHeaders} from "@angular/common/http";
import {SettingsService} from "./settings";
import {NotificationService} from "./notification";
import {Severity} from "../domain_value/severity";
import {Router} from "@angular/router";
import {LoadingBarService} from "./loading_bar";

@Injectable({
    providedIn: "root",
})
export class APIService {
    static readonly API_PREFIX: string = "/API";

    constructor(private httpClient: HttpClient,
                private settingsService: SettingsService,
                private notificationService: NotificationService,
                private routingService: Router,
                private loadingBarService: LoadingBarService) {
    }

    httpHeaderFactory(): HttpHeaders {
        return new HttpHeaders()
            .set("Content-Type", "application/json");
    }

    setAuthHeader(headers: HttpHeaders): HttpHeaders {
        let api_token = "";
        if (this.settingsService.check("API_TOKEN"))
            api_token = this.settingsService.get("API_TOKEN");
        return headers.set("Authentication", api_token);
    }

    handleFailure(reason: HttpErrorResponse): void {
        if (reason.status <= 520)
            return;

        // Token invalid
        if (reason.status === 401) {
            this.settingsService.set("API_TOKEN", undefined, -1);
            this.routingService.navigate(["/login"]);
            return;
        }

        this.notificationService.notify(Severity.Error, reason.error);
    }


    get_auth<T>(url: string, on_success: (T) => void): void {
        this.loadingBarService.incrementCounter();
        this.httpClient.get<T>(APIService.API_PREFIX + url, {headers: this.setAuthHeader(this.httpHeaderFactory())})
            .toPromise()
            .then(response => on_success.call(on_success, response))
            .catch(reason => this.handleFailure(reason))
            .finally(() => this.loadingBarService.decrementCounter());
    }

    get<T>(url: string, on_success: (T) => void): void {
        this.loadingBarService.incrementCounter();
        this.httpClient.get<T>(APIService.API_PREFIX + url, {headers: this.httpHeaderFactory()})
            .toPromise()
            .then(response => on_success.call(on_success, response))
            .catch(reason => this.handleFailure(reason))
            .finally(() => this.loadingBarService.decrementCounter());
    }

    post<T1, T2>(url: string, on_success: (T1) => void, body: T2): void {
        this.loadingBarService.incrementCounter();
        this.httpClient.post<T1>(APIService.API_PREFIX + url, JSON.stringify(body), {headers: this.httpHeaderFactory()})
            .toPromise()
            .then(response => on_success.call(on_success, response))
            .catch(reason => this.handleFailure(reason))
            .finally(() => this.loadingBarService.decrementCounter());
    }
}

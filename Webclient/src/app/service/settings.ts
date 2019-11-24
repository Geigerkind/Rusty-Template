import {CookieService} from "ngx-cookie-service";
import {Injectable} from "@angular/core";

@Injectable({
  providedIn: "root",
})
export class SettingsService {
  settings: Array<string> = [
    "cookieDecisions",
    "PWA_PROMPT"
  ];

  observers: any = {};

  constructor(private cookieService: CookieService) {
    for (const setting in cookieService.getAll()) {
      if (!this.settings.includes(setting))
        continue;

      this[setting] = JSON.parse(this.cookieService.get(setting));
    }
  }

  set(cookieName: string, value: any, days: number): void {
    if (!this.settings.includes(cookieName))
      throw new Error("Cookie: " + cookieName + " was not predefined!");

    this.cookieService.set(cookieName, JSON.stringify(value), days);
    this[cookieName] = value;

    // Inform observers
    if (this.observers[cookieName]) {
      this.observers[cookieName].forEach(callback => callback.call(callback, this[cookieName]));
    }
  }

  get(cookieName: string): any {
    return this[cookieName];
  }

  check(cookieName: string): boolean {
    return !!this[cookieName];
  }

  subscribe(cookieName: string, callback: any): void {
    if (!this.observers[cookieName]) {
      this.observers[cookieName] = [];
    }
    this.observers[cookieName].push(callback);
  }

}

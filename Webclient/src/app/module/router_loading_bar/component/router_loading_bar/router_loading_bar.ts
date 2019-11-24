import { Component } from "@angular/core";
import { Router, NavigationStart, NavigationEnd, NavigationCancel, NavigationError } from "@angular/router";

@Component({
  selector: "RouterLoadingBar",
  templateUrl: "./router_loading_bar.html",
  styleUrls: ["./router_loading_bar.scss"]
})
export class RouterLoadingBarComponent {
  scaleX = 0;
  displayBar = false;

  constructor(private router: Router) {
    this.router.events.subscribe(event => {
      switch (true) {
        case event instanceof NavigationStart: {
          this.scaleX = 0;
          this.displayBar = true;

          const max_iterations = 20;
          for (let i = 0; i < max_iterations; ++i) {
            const scaleFactor = (0.8 / max_iterations) * i;
            const afterTime = (1000 / max_iterations) * i;
            setTimeout(() => { if (this.scaleX < scaleFactor) this.scaleX = scaleFactor; }, afterTime);
          }
          break;
        }
        case event instanceof NavigationEnd:
        case event instanceof NavigationCancel:
        case event instanceof NavigationError: {
          this.scaleX = 1;
          setTimeout(() => this.displayBar = false, 100);
          break;
        }
        default:
          break;
      }
    });
  }
}

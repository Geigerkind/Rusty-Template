import { NgModule } from "@angular/core";
import { CommonModule } from "@angular/common";
import { RouterLoadingBar } from "./component/router_loading_bar/router_loading_bar";

@NgModule({
  declarations: [RouterLoadingBar],
  imports: [CommonModule],
  exports: [RouterLoadingBar]
})
export class RouterLoadingBarModule {}

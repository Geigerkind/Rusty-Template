import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { HomeComponent } from "./home";
import { HomeRoutingModule } from "./routing.module";

@NgModule({
  declarations: [HomeComponent],
  imports: [
    CommonModule,
    TranslateModule,
    HomeRoutingModule
  ],
  exports: [HomeComponent]
})
export class HomeModule { }

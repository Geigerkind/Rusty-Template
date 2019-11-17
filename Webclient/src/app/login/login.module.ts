import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { LoginRoutingModule } from "./routing.module";
import { Login } from "./login";

@NgModule({
  declarations: [Login],
  imports: [
    CommonModule,
    TranslateModule,
    LoginRoutingModule
  ],
  exports: [Login]
})
export class LoginModule { }

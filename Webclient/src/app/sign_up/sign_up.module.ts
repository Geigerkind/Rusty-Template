import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { SignUpRoutingModule } from "./routing.module";
import { SignUp } from "./sign_up";

@NgModule({
  declarations: [SignUp],
  imports: [
    CommonModule,
    TranslateModule,
    SignUpRoutingModule
  ],
  exports: [SignUp]
})
export class SignUpModule { }

import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { Home } from './home';
import { HomeRoutingModule } from './routing.module';

@NgModule({
  declarations: [Home],
  imports: [
    CommonModule,
    TranslateModule,
    HomeRoutingModule
  ],
  exports: [Home]
})
export class HomeModule { }

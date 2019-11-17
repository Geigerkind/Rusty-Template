import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { BriefNote } from "./brief_note";

@NgModule({
  declarations: [BriefNote],
  imports: [
    CommonModule,
    TranslateModule
  ],
  exports: [BriefNote]
})
export class BriefNoteModule { }

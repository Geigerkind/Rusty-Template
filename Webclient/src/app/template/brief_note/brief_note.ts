import { Component, Input } from "@angular/core";

@Component({
  selector: "BriefNote",
  templateUrl: "./brief_note.html",
  styleUrls: ["./brief_note.scss"]
})
export class BriefNote {
  @Input() noteKey: string;
}

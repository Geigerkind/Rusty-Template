import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";
import { Imprint } from "./imprint";

const routes: Routes = [
  { path: "", component: Imprint }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class ImprintRoutingModule { }

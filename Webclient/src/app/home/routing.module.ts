import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";
import { Home } from './home';

const routes: Routes = [
  { path: "", component: Home }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class HomeRoutingModule { }

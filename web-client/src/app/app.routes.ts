import { Routes } from '@angular/router';
import { LoginPage } from './layout/page/login-page/login-page';
import { HomePage } from './layout/page/home-page/home-page';

export const routes: Routes = [
  {
    path: "",
    component: HomePage
  },
  {
    path: "login",
    component: LoginPage
  }
];

import { Routes } from '@angular/router';
import { LoginPage } from './layout/page/login-page/login-page';
import { HomePage } from './layout/page/home-page/home-page';
import { GroupPage } from './layout/page/group-page/group-page';
import { authGuard } from './guards/auth-guard';

export const routes: Routes = [
  {
    path: '',
    component: HomePage,
  },
  {
    path: 'login',
    component: LoginPage,
  },
  {
    path: 'groups/:groupId',
    component: GroupPage,
    canActivate: [authGuard],
  },
];

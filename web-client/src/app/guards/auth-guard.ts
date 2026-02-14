import { inject } from '@angular/core';
import { CanActivateFn } from '@angular/router';
import { AuthService } from '../core/auth/auth.service';
import { AuthStatus } from '../core/auth/auth.models';

export const authGuard: CanActivateFn = () => {
  const authService = inject(AuthService);
  return authService.authState().status === AuthStatus.Authenticated;
};

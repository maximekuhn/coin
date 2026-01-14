import {
  ApplicationConfig,
  inject,
  provideAppInitializer,
  provideBrowserGlobalErrorListeners,
} from '@angular/core';
import { provideRouter } from '@angular/router';

import { routes } from './app.routes';
import { provideHttpClient, withFetch, withInterceptors } from '@angular/common/http';
import { withCredentialsInterceptor } from './core/http/interceptors/with-credentials-interceptor';
import { AuthService } from './core/auth/auth.service';
import { AuthError, AuthErrorKind } from './core/auth/auth.errors';
import { AuthStatus } from './core/auth/auth.models';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideRouter(routes),
    provideHttpClient(withFetch(), withInterceptors([withCredentialsInterceptor])),
    provideAppInitializer(() => {
      const authService = inject(AuthService);
      authService.fetchSession().subscribe({
        error: (err: AuthError) => {
          if (
            err.kind == AuthErrorKind.Unknown &&
            authService.authState().status === AuthStatus.Guest
          ) {
            return;
          }
          console.error('failed to initialize app: could not retrive session');
        },
      });
    }),
  ],
};

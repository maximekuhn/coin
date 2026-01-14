import { HttpErrorResponse } from '@angular/common/http';
import { AuthError, AuthErrorKind } from './auth.errors';

/* eslint-disable-next-line @typescript-eslint/no-unused-vars */
export function mapToAuthError(_: HttpErrorResponse): AuthError {
  // TODO: implement mapToAuthError
  return { kind: AuthErrorKind.Unknown };
}

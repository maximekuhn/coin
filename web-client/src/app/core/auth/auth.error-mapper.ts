import { HttpErrorResponse } from "@angular/common/http";
import { AuthError, AuthErrorKind } from "./auth.errors";

export function mapToAuthError(_: HttpErrorResponse): AuthError {
  // TODO: implement mapToAuthError
  return { kind: AuthErrorKind.Unknown };
}

import { AuthError, AuthErrorKind } from './auth.errors';

export function translateAuthError(err: AuthError): string {
  switch (err.kind) {
    case AuthErrorKind.BadCredentials:
      return $localize`:Authentication error when user provided bad credentials@@auth.error.bad-credentials:Bad credentials. Please check your email and/or password and try again.`;

    case AuthErrorKind.Unknown:
    default:
      return $localize`:@@auth.error.unknown:Unknown authentication error.`;
  }
}

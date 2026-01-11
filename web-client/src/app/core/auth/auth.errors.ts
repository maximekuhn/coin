export enum AuthErrorKind {
  BadCredentials,
  Unknown,
}

export interface AuthError {
  kind: AuthErrorKind;
}

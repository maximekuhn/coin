export interface User {
  id: string;
  name: string;
}

export enum AuthStatus {
  Unknown = "unknown",
  Guest = "guest",
  Authenticated = "authenticated",
}

export type AuthState =
  | { status: AuthStatus.Unknown }
  | { status: AuthStatus.Guest }
  | { status: AuthStatus.Authenticated, user: User };


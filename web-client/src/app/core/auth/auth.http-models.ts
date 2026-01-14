import { User } from "./auth.models";

export interface BackendSessionInfo {
  user: {
    id: string;
    name: string;
  };
  activeSessionsCount: number;
}

export function backendSessionInfoToUser(bsi: BackendSessionInfo): User {
  return {
    id: bsi.user.id,
    name: bsi.user.name
  };
}

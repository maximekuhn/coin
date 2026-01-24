export interface GroupOverview {
  id: string;
  name: string;
  owner: UserOverview;
  createdAt: Date;
  lastActivity?: Date;
  currentUserBalanceEuros?: number;
}

export interface UserOverview {
  id: string;
  name: string;
}

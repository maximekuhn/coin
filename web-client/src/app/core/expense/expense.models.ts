export interface ExpenseOverview {
  id: string;
  group: GroupOverview;
  occurredAt: Date;
  totalEuros: number;
  paidBy: UserOverview;
}

export interface UserOverview {
  id: string;
  name: string;
}

export interface GroupOverview {
  id: string;
  name: string;
}

import { ExpenseOverview } from './expense.models';

export interface BackendCreateExpenseResponse {
  expenseId: string;
}

export interface BackendExpenseOverview {
  id: string;
  group: BackendGroupOverview;
  occurredAt: Date;
  totalEuros: number;
  paidBy: BackendUserOverview;
}

export interface BackendUserOverview {
  id: string;
  name: string;
}

export interface BackendGroupOverview {
  id: string;
  name: string;
}

export function backendExpenseOverviewToExpenseOverview(
  b: BackendExpenseOverview,
): ExpenseOverview {
  return {
    id: b.id,
    group: {
      id: b.group.id,
      name: b.group.name,
    },
    occurredAt: b.occurredAt,
    totalEuros: b.totalEuros,
    paidBy: {
      id: b.paidBy.id,
      name: b.paidBy.name,
    },
  };
}

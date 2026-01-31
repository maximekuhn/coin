import { ExpenseError } from '../expense.error';

export enum CreateExpenseFormResultType {
  Success,
  Error,
}

export type CreateExpenseFormDialogResult =
  | { status: CreateExpenseFormResultType.Success }
  | { status: CreateExpenseFormResultType.Error; error: ExpenseError };

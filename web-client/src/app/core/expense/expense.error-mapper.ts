import { HttpErrorResponse } from '@angular/common/http';
import { ExpenseError, ExpenseErrorKind } from './expense.error';

export function mapToExpenseError(_: HttpErrorResponse): ExpenseError {
  // TODO: correct implementation
  return { kind: ExpenseErrorKind.Unknown };
}

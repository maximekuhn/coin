import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { catchError, map, Observable, throwError } from 'rxjs';
import {
  BackendCreateExpenseResponse,
  BackendExpenseOverview,
  backendExpenseOverviewToExpenseOverview,
} from './expense.http-models';
import { environment } from '../../../environments/environment';
import { mapToExpenseError } from './expense.error-mapper';
import { ListResponse } from '../../shared/api/list-response';
import { ExpenseOverview } from './expense.models';

@Injectable({
  providedIn: 'root',
})
export class ExpenseService {
  private http = inject(HttpClient);

  createExpense(
    groupId: string,
    totalEuros: number,
    occurredAt: Date,
    payerId: string,
  ): Observable<{ expenseId: string }> {
    return this.http
      .post<BackendCreateExpenseResponse>(`${environment.API_URL}/api/groups/${groupId}/expenses`, {
        totalEuros,
        occurredAt,
        payerId,
      })
      .pipe(
        map((res: BackendCreateExpenseResponse) => {
          return { expenseId: res.expenseId };
        }),
        catchError((err) => throwError(() => mapToExpenseError(err))),
      );
  }

  getLatestExpense(): Observable<{ expenses: ExpenseOverview[]; hasMore: boolean }> {
    return this.http
      .get<
        ListResponse<BackendExpenseOverview>
      >(`${environment.API_URL}/api/expenses/latest?page=1&pageSize=4`)
      .pipe(
        map((res: ListResponse<BackendExpenseOverview>) => {
          const expenses = res.data.map(backendExpenseOverviewToExpenseOverview);
          return { expenses, hasMore: res.totalItems > res.data.length };
        }),
        catchError((err) => throwError(() => mapToExpenseError(err))),
      );
  }
}

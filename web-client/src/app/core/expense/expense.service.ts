import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { catchError, map, Observable, throwError } from 'rxjs';
import { BackendCreateExpenseResponse } from './expense.http-models';
import { environment } from '../../../environments/environment';
import { mapToExpenseError } from './expense.error-mapper';

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
}

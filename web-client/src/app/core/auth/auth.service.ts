import { HttpClient, HttpErrorResponse } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { catchError, Observable, throwError } from 'rxjs';
import { environment } from '../../../environments/environment';
import { mapToAuthError } from './auth.error-mapper';

@Injectable({
  providedIn: 'root',
})
export class AuthService {
  private http = inject(HttpClient);

  login(email: string, password: string): Observable<void> {
    return this.http.post<void>(`${environment.API_URL}/api/auth/login`, { email, password }).pipe(
      catchError((err: HttpErrorResponse) => throwError(() => mapToAuthError(err)))
    );
  }
}

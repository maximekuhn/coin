import { HttpClient, HttpErrorResponse } from '@angular/common/http';
import { computed, inject, Injectable, signal } from '@angular/core';
import { catchError, map, Observable, switchMap, tap, throwError } from 'rxjs';
import { environment } from '../../../environments/environment';
import { mapToAuthError } from './auth.error-mapper';
import { BackendSessionInfo, backendSessionInfoToUser } from './auth.http-models';
import { AuthState, AuthStatus, User } from './auth.models';

@Injectable({
  providedIn: 'root',
})
export class AuthService {
  private http = inject(HttpClient);

  private _authState = signal<AuthState>(({ status: AuthStatus.Unknown }));
  readonly authState = this._authState.asReadonly();

  readonly user = computed<User | null>(() => {
    const state = this._authState();
    return state.status === AuthStatus.Authenticated ? state.user : null;
  });


  login(email: string, password: string): Observable<User> {
    return this.http.post<void>(`${environment.API_URL}/api/auth/login`, { email, password }).pipe(
      switchMap(() => this.fetchSession()),
      catchError((err: HttpErrorResponse) => throwError(() => mapToAuthError(err)))
    );
  }

  fetchSession(): Observable<User> {
    return this.http.post<BackendSessionInfo>(`${environment.API_URL}/api/auth/session-info`, {}).pipe(
      map((res: BackendSessionInfo) => backendSessionInfoToUser(res)),
      tap((user: User) => {
        this._authState.set({ status: AuthStatus.Authenticated, user });
      }),
      catchError((err: HttpErrorResponse) => {
        if (err.status === 401) {
          this._authState.set({ status: AuthStatus.Guest });
        }
        return throwError(() => mapToAuthError(err));
      }
      )
    );
  }

  logout(): Observable<void> {
    return this.http.post<void>(`${environment.API_URL}/api/auth/logout`, {}).pipe(
      tap(() => {
        this._authState.set({ status: AuthStatus.Guest });
      }),
      catchError((err: HttpErrorResponse) => {
        this._authState.set({ status: AuthStatus.Guest });
        return throwError(() => mapToAuthError(err));
      })
    );
  }
}

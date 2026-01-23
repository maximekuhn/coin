import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { catchError, map, Observable, throwError } from 'rxjs';
import { CreateGroupResponse } from './group.http-models';
import { environment } from '../../../environments/environment';
import { mapToGroupError } from './group.error-mapper';

@Injectable({
  providedIn: 'root',
})
export class GroupService {
  private http = inject(HttpClient);

  createGroup(name: string): Observable<{ groupId: string }> {
    return this.http.post<CreateGroupResponse>(`${environment.API_URL}/api/groups`, { name }).pipe(
      map((res: CreateGroupResponse) => {
        return { groupId: res.groupId };
      }),
      catchError((err) => throwError(() => mapToGroupError(err))),
    );
  }
}

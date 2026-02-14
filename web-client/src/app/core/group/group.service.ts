import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { catchError, map, Observable, throwError } from 'rxjs';
import {
  BackendCreateGroupResponse,
  BackendGroup,
  BackendGroupOverview,
  backendGroupOverviewListToGroupOverviewList,
  backendGroupToGroup,
} from './group.http-models';
import { environment } from '../../../environments/environment';
import { mapToGroupError } from './group.error-mapper';
import { Group, GroupOverview } from './group.models';
import { ListResponse } from '../../shared/api/list-response';

@Injectable({
  providedIn: 'root',
})
export class GroupService {
  private http = inject(HttpClient);

  createGroup(name: string): Observable<{ groupId: string }> {
    return this.http
      .post<BackendCreateGroupResponse>(`${environment.API_URL}/api/groups`, { name })
      .pipe(
        map((res: BackendCreateGroupResponse) => {
          return { groupId: res.groupId };
        }),
        catchError((err) => throwError(() => mapToGroupError(err))),
      );
  }

  getGroupsOverview(): Observable<{ groups: GroupOverview[]; hasMore: boolean }> {
    return this.http
      .get<
        ListResponse<BackendGroupOverview>
      >(`${environment.API_URL}/api/groups/overview?page=1&pageSize=20`, {})
      .pipe(
        map((res: ListResponse<BackendGroupOverview>) =>
          backendGroupOverviewListToGroupOverviewList(res),
        ),
        catchError((err) => throwError(() => mapToGroupError(err))),
      );
  }

  getGroupById(groupId: string): Observable<Group> {
    return this.http.get<BackendGroup>(`${environment.API_URL}/api/groups/${groupId}`, {}).pipe(
      map((res: BackendGroup) => backendGroupToGroup(res)),
      catchError((err) => {
        return throwError(() => mapToGroupError(err));
      }),
    );
  }
}

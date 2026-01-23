import { HttpErrorResponse } from '@angular/common/http';
import { GroupError, GroupErrorKind } from './group.errors';

export function mapToGroupError(err: HttpErrorResponse): GroupError {
  if (err.status === 409) {
    return { kind: GroupErrorKind.GroupNameNotAvailable };
  }
  return { kind: GroupErrorKind.Unknown };
}

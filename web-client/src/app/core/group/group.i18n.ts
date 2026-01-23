import { GroupError, GroupErrorKind } from './group.errors';

export function translateGroupError(err: GroupError): string {
  switch (err.kind) {
    case GroupErrorKind.GroupNameNotAvailable:
      return $localize`:Error returned to the user when he tries to create a group with a name that is not available@@group.error.name-not-available:You already own a group with the same name. Please pick another one and try again.`;

    case GroupErrorKind.Unknown:
    default:
      return $localize`:@@group.error.unknown:Unknown group related error.`;
  }
}

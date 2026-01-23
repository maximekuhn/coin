export enum GroupErrorKind {
  GroupNameNotAvailable,
  Unknown,
}

export interface GroupError {
  kind: GroupErrorKind;
}

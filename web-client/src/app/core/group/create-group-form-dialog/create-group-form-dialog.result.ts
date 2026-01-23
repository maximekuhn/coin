import { GroupError } from '../group.errors';

export enum CreateGroupFormResultType {
  Success,
  Error,
}
export type CreateGroupFormDialogResult =
  | { status: CreateGroupFormResultType.Success }
  | { status: CreateGroupFormResultType.Error; error: GroupError };

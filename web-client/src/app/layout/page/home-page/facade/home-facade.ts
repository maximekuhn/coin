import { DestroyRef, inject, Injectable, signal } from '@angular/core';
import { GroupService } from '../../../../core/group/group.service';
import { GroupOverview } from '../../../../core/group/group.models';
import { GroupError } from '../../../../core/group/group.errors';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';

@Injectable()
export class HomeFacade {
  private destroyRef = inject(DestroyRef);

  private groupService = inject(GroupService);

  groupsLoading = signal<boolean>(false);
  groups = signal<{ groups: GroupOverview[]; hasMore: boolean }>({ groups: [], hasMore: false });
  groupsError = signal<GroupError | null>(null);

  loadGroups() {
    if (this.groupsLoading()) {
      return;
    }

    this.groupsError.set(null);
    this.groupsLoading.set(true);

    this.groupService
      .getGroupsOverview()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: (res: { groups: GroupOverview[]; hasMore: boolean }) => {
          this.groups.set(res);
        },
        error: (err: GroupError) => {
          this.groupsError.set(err);
          this.groupsLoading.set(false);
        },
        complete: () => {
          this.groupsLoading.set(false);
        },
      });
  }
}

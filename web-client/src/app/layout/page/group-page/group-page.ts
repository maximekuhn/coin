import { Component, DestroyRef, inject, OnInit, signal } from '@angular/core';
import { GroupService } from '../../../core/group/group.service';
import { Group } from '../../../core/group/group.models';
import { GroupError } from '../../../core/group/group.errors';
import { ActivatedRoute } from '@angular/router';
import { takeUntilDestroyed, toSignal } from '@angular/core/rxjs-interop';
import { map } from 'rxjs';
import { SnackbarService } from '../../../shared/ui/snackbar/snackbar.service';
import {
  SnackbarAction,
  SnackbarDuration,
  SnackbarType,
} from '../../../shared/ui/snackbar/snackbar.types';
import { translateGroupError } from '../../../core/group/group.i18n';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';

@Component({
  selector: 'app-group-page',
  imports: [MatProgressSpinnerModule],
  templateUrl: './group-page.html',
  styleUrl: './group-page.scss',
})
export class GroupPage implements OnInit {
  private destroyRef = inject(DestroyRef);

  private groupService = inject(GroupService);
  private snackbarService = inject(SnackbarService);

  private route = inject(ActivatedRoute);
  readonly groupId = toSignal(this.route.paramMap.pipe(map((params) => params.get('groupId')!)));

  isLoading = signal<boolean>(false);
  group = signal<Group | null>(null);

  ngOnInit(): void {
    this.loadGroup();
  }

  loadGroup() {
    if (this.isLoading()) {
      return;
    }

    this.isLoading.set(true);

    this.groupService
      .getGroupById(this.groupId()!)
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: (group: Group) => {
          this.group.set(group);
        },
        error: (err: GroupError) => {
          this.snackbarService.open(
            SnackbarType.Error,
            SnackbarDuration.Short,
            translateGroupError(err),
            SnackbarAction.Dismiss,
          );
          this.isLoading.set(false);
        },
        complete: () => {
          this.isLoading.set(false);
        },
      });
  }
}

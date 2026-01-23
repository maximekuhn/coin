import { Component, DestroyRef, inject } from '@angular/core';
import { AuthService } from '../../../core/auth/auth.service';
import { CreateGroupFormDialog } from '../../../core/group/create-group-form-dialog/create-group-form-dialog';
import { MatButtonModule } from '@angular/material/button';
import { MatDialog, MatDialogModule } from '@angular/material/dialog';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import {
  CreateGroupFormDialogResult,
  CreateGroupFormResultType,
} from '../../../core/group/create-group-form-dialog/create-group-form-dialog.result';
import { SnackbarService } from '../../../shared/ui/snackbar/snackbar.service';
import {
  SnackbarAction,
  SnackbarDuration,
  SnackbarType,
} from '../../../shared/ui/snackbar/snackbar.types';
import { translateGroupError } from '../../../core/group/group.i18n';

@Component({
  selector: 'app-home-page',
  imports: [MatButtonModule, MatDialogModule],
  templateUrl: './home-page.html',
  styleUrl: './home-page.scss',
})
export class HomePage {
  private authService = inject(AuthService);
  private snackbarService = inject(SnackbarService);

  private dialog = inject(MatDialog);

  private destroyRef = inject(DestroyRef);

  readonly user = this.authService.user;

  openCreateGroupDialog() {
    const dialogRef = this.dialog.open(CreateGroupFormDialog);

    dialogRef
      .afterClosed()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe((result: CreateGroupFormDialogResult | undefined) => {
        if (result === undefined) {
          return;
        }
        if (result.status === CreateGroupFormResultType.Success) {
          this.snackbarService.open(
            SnackbarType.Success,
            SnackbarDuration.Short,
            $localize`:Message shown to the user when he successfully created a group@@group.creation.success:Group created successfully.`,
            SnackbarAction.Ok,
          );
        } else {
          this.snackbarService.open(
            SnackbarType.Error,
            SnackbarDuration.Short,
            translateGroupError(result.error),
            SnackbarAction.Dismiss,
          );
        }
      });
  }
}

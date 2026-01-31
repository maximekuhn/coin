import { Component, DestroyRef, effect, inject, OnInit } from '@angular/core';
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
import { HomeFacade } from './facade/home-facade';
import { GroupOverviewList } from './group-overview-list/group-overview-list';
import { AuthStatus } from '../../../core/auth/auth.models';
import { MatIconModule } from '@angular/material/icon';
import { OverallBalance } from './overall-balance/overall-balance';
import { LatestExpenses } from './latest-expenses/latest-expenses';
import { MatDividerModule } from '@angular/material/divider';
import { CreateExpenseFormDialog } from '../../../core/expense/create-expense-form-dialog/create-expense-form-dialog';
import {
  CreateExpenseFormDialogResult,
  CreateExpenseFormResultType,
} from '../../../core/expense/create-expense-form-dialog/create-expense-form-dialog.result';
import { translateExpenseError } from '../../../core/expense/expense.i18n';

@Component({
  selector: 'app-home-page',
  imports: [
    MatButtonModule,
    MatDialogModule,
    GroupOverviewList,
    MatIconModule,
    OverallBalance,
    LatestExpenses,
    MatDividerModule,
  ],
  templateUrl: './home-page.html',
  styleUrl: './home-page.scss',
  providers: [HomeFacade],
})
export class HomePage implements OnInit {
  private authService = inject(AuthService);
  private snackbarService = inject(SnackbarService);

  private homeFacade = inject(HomeFacade);

  private dialog = inject(MatDialog);

  private destroyRef = inject(DestroyRef);

  readonly user = this.authService.user;

  private readonly _groupsErrorEffect = effect(() => {
    const groupsError = this.homeFacade.groupsError();
    if (groupsError) {
      this.snackbarService.open(
        SnackbarType.Error,
        SnackbarDuration.Short,
        $localize`:Message shown to the user when the application failed to load groups overview from the home page@@group.overview.load.error:Failed to load groups. Please try again in a few minutes.`,
        SnackbarAction.Dismiss,
      );
    }
  });

  ngOnInit(): void {
    if (this.authService.authState().status === AuthStatus.Authenticated) {
      this.homeFacade.loadGroups();
    }
  }

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
          this.homeFacade.loadGroups();
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

  openCreateExpenseDialog() {
    const dialogRef = this.dialog.open(CreateExpenseFormDialog);

    dialogRef
      .afterClosed()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe((result: CreateExpenseFormDialogResult | undefined) => {
        if (result === undefined) {
          return;
        }
        if (result.status === CreateExpenseFormResultType.Success) {
          this.snackbarService.open(
            SnackbarType.Success,
            SnackbarDuration.Short,
            $localize`:Message shown to the user when he successfully created a new expense@@expense.creation.success:Expense created successfully.`,
            SnackbarAction.Ok,
          );
          this.homeFacade.loadGroups();
        } else {
          this.snackbarService.open(
            SnackbarType.Error,
            SnackbarDuration.Short,
            translateExpenseError(result.error),
            SnackbarAction.Dismiss,
          );
        }
      });
  }
}

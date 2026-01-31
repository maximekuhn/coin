import { DestroyRef, inject, Injectable, signal } from '@angular/core';
import { GroupService } from '../../../../core/group/group.service';
import { GroupOverview } from '../../../../core/group/group.models';
import { GroupError } from '../../../../core/group/group.errors';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import { ExpenseOverview } from '../../../../core/expense/expense.models';
import { ExpenseError } from '../../../../core/expense/expense.error';
import { ExpenseService } from '../../../../core/expense/expense.service';

@Injectable()
export class HomeFacade {
  private destroyRef = inject(DestroyRef);

  private groupService = inject(GroupService);
  private expenseService = inject(ExpenseService);

  groupsLoading = signal<boolean>(false);
  groups = signal<{ groups: GroupOverview[]; hasMore: boolean }>({ groups: [], hasMore: false });
  groupsError = signal<GroupError | null>(null);

  latestExpensesLoading = signal<boolean>(false);
  latestExpenses = signal<{ expenses: ExpenseOverview[]; hasMore: boolean }>({
    expenses: [],
    hasMore: false,
  });
  latestExpensesError = signal<ExpenseError | null>(null);

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

  loadLatestExpenses() {
    if (this.latestExpensesLoading()) {
      return;
    }

    this.latestExpensesError.set(null);
    this.latestExpensesLoading.set(true);

    this.expenseService
      .getLatestExpense()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: (res: { expenses: ExpenseOverview[]; hasMore: boolean }) => {
          this.latestExpenses.set(res);
        },
        error: (err: ExpenseError) => {
          this.latestExpensesError.set(err);
          this.latestExpensesLoading.set(false);
        },
        complete: () => {
          this.latestExpensesLoading.set(false);
        },
      });
  }
}

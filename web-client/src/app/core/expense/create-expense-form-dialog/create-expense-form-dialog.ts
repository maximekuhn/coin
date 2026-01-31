import { Component, DestroyRef, inject, OnInit, signal } from '@angular/core';
import { ExpenseService } from '../expense.service';
import { FormControl, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatSelectModule } from '@angular/material/select';
import { GroupService } from '../../group/group.service';
import { GroupOverview } from '../../group/group.models';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import { GroupError } from '../../group/group.errors';
import { AuthService } from '../../auth/auth.service';
import { MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { CreateExpenseFormResultType } from './create-expense-form-dialog.result';
import { ExpenseError } from '../expense.error';

@Component({
  selector: 'app-create-expense-form-dialog',
  imports: [ReactiveFormsModule, MatInputModule, MatButtonModule, MatSelectModule, MatDialogModule],
  templateUrl: './create-expense-form-dialog.html',
  styleUrl: './create-expense-form-dialog.scss',
})
export class CreateExpenseFormDialog implements OnInit {
  private dialogRef = inject(MatDialogRef<CreateExpenseFormDialog>);
  private destroyRef = inject(DestroyRef);
  private expenseService = inject(ExpenseService);
  private groupService = inject(GroupService);
  private authService = inject(AuthService);

  groups = signal<GroupOverview[]>([]);
  groupsLoading = signal(false);
  groupsError = signal<GroupError | null>(null);

  createExpenseForm = new FormGroup({
    amount: new FormControl(0, [Validators.min(1)]),
    groupId: new FormControl('', [Validators.minLength(1)]),
  });

  ngOnInit(): void {
    this.loadGroups();
  }

  createExpense() {
    if (!this.canSubmit()) {
      return;
    }

    const amount = this.createExpenseForm.value.amount!;
    const groupId = this.createExpenseForm.value.groupId!;
    this.expenseService
      .createExpense(groupId, amount, new Date(), this.authService.user()!.id)
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: () => {
          this.dialogRef.close({
            status: CreateExpenseFormResultType.Success,
          });
        },
        error: (err: ExpenseError) => {
          this.dialogRef.close({
            status: CreateExpenseFormResultType.Error,
            error: err,
          });
        },
      });
  }

  private loadGroups() {
    if (this.groupsLoading()) {
      return;
    }

    this.groupsLoading.set(true);
    this.groupsError.set(null);

    this.groupService
      .getGroupsOverview()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: (res: { groups: GroupOverview[]; hasMore: boolean }) => {
          this.groups.set(res.groups);
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

  canSubmit(): boolean {
    return this.createExpenseForm.valid;
  }
}

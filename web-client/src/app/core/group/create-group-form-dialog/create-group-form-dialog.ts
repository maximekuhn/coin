import { Component, DestroyRef, inject } from '@angular/core';
import { FormControl, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { MatInputModule } from '@angular/material/input';
import { GroupService } from '../group.service';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import { GroupError } from '../group.errors';
import { CreateGroupFormResultType } from './create-group-form-dialog.result';

@Component({
  selector: 'app-create-group-form-dialog',
  imports: [ReactiveFormsModule, MatInputModule, MatButtonModule, MatDialogModule],
  templateUrl: './create-group-form-dialog.html',
  styleUrl: './create-group-form-dialog.scss',
})
export class CreateGroupFormDialog {
  private dialogRef = inject(MatDialogRef<CreateGroupFormDialog>);
  private groupService = inject(GroupService);
  private destroyRef = inject(DestroyRef);

  createGroupForm = new FormGroup({
    name: new FormControl('', [Validators.minLength(1)]),
  });

  createGroup() {
    if (!this.canSubmit()) {
      return;
    }
    this.groupService
      .createGroup(this.createGroupForm.value.name!)
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: () => {
          this.dialogRef.close({
            status: CreateGroupFormResultType.Success,
          });
        },
        error: (err: GroupError) => {
          this.dialogRef.close({
            status: CreateGroupFormResultType.Error,
            error: err,
          });
        },
      });
  }

  canSubmit(): boolean {
    return this.createGroupForm.valid;
  }
}

import { inject, Injectable } from '@angular/core';
import { SnackbarDuration, SnackbarType } from './snackbar.types';
import { MatSnackBar, MatSnackBarVerticalPosition } from '@angular/material/snack-bar';
import { getSnackbarDurationMs, getSnackbarPanelClass } from './snackbar.constants';

@Injectable({
  providedIn: 'root',
})
export class SnackbarService {
  private snackBar = inject(MatSnackBar);

  open(type: SnackbarType, duration: SnackbarDuration, message: string, action: string, position: MatSnackBarVerticalPosition = "bottom") {
    this.snackBar.open(message, action, {
      duration: getSnackbarDurationMs(duration) ?? undefined,
      panelClass: getSnackbarPanelClass(type),
      verticalPosition: position,
    });
  }

  dismiss() {
    this.snackBar.dismiss();
  }
}

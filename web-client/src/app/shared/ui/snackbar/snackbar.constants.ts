import { SnackbarAction, SnackbarDuration, SnackbarType } from './snackbar.types';

export function getSnackbarPanelClass(type: SnackbarType): string {
  switch (type) {
    case SnackbarType.Error:
      return 'snackbar-error';
    case SnackbarType.Success:
      return 'snackbar-success';
    default:
      return '';
  }
}

export function getSnackbarDurationMs(duration: SnackbarDuration): number | null {
  switch (duration) {
    case SnackbarDuration.Short:
      return 30_000;
    default:
      return null;
  }
}

export function getSnackbarLocalizedAction(action: SnackbarAction): string {
  switch (action) {
    case SnackbarAction.Ok:
      return $localize`:@@snackbar.ok:Ok`;
    case SnackbarAction.Dismiss:
      return $localize`:@@snackbar.dismiss:Dismiss`;
  }
}

import { SnackbarDuration, SnackbarType } from "./snackbar.types";

export function getSnackbarPanelClass(type: SnackbarType): string {
  switch (type) {
    case SnackbarType.Error:
      return "snackbar-error";
    default:
      return "";
  }
}

export function getSnackbarDurationMs(duration: SnackbarDuration): number | null {
  switch (duration) {
    case SnackbarDuration.Short:
      return 30_000
    default:
      return null;
  }
}

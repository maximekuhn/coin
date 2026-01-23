import { Component, inject, OnDestroy, signal } from '@angular/core';
import { FormControl, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import { MatButton, MatIconButton } from '@angular/material/button';
import { MatFormField, MatFormFieldModule, MatLabel } from '@angular/material/form-field';
import { MatIcon } from '@angular/material/icon';
import { MatInputModule } from '@angular/material/input';
import { Router, RouterLink } from '@angular/router';
import { AuthService } from '../auth.service';
import { AuthError } from '../auth.errors';
import { Subscription } from 'rxjs';
import { translateAuthError } from '../auth.i18n';
import { SnackbarService } from '../../../shared/ui/snackbar/snackbar.service';
import {
  SnackbarAction,
  SnackbarDuration,
  SnackbarType,
} from '../../../shared/ui/snackbar/snackbar.types';

@Component({
  selector: 'app-login-form',
  imports: [
    ReactiveFormsModule,
    MatFormFieldModule,
    MatInputModule,
    MatFormField,
    MatLabel,
    MatButton,
    RouterLink,
    MatIcon,
    MatIconButton,
  ],
  templateUrl: './login-form.html',
  styleUrl: './login-form.scss',
})
export class LoginForm implements OnDestroy {
  private authService = inject(AuthService);
  private router = inject(Router);
  private snackbarService = inject(SnackbarService);

  private subscriptions = new Subscription();

  hidePassword = signal(true);

  loginForm = new FormGroup({
    email: new FormControl('', [Validators.email]),
    password: new FormControl('', [Validators.minLength(1), Validators.maxLength(128)]),
  });

  ngOnDestroy(): void {
    this.snackbarService.dismiss();
    this.subscriptions.unsubscribe();
  }

  onSubmit() {
    if (!this.canSubmit()) {
      return;
    }
    this.subscriptions.add(
      this.authService
        .login(this.loginForm.value.email!, this.loginForm.value.password!)
        .subscribe({
          next: () => {
            this.router.navigate(['/']);
          },
          error: (err: AuthError) => {
            this.openErrorSnackbar(err);
          },
        }),
    );
  }

  canSubmit() {
    return this.loginForm.valid;
  }

  toggleHidePassword(event: MouseEvent) {
    this.hidePassword.set(!this.hidePassword());
    event.stopPropagation();
  }

  private openErrorSnackbar(err: AuthError) {
    this.snackbarService.open(
      SnackbarType.Error,
      SnackbarDuration.Short,
      translateAuthError(err),
      SnackbarAction.Dismiss,
      'top',
    );
  }
}

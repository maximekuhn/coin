import { Component, computed, DestroyRef, inject } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatToolbar } from '@angular/material/toolbar';
import { Router, RouterLink } from '@angular/router';
import { AuthService } from '../../core/auth/auth.service';
import { AuthStatus } from '../../core/auth/auth.models';
import { MatMenuModule } from '@angular/material/menu';
import { MatIconModule } from '@angular/material/icon';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';
import { formatUsername } from '../../shared/utils/user.utils';

@Component({
  selector: 'app-navbar',
  imports: [MatToolbar, RouterLink, MatMenuModule, MatButtonModule, MatIconModule],
  templateUrl: './navbar.html',
  styleUrl: './navbar.scss',
})
export class Navbar {
  private authService = inject(AuthService);
  private router = inject(Router);
  private destroyRef = inject(DestroyRef);

  readonly isLoading = computed(() => this.authService.authState().status === AuthStatus.Unknown);

  readonly isLoggedIn = computed(
    () => this.authService.authState().status === AuthStatus.Authenticated,
  );

  readonly username = computed(() => {
    const name = this.authService.user()?.name ?? '';
    const id = this.authService.user()?.id ?? '';
    return formatUsername(name, id);
  });

  logout() {
    this.authService
      .logout()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: () => {
          this.router.navigate(['/login']);
        },
        error: () => {
          this.router.navigate(['/error']);
        },
      });
  }
}

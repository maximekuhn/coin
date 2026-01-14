import { Component, computed, inject, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { Navbar } from './layout/navbar/navbar';
import { AuthService } from './core/auth/auth.service';
import { AuthStatus } from './core/auth/auth.models';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, Navbar, MatProgressSpinnerModule],
  templateUrl: './app.html',
  styleUrl: './app.scss'
})
export class App {
  private authService = inject(AuthService);

  protected readonly title = signal('web-client');

  readonly isLoading = computed(
    () => this.authService.authState().status === AuthStatus.Unknown
  );

}

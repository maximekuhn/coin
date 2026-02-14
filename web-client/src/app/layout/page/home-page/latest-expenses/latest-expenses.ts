import { Component, inject } from '@angular/core';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { HomeFacade } from '../facade/home-facade';
import { MatListModule } from '@angular/material/list';
import { MatIconModule } from '@angular/material/icon';
import { CurrencyPipe, DatePipe } from '@angular/common';
import { formatUsername } from '../../../../shared/utils/user.utils';
import { UserOverview } from '../../../../core/expense/expense.models';
import { Router } from '@angular/router';

@Component({
  selector: 'app-latest-expenses',
  imports: [MatProgressSpinnerModule, MatListModule, MatIconModule, CurrencyPipe, DatePipe],
  templateUrl: './latest-expenses.html',
  styleUrl: './latest-expenses.scss',
})
export class LatestExpenses {
  private homeFacade = inject(HomeFacade);
  private router = inject(Router);

  readonly isLoading = this.homeFacade.latestExpensesLoading.asReadonly();
  readonly latestExpenses = this.homeFacade.latestExpenses.asReadonly();

  formatPayerName(payer: UserOverview): string {
    return formatUsername(payer.name, payer.id);
  }

  navigateToGroup(groupId: string) {
    this.router.navigate([`/groups/${groupId}`]);
  }
}

import { Component, inject } from '@angular/core';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { HomeFacade } from '../facade/home-facade';
import { MatListModule } from '@angular/material/list';
import { MatIconModule } from '@angular/material/icon';
import { UserOverview } from '../../../../core/group/group.models';
import { formatUsername } from '../../../../shared/utils/user.utils';
import { CurrencyPipe, DatePipe } from '@angular/common';
import { Router } from '@angular/router';

@Component({
  selector: 'app-group-overview-list',
  imports: [MatProgressSpinnerModule, MatListModule, MatIconModule, DatePipe, CurrencyPipe],
  templateUrl: './group-overview-list.html',
  styleUrl: './group-overview-list.scss',
})
export class GroupOverviewList {
  private homeFacade = inject(HomeFacade);
  private router = inject(Router);

  readonly isLoading = this.homeFacade.groupsLoading.asReadonly();
  readonly groups = this.homeFacade.groups.asReadonly();

  formatOwnerName(owner: UserOverview): string {
    return formatUsername(owner.name, owner.id);
  }

  navigateToGroup(groupId: string) {
    this.router.navigate([`/groups/${groupId}`]);
  }
}

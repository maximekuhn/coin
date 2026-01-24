import { Component, inject } from '@angular/core';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { HomeFacade } from '../facade/home-facade';
import { MatListModule } from '@angular/material/list';
import { MatIconModule } from '@angular/material/icon';
import { UserOverview } from '../../../../core/group/group.models';
import { formatUsername } from '../../../../shared/utils/user.utils';
import { DatePipe } from '@angular/common';

@Component({
  selector: 'app-group-overview-list',
  imports: [MatProgressSpinnerModule, MatListModule, MatIconModule, DatePipe],
  templateUrl: './group-overview-list.html',
  styleUrl: './group-overview-list.scss',
})
export class GroupOverviewList {
  private homeFacade = inject(HomeFacade);

  readonly isLoading = this.homeFacade.groupsLoading.asReadonly();
  readonly groups = this.homeFacade.groups.asReadonly();

  formatOwnerName(owner: UserOverview): string {
    return formatUsername(owner.name, owner.id);
  }
}

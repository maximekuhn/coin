import { ListResponse } from '../../shared/api/list-response';
import { Group, GroupOverview, UserOverview } from './group.models';

export interface BackendCreateGroupResponse {
  groupId: string;
}

export interface BackendGroupOverview {
  id: string;
  name: string;
  owner: BackendUserOverview;
  createdAt: Date;
  lastActivity?: Date;
  currentUserBalanceEuros?: number;
}

export interface BackendUserOverview {
  id: string;
  name: string;
}

export interface BackendGroup {
  id: string;
  name: string;
  ownerId: string;
  members: string[];
  createdAt: Date;
}

export function backendGroupToGroup(b: BackendGroup): Group {
  return {
    id: b.id,
    name: b.name,
    ownerId: b.ownerId,
    members: b.members,
    createdAt: b.createdAt,
  };
}

export function backendGroupOverviewListToGroupOverviewList(res: ListResponse<GroupOverview>): {
  groups: GroupOverview[];
  hasMore: boolean;
} {
  return {
    groups: res.data.map(backendGroupOverviewToGroupOverview),
    hasMore: res.totalItems > res.data.length,
  };
}

function backendGroupOverviewToGroupOverview(b: BackendGroupOverview): GroupOverview {
  return {
    id: b.id,
    name: b.name,
    owner: backendUserOverviewToUserView(b.owner),
    createdAt: b.createdAt,
    lastActivity: b.lastActivity,
    currentUserBalanceEuros: b.currentUserBalanceEuros,
  };
}

function backendUserOverviewToUserView(b: BackendUserOverview): UserOverview {
  return {
    id: b.id,
    name: b.name,
  };
}

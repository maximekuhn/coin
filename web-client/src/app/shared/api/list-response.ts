import { Pagination } from './pagination';

export interface ListResponse<T> {
  data: T[];
  requestPagination: Pagination;
  totalItems: number;
}

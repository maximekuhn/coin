export enum ExpenseErrorKind {
  // Invalid amount during expense creation.
  InvalidAmount,

  Unknown,
}

export interface ExpenseError {
  kind: ExpenseErrorKind;
}

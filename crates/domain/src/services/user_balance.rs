use crate::{
    entities::Expense,
    types::{money::Money, user_id::UserId},
};

pub fn compute_user_balance(expenses: &[Expense], user_id: UserId) -> Option<Money> {
    if expenses.is_empty() {
        return None;
    }

    let mut balance = Money::zero();

    for expense in expenses {
        if !expense.contains_user(&user_id) {
            continue;
        }

        let shares = expense.participants.len() + 1;
        if shares == 1 {
            continue;
        }

        let share = Money::from_cents((expense.total.cents() as usize / shares) as i64);

        if expense.is_payer(&user_id) {
            balance += share;
        } else {
            balance -= share;
        }
    }

    match balance.cents() {
        0 => None,
        _ => Some(balance),
    }
}

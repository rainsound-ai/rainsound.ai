use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
pub struct NumBytes(pub usize);

pub trait HasSizeBudget {
    fn size_budget(&self) -> NumBytes;
    fn check_size_budget(&self) -> HowCloseToBudget;
}

pub enum HowCloseToBudget {
    WellBelowBudget,
    CloseToBudget {
        actual_size: NumBytes,
        budget: NumBytes,
    },
    OverBudget {
        actual_size: NumBytes,
        budget: NumBytes,
    },
}

impl HowCloseToBudget {
    pub fn from_num_bytes(actual_size: usize, budget: NumBytes) -> Self {
        let budget = budget.0;
        let half_of_budget = budget / 2;

        if (0..=half_of_budget).contains(&actual_size) {
            return HowCloseToBudget::WellBelowBudget;
        }

        if (half_of_budget..=budget).contains(&actual_size) {
            return HowCloseToBudget::CloseToBudget {
                actual_size: NumBytes(actual_size),
                budget: NumBytes(budget),
            };
        }

        HowCloseToBudget::OverBudget {
            actual_size: NumBytes(actual_size),
            budget: NumBytes(budget),
        }
    }
}

impl Display for HowCloseToBudget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HowCloseToBudget::WellBelowBudget => write!(f, "Well below budget"),
            HowCloseToBudget::CloseToBudget {
                actual_size,
                budget,
            } => write!(
                f,
                "Close to budget ({} bytes out of {} bytes)",
                actual_size.0, budget.0
            ),
            HowCloseToBudget::OverBudget {
                actual_size,
                budget,
            } => write!(
                f,
                "Over budget ({} bytes out of {} bytes)",
                actual_size.0, budget.0
            ),
        }
    }
}

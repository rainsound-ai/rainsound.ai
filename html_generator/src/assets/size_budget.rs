use std::fmt::Display;

use super::NonImageAsset;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct NumBytes(pub usize);

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
    pub fn new<Asset: NonImageAsset + ?Sized>(asset: &Asset) -> Self {
        let actual_size = asset.bytes().len();
        let budget = asset.size_budget().0;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[tokio::test]
    async fn test_bundle_size() {
        let assets = Assets::new().await;
        let html_assets_with_size_budget = assets.html_assets_with_size_budget();
        let non_html_assets_with_size_budget = non_html_assets.assets_with_size_budget();

        let assets_with_size_budget: Vec<&dyn NonImageAsset> = html_assets_with_size_budget
            .into_iter()
            .chain(non_html_assets_with_size_budget.into_iter())
            .collect::<Vec<_>>();

        for asset in assets_with_size_budget {
            let how_close_to_budget = asset.check_size_budget();

            match how_close_to_budget {
                HowCloseToBudget::WellBelowBudget => {}

                HowCloseToBudget::CloseToBudget { .. } => {
                    println!("{}", how_close_to_budget);
                }

                HowCloseToBudget::OverBudget { .. } => {
                    println!("{}", how_close_to_budget);
                    panic!("Asset is over budget.");
                }
            }
        }
    }
}

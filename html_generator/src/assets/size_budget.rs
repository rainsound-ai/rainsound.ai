use std::{fmt::Display, path::Path, time::Duration};

use super::NonImageAsset;

// Assuming 10 Mbps download speed, which seems like a conservative
// estimate for mobile phones.
//
// Should also factor in typical latency and parsing time.
pub fn loading_time_budget(duration: Duration) -> NumBytes {
    //
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct NumBytes(pub usize);

pub enum HowCloseToBudget<'asset> {
    WellBelowBudget,
    CloseToBudget {
        path: &'asset Path,
        actual_size: NumBytes,
        budget: NumBytes,
    },
    OverBudget {
        path: &'asset Path,
        actual_size: NumBytes,
        budget: NumBytes,
    },
}

impl<'asset> HowCloseToBudget<'asset> {
    pub fn new<Asset: NonImageAsset + ?Sized>(asset: &'asset Asset) -> HowCloseToBudget<'asset> {
        let path = asset.path();
        let actual_size = asset.bytes().len();
        let budget = asset.size_budget().0;
        let half_of_budget = budget / 2;

        if (0..=half_of_budget).contains(&actual_size) {
            return HowCloseToBudget::WellBelowBudget;
        }

        if (half_of_budget..=budget).contains(&actual_size) {
            return HowCloseToBudget::CloseToBudget {
                path,
                actual_size: NumBytes(actual_size),
                budget: NumBytes(budget),
            };
        }

        HowCloseToBudget::OverBudget {
            path,
            actual_size: NumBytes(actual_size),
            budget: NumBytes(budget),
        }
    }
}

impl Display for HowCloseToBudget<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HowCloseToBudget::WellBelowBudget => Ok(()),
            HowCloseToBudget::CloseToBudget {
                path,
                actual_size,
                budget,
            } => write!(
                f,
                "{} is close to budget ({} bytes out of {} bytes)",
                path.to_str().unwrap(),
                actual_size.0,
                budget.0
            ),
            HowCloseToBudget::OverBudget {
                path,
                actual_size,
                budget,
            } => write!(
                f,
                "{} is over budget ({} bytes out of {} bytes)",
                path.to_str().unwrap(),
                actual_size.0,
                budget.0
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

use std::{fmt::Display, path::Path, time::Duration};

use super::Asset;

pub static assumed_latency: Duration = Duration::from_millis(100);
pub static assumed_bits_per_second: f64 = 5_000_000.0;
pub static assumed_bytes_per_second: f64 = assumed_bits_per_second / 8.0;

/*

load_time_budget_allowed
load_time_budget

*/

// We assume 5 Mbps of bandwidth and 100 milliseconds of
// latency. These are conservative estimates based on this
// survey of U.S. mobile networks from Ookla.
//
// https://www.ookla.com/ookla-for-good/open-data
pub fn estimate_load_time(num_bytes: usize) -> Duration {
    let num_bytes = num_bytes as f64;

    let num_seconds = num_bytes / assumed_bytes_per_second;
    let num_seconds = Duration::from_secs(num_seconds.round() as u64);

    num_seconds + assumed_latency
}

pub enum HowCloseToBudget<'asset> {
    WellBelowBudget,
    CloseToBudget {
        path: &'asset Path,
        estimated_load_time: Duration,
        budgeted_load_time: Duration,
    },
    OverBudget {
        path: &'asset Path,
        estimated_load_time: Duration,
        budgeted_load_time: Duration,
    },
}

impl<'asset> HowCloseToBudget<'asset> {
    pub fn new<Asset: HasPerformanceBudget + ?Sized>(
        asset: &'asset Asset,
    ) -> HowCloseToBudget<'asset> {
        let asset_size = asset.bytes().len();

        let estimated_load_time = estimate_load_time(asset_size);
        let estimated_load_time_secs = estimated_load_time.as_secs_f64();
        let budgeted_load_time = asset.load_time_budget();
        let budgeted_load_time_secs = budgeted_load_time.as_secs_f64();

        let half_of_budget = budgeted_load_time_secs / 2.0;

        if (0.0..=half_of_budget).contains(&estimated_load_time_secs) {
            return HowCloseToBudget::WellBelowBudget;
        }

        let path = asset.path();

        if (half_of_budget..=budgeted_load_time_secs).contains(&estimated_load_time_secs) {
            return HowCloseToBudget::CloseToBudget {
                path,
                estimated_load_time,
                budgeted_load_time,
            };
        }

        HowCloseToBudget::OverBudget {
            path,
            estimated_load_time,
            budgeted_load_time,
        }
    }
}

impl Display for HowCloseToBudget<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HowCloseToBudget::WellBelowBudget => Ok(()),
            HowCloseToBudget::CloseToBudget {
                path,
                estimated_load_time,
                budgeted_load_time,
            } => write!(
                f,
                "{} is close to budget. Esimated load time: {} seconds, budgeted load time: {} seconds.",
                path.to_str().unwrap(),
                estimated_load_time.as_secs_f64(),
                budgeted_load_time.as_secs_f64(),
            ),
            HowCloseToBudget::OverBudget {
                path,
                estimated_load_time,
                budgeted_load_time,
            } => write!(
                f,
                "{} is over budget. Esimated load time: {} seconds, budgeted load time: {} seconds.",
                path.to_str().unwrap(),
                estimated_load_time.as_secs_f64(),
                budgeted_load_time.as_secs_f64(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::prelude::*;

    // #[test]
    // fn check_performance_budgets() {
    //     let assets = Assets::new();
    //     let html_assets_with_performance_budget = assets.html_assets_with_performance_budget();
    //     let non_html_assets_with_performance_budget =
    //         non_html_assets.assets_with_performance_budget();

    //     let assets_with_performance_budget: Vec<&dyn Asset> =
    //         html_assets_with_performance_budget
    //             .into_iter()
    //             .chain(non_html_assets_with_performance_budget.into_iter())
    //             .collect::<Vec<_>>();

    //     for asset in assets_with_performance_budget {
    //         let how_close_to_budget = asset.check_performance_budget();

    //         match how_close_to_budget {
    //             HowCloseToBudget::WellBelowBudget => {}

    //             HowCloseToBudget::CloseToBudget { .. } => {
    //                 println!("{}", how_close_to_budget);
    //             }

    //             HowCloseToBudget::OverBudget { .. } => {
    //                 println!("{}", how_close_to_budget);
    //                 panic!("Asset is over budget.");
    //             }
    //         }
    //     }
    // }
}

pub trait HasPerformanceBudget: Asset {
    fn check_performance_budget(&self) -> HowCloseToBudget {
        HowCloseToBudget::new(self)
    }

    // Used for enforcing performance budgets.
    fn load_time_budget(&self) -> Duration;
}

use analyzis::analyze_crates;
use clear_cfg::clear_conditional_compilation;
use count_code::{count_crates_code, count_expanded_code};
use crate_paths::find_crate_paths;
use expand::expand_crates;
use github::clone_repos;
use github::get_most_popular_repos;
use results::AnalyzisResults;
use state::ScraperState;
use std::error::Error;
use std::path::Path;
use tokio::task::JoinHandle;

#[macro_use]
mod utils;
mod analyzis;
mod clear_cfg;
mod count_code;
mod crate_paths;
mod error;
mod expand;
mod github;
mod lines_count;
mod results;
mod state;

const DATA_PATH: &str = "./data";

fn create_data_folder() {
    std::fs::create_dir_all(DATA_PATH).unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    create_data_folder();
    let mut state = ScraperState::load().unwrap_or_default();
    let repos = get_most_popular_repos(&mut state).await?;
    let repos_path = clone_repos(&mut state, repos).await?;
    let crate_paths = find_crate_paths(&mut state, Path::new(&repos_path))?;
    let mut results = AnalyzisResults::load().unwrap_or(AnalyzisResults::from(&crate_paths));
    count_crates_code(&mut state, &mut results)?;
    clear_conditional_compilation(&mut state, &results)?;
    expand_crates(&mut state, &mut results).await?;
    count_expanded_code(&mut state, &mut results)?;
    analyze_crates(&mut state, &mut results)?;
    results.save()?;
    state.save()?;
    Ok(())
}

use async_trait::async_trait;
use tavern_pathfinder::summary::{Summary, Summarize};
use uuid::Uuid;
use warp::reject::Rejection;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}

pub mod character;
pub mod class;
pub mod effects;
pub mod feat;
pub mod item;
pub mod religion;
pub mod spell;

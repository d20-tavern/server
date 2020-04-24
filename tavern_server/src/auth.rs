use argon2::{self, Config, ThreadMode, Variant, Version};
use structopt::StructOpt;

pub const ARGON2_HASH_LENGTH: u32 = 32;

#[cfg(test)]
mod tests {
	use super::*;

    const TEST_MEMORY: u32 = 1024u32;
    const TEST_TIME_COST: u32 = 10u32;
    const TEST_THREADS: u32 = 4_u32;

	#[test]
    fn argon2_config_memory_is_set() {
        let a2 = Argon2Opt {
            memory: TEST_MEMORY,
            time_cost: TEST_TIME_COST,
            threads: TEST_THREADS,
        };

        let a2conf: Config = a2.into();

        assert_eq!(a2conf.mem_cost, TEST_MEMORY);
    }

	#[test]
    fn argon2_config_time_cost_is_set() {
        let a2 = Argon2Opt {
            memory: TEST_MEMORY,
            time_cost: TEST_TIME_COST,
            threads: TEST_THREADS,
        };

        let a2conf: Config = a2.into();

        assert_eq!(a2conf.time_cost, TEST_TIME_COST);
    }

	#[test]
    fn argon2_config_threads_is_set() {
        let a2 = Argon2Opt {
            memory: TEST_MEMORY,
            time_cost: TEST_TIME_COST,
            threads: TEST_THREADS,
        };

        let a2conf: Config = a2.into();

        assert_eq!(a2conf.lanes, TEST_THREADS);
    }
}

#[derive(StructOpt, Debug)]
pub struct Argon2Opt {
    #[structopt(long = "argon-memory", env = "TAVERN_ARGON2_MEMORY", help = "the amount of memory in KB to use while hashing")]
    memory: u32,
    #[structopt(long = "argon-time-cost", env = "TAVERN_ARGON2_TIME_COST", help = "the amount of time a single hash should take")]
    time_cost: u32,
    #[structopt(long = "argon-threads", env = "TAVERN_ARGON2_THREADS", help = "the number of threads to use while hashing")]
    threads: u32,
}

impl From<Argon2Opt> for argon2::Config<'static> {
    fn from(opt: Argon2Opt) -> Config<'static> {
        let mut config = Config::default();
        config.variant = Variant::Argon2i;
        config.version = Version::Version13;
        config.thread_mode = ThreadMode::Parallel;
        config.mem_cost = opt.memory;
        config.lanes = opt.threads;
        config.time_cost = opt.time_cost;
        config
    }
}

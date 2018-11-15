#![deny(bare_trait_objects)]

extern crate bloom;
#[macro_use] extern crate clap;
#[macro_use] extern crate log;
extern crate noodles;
extern crate rand;

pub use generator::Generator;
pub use pair_writer::PairWriter;
pub use validators::ValidationLevel;

pub mod commands;
pub mod distributions;
pub mod generator;
pub mod pair_writer;
pub mod validators;
pub mod record;

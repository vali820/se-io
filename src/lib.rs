pub mod serialization;
pub(crate) mod data_interface;
mod binary_reader;

pub(crate) use data_interface::DataInterface;

pub use binary_reader::BinaryReader;
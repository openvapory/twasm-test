#[macro_use]
extern crate pwasm_test;
extern crate pwasm_std;

use pwasm_std::hash::H256;
use pwasm_std::storage;
use pwasm_test::{ExternalBuilder};

test_with_external!(
	ExternalBuilder::new().storage_write(H256::new(), [250; 32]).build(),
	read_storage {
		assert_eq!([250; 32], storage::read(&H256::new()));
	}
);

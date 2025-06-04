use std::collections::Hashmap

// Each intent holds a unique label and whether its been acted on
pub struct Intent {
	pub label: String,
	pub completed: bool,
}

// The tracker keesps track of all intentions

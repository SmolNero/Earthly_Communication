use std::collections::Hashmap

// Each intent holds a unique label and whether its been acted on
pub struct Intent {
	pub label: String,		          // A name or description of the intent
	pub completed: bool,	          // When the intent was recorded 
    pub value: f32,			          // Optional weight or intesisty
}

pub struct IntentTracker {
	intents: HasMap<String, Intent>,  // Using a HashMap for label-based access
}
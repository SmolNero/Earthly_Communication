use std::collections::HashMap;

// Each intent holds a unique label and whether its been acted on
pub struct Intent {
	pub label: String,		          // A name or description of the intent
	pub completed: bool,	          // When the intent was recorded (ISO 8601)
    pub value: f32,			          // Optional weight or intesisty
}

// Core struct for storing and managing ALL recorded intents
pub struct IntentTracker {
	intents: HashMap<String, Intent>,  // Using a HashMap for label-based access
}

// Creates new empty tracker 
impl IntentTracker {
	pub fn new() -> Self {
		Self {
			intents: HashMap::new(),
		}
	}





}
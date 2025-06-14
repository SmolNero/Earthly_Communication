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

	pub fn record(&mut self, label: &str, timestamp: &str, value: f32){
		let intent = Intent {
			label: label.to_string(),
			timestamp: timestamp.to_string(),
			value,
		};
		self.intents.insert(label.to_string(), intents);	
	}

	// Remove an intent from the log
	pub fn discard(&mut self, label: &str) {
		self.intents.remove(label);
	}

	// Print all retained intents - acts as a simple inspection tool
	pub	fn inspect(&self) {
		println!("\n-- Current intent State ---");
		for intnet in self.intents.values() {
			println! (
				"[{}] @ {} -> value: {}",
				intent,label, intent.timestamp, intent.value
			);
		}
	}
}
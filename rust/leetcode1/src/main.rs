use llama_cpp::LlamaLLM;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Document {
    id: String,
    text: String,
}

fn main() {
    let documents = vec![
        Document {
            id: "1".to_string(),
            text: "Hello from LlamaIndex in Rust!".to_string(),
        },
        Document {
            id: "2".to_string(),
            text: "Rust is robust & safe!".to_string(),
        },
    ];

    // Simulate indexing data.
    let index = LlamaLLM::new_list_index(documents);

    // Simulate a query
    let result = index.query("What does Rust provide?");
    println!("Response: {result}");
}

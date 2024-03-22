use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use anyhow::Result;

pub struct LLMClient {
    inner: Ollama
}

impl LLMClient {
    pub fn new() -> LLMClient {
        // TODO: Should be configurable.
        let inner = Ollama::default();
        LLMClient {
            inner
        }
    }

     pub async fn get(&self, language: &str, code: &str) -> Result<String> {
        let model = "codellama".to_string();
        // TODO: Improve this prompt
        let prompt = format!(r#"Understand what this method does and 
            add a detailed doc comment explaining that to the following {} method:
            {}
            The doc comment should describe what the method does. 
            Return the method implementaion with the doc comment above the method
            as a go code comment. Don't include any explanations in your response."#, language, code);

        let response = self.inner.generate(GenerationRequest::new(model, prompt)).await;
        match response {
            Ok(generated) => Ok(generated.response),
            Err(e) => Err(anyhow::anyhow!("Error generating response: {}", e))
        }
    }
}



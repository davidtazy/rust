use std::sync::Arc;




pub trait Transformer : Send + Sync {
    fn transform(&self, input: &str) -> String;
}

pub trait Publisher : Send + Sync{
    fn publish(&self, message: String);
}

pub trait Repository {
    fn save(&self, message: String);
}

pub struct UpperCaseTransformer;
impl Transformer for UpperCaseTransformer {
    fn transform(&self, input: &str) -> String {
        input.to_uppercase()
    }
}


pub struct Dispatcher{
     publisher: Arc<dyn Publisher>,
     //repository: Box<dyn Repository>,
}

impl Dispatcher {
    pub fn new(publisher: Arc<dyn Publisher>/* , repository: Box<dyn Repository>*/) -> Self {
        Dispatcher { publisher/* , repository*/ }
    }
    
    pub fn dispatch(&self, message: String) {
        self.publisher.publish(message.clone());
        //self.repository.save(message);
    }
}
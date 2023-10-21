trait Resource {
    type Output;
    fn modified(&self) -> Modified;
    fn generate(&self) -> Self::Output;
}

pub enum Modified {
    
}
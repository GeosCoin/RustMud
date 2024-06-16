pub trait PostProcess {
    fn execute(&mut self) -> String;
}


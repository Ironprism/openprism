use crate::parse::*;

pub struct PythonBindings<'a> {
    pub module: &'a Module,
    pub items: Vec<syn::Item>,
}

impl<'a> PythonBindings<'a> {
    pub fn new(module: &'a Module) -> Self {
        Self {
            module: module,
            items: Vec::new(),
        }
    }

    fn parse_module(&mut self, module: &Module) {


    }

    pub fn build(mut self) -> syn::File {
        self.parse_module(self.module);

        // build function with modules tree
        todo!();
    }


}
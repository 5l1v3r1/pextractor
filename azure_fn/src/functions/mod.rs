mod convert;

pub const FUNCTIONS: &[&azure_functions::codegen::Function] = azure_functions::export!{
  convert::convert,
};

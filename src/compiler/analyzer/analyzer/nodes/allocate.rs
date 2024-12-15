pub use read::handle_read;
pub use store::handle_store;

use crate::compiler::{
    analyzer::{FunctionCtx, ProgramCtx},
    errors::Location,
    parser::ExpressionInfo,
    types::Type,
};

mod read;
mod store;

pub fn handle_allocation(
    program: &mut ProgramCtx,
    function: &mut FunctionCtx,
    location: &Location,
    destination: &String,
    data_type: &Type,
    info: ExpressionInfo,
) {
    // if data_type.base.is_basic() {
    //     function
    //         .operations
    //         .allocate(&destination, &data_type.convert());
    // }
    
    handle_store(program, function, location, destination, data_type, info);
}

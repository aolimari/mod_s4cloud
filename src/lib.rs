#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
    unused_parens,
	unused_macros,
	unused_variables,
	unused_assignments,
	non_upper_case_globals,
	non_snake_case,
	dead_code,
    clippy::borrow_interior_mutable_const
)]

mod aerials;
mod ground;
mod specials;
mod smashattacks;

#[skyline::main(name = "s4cloud-test1")]
pub fn main() {
    aerials::install();
    ground::install();
    specials::install();
    println!("time to get cooked!");
}
 
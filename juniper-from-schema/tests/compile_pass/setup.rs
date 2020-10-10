use juniper::{EmptyMutation, EmptySubscription, Executor, FieldResult, Variables, ID};

pub struct Context;
impl juniper::Context for Context {}

fn main() {}

#[allow(dead_code)]
fn __use_all_the_imports(
    _: EmptyMutation<()>,
    _: Executor<()>,
    _: FieldResult<(), ()>,
    _: Variables,
    _: ID,
    _: EmptySubscription<()>,
) {
}

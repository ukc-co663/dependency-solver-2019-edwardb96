use z3::{Ast, Context};
use super::utils::{ConditionIterator, SummableIterator};

pub fn make_one_change_at_a_time_invariant<'ctx>(ctx: &'ctx Context,
                                                 package_variables: &Vec<Vec<Ast<'ctx>>>)
    -> Option<Ast<'ctx>> {

    fn only_one_change<'ctx>(ctx: &'ctx Context,
                                   prev_state: &Vec<Ast<'ctx>>,
                                   next_state: &Vec<Ast<'ctx>>) -> Ast<'ctx> {
        izip!(prev_state, next_state).map(|(prev, next)| {
            is_transition_int(ctx, prev, next)
        }).total().unwrap().le(&ctx.from_i64(1))
    }

    fn is_transition_int<'ctx>(ctx: &'ctx Context,
                             prev: &Ast<'ctx>,
                             next: &Ast<'ctx>) -> Ast<'ctx> {
        let is_change = prev.xor(&next);
        is_change.ite(&ctx.from_i64(1), &ctx.from_i64(0))
    }

    package_variables.windows(2).map(|w| match w {
        [prev_state, next_state] =>
            only_one_change(&ctx, &prev_state, &next_state),
        _ => panic!("matching window of wrong size")
    }).conjunction()
}

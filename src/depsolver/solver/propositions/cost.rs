use crate::depsolver::model::package::Package;
use z3::{Context, Ast};
use super::utils::SummableIterator;

const UNINSTALL_COST : i64 = 1000000;

pub fn make_cost_constraint<'ctx>(ctx : &'ctx Context,
                                  package_variables: &Vec<Vec<Ast<'ctx>>>,
                                  packages: &Vec<Package>) -> (Ast<'ctx>, Ast<'ctx>) {

    fn state_transition_cost<'ctx>(ctx: &'ctx Context,
                                   sizes: &Vec<u32>,
                                   prev_state: &Vec<Ast<'ctx>>,
                                   next_state: &Vec<Ast<'ctx>>) -> Option<Ast<'ctx>> {
        izip!(prev_state, next_state, sizes).map(|(prev, next, size)| {
            transition_cost(ctx, prev, next, *size)
        }).total()
    }

    fn transition_cost<'ctx>(ctx: &'ctx Context,
                             prev: &Ast<'ctx>,
                             next: &Ast<'ctx>,
                             size: u32) -> Ast<'ctx> {
        let is_install = prev.not().and(&[next]);
        let is_uninstall = prev.and(&[&next.not()]);
        is_uninstall.ite(&ctx.from_i64(UNINSTALL_COST),
            &is_install.ite(&ctx.from_i64(size as i64), &ctx.from_i64(0)))
    }

    let sizes = packages.iter().map(|p| p.size).collect();
    let cost_value = package_variables.windows(2).filter_map(|w| match w {
        [prev_state, next_state] =>
            state_transition_cost(ctx, &sizes, &prev_state, &next_state),
        _ => panic!("matching window of wrong size")
    }).total().expect("Too few steps to define cost, step_limit must be > 2");

    let cost_variable = ctx.named_int_const("cost");
    let cost_constraint = cost_variable._eq(&cost_value);

    (cost_variable, cost_constraint)
}

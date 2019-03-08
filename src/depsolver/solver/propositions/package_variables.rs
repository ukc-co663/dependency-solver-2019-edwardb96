use z3::{Context, Ast};

pub fn make_package_variables(ctx: &Context, package_count: usize, step_limit: usize)
    -> Vec<Vec<Ast>> {
    let mut variables = vec![vec![]; step_limit];
    for time in 0..step_limit {
        for package_id in 0..package_count {
            let var = ctx.named_bool_const(&format!("{}_{}", package_id, time));
            variables[time].push(var);
        }
    }
    variables
}

use std::iter::Iterator;
use z3::Ast;

pub trait SummableIterator<'ctx> {
    fn total(self) -> Option<Ast<'ctx>>;
}

impl<'ctx, I> SummableIterator<'ctx> for I
    where I : Iterator<Item=Ast<'ctx>> {

    fn total(self) -> Option<Ast<'ctx>> {
        let mut conds : Vec<Ast> = self.collect();
        if conds.len() == 1 {
            conds.pop()
        } else {
            let mut conds_ref : Vec<&Ast<'ctx>> = conds.iter().collect();
            conds_ref.pop().map(|cond| cond.add(&conds_ref[..]))
        }
    }
}

pub trait ConditionIterator<'ctx> {
    fn conjunction(self) -> Option<Ast<'ctx>>;
    fn disjunction(self) -> Option<Ast<'ctx>>;
}

impl<'ctx, I> ConditionIterator<'ctx> for I
    where I : Iterator<Item=Ast<'ctx>> {

    fn conjunction(self) -> Option<Ast<'ctx>> {
        let mut conds : Vec<Ast> = self.collect();
        if conds.len() == 1 {
            conds.pop()
        } else {
            let mut conds_ref : Vec<&Ast<'ctx>> = conds.iter().collect();
            conds_ref.pop().map(|cond| cond.and(&conds_ref[..]))
        }
    }

    fn disjunction(self) -> Option<Ast<'ctx>> {
        let mut conds : Vec<Ast> = self.collect();
        if conds.len() == 1 {
            conds.pop()
        } else {
            let mut conds_ref : Vec<&Ast<'ctx>> = conds.iter().collect();
            conds_ref.pop().map(|cond| cond.or(&conds_ref[..]))
        }
    }
}

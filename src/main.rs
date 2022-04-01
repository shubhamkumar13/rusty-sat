#[derive(Debug, PartialEq, Clone, Copy)]
enum Expr {
    Var(char),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Const(bool),
}

impl Copy for Expr {

}

impl Expr {
    fn free_variable(self) -> Option<char> {
        match self {
            Expr::Var(v) => Some(v),
            Expr::Const(_) => None,
            Expr::Not(new_self) => {
                let new_self = *new_self;
                new_self.free_variable()
            }
            Expr::And(e1, e2) => {
                let (e1, e2) = (*e1, *e2);
                match (e1.free_variable(), e2.free_variable()) {
                    (Some(l), _) => Some(l),
                    (None, r) => r,
                }
            }
            Expr::Or(e1, e2) => {
                let (e1, e2) = (*e1, *e2);
                match (e1.free_variable(), e2.free_variable()) {
                    (Some(l), _) => Some(l),
                    (None, r) => r,
                }
            }
            _ => None,
        }
    }
    fn guess_variable(self, var: char, val: bool) -> Self {
        match self {
            Expr::Var(v) => {
                if v == var {
                    Expr::Const(val)
                } else {
                    Expr::Var(v)
                }
            }
            Expr::Not(e) => {
                let e = *e;
                let e = e.guess_variable(var, val);
                Expr::Not(Box::new(e))
            }
            Expr::And(e1, e2) => {
                let (e1, e2) = (*e1, *e2);
                let (e1, e2) = (e1.guess_variable(var, val), e2.guess_variable(var, val));
                Expr::And(Box::new(e1), Box::new(e2))
            }
            Expr::Or(e1, e2) => {
                let (e1, e2) = (*e1, *e2);
                let (e1, e2) = (e1.guess_variable(var, val), e2.guess_variable(var, val));
                Expr::Or(Box::new(e1), Box::new(e2))
            }
            Expr::Const(b) => Expr::Const(b),
        }
    }
    fn simplify(self) -> Self {
        match self {
            Expr::Const(b) => Expr::Const(b),
            Expr::Var(v) => Expr::Var(v),
            Expr::Not(e) => match e.simplify() {
                Expr::Const(b) => Expr::Const(!b),
                e => Expr::Not(Box::new(e)),
            },
            Expr::Or(x, y) => {
                let x = x.simplify();
                let y = y.simplify();
                let v = vec![x, y];
                let v = v
                    .into_iter()
                    .filter(|x| *x != Expr::Const(false))
                    .collect::<Vec<Expr>>();

                if v.contains(&Expr::Const(true)) {
                    Expr::Const(true)
                } else {
                    match v.as_slice() {
                        [] => Expr::Const(false),
                        [e] => *e,
                        [e1, e2] => Expr::Or(Box::new(*e1), Box::new(*e2)),
                        _ => panic!("error in Or branch of simplify"),
                    }
                }
            }
            Expr::And(x, y) => {
                let x = x.simplify();
                let y = y.simplify();
                let v = vec![x, y];
                let v = v
                    .into_iter()
                    .filter(|&x| x != Expr::Const(true))
                    .collect::<Vec<Expr>>();

                if v.contains(&Expr::Const(false)) {
                    Expr::Const(false)
                } else {
                    match v.as_slice() {
                        [] => Expr::Const(true),
                        [e] => *e,
                        [e1, e2] => Expr::Or(Box::new(*e1), Box::new(*e2)),
                    }
                }
            }
        }
    }
}

fn main() {
    print!("{:#?}", Expr::Var('c'));
}

trait Differentiable: DifferentiableClone + core::fmt::Debug {
    fn differentiate(&self) -> Box<dyn Differentiable>;
}

trait DifferentiableClone {
    fn clone_boxed(&self) -> Box< dyn Differentiable>;
}

impl<T> DifferentiableClone for T
where
    T: 'static + Differentiable + Clone,
{
    fn clone_boxed(&self) -> Box<dyn Differentiable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Differentiable> {
    fn clone(&self) -> Box<dyn Differentiable> {
        self.clone_boxed()
    }
}

#[derive(Clone, Debug)]
struct Constant(i64);

#[derive(Clone, Debug)]
struct Mul(Box<dyn Differentiable>, Box<dyn Differentiable>);

#[derive(Clone, Debug)]
struct Div(Box<dyn Differentiable>, Box<dyn Differentiable>);

#[derive(Clone, Debug)]
struct Add(Box<dyn Differentiable>, Box<dyn Differentiable>);

#[derive(Clone, Debug)]
struct Sub(Box<dyn Differentiable>, Box<dyn Differentiable>);

#[derive(Clone, Debug)]
struct Sin;

#[derive(Clone, Debug)]
struct Cos;

impl Constant {
    fn boxed(i: i64) -> Box<Self> {
        Box::new(Constant(i))
    }
}

impl Sin {
    fn boxed() -> Box<Self> {
        Box::new(Sin)
    }
}

impl Cos {
    fn boxed() -> Box<Self> {
        Box::new(Cos)
    }
}

impl Add {
    fn boxed(a: Box<dyn Differentiable>, b: Box<dyn Differentiable>) -> 
Box<Self> {
        Box::new(Add(a, b))
    }
}

impl Sub {
    fn boxed(a: Box<dyn Differentiable>, b: Box<dyn Differentiable>) -> 
Box<Self> {
        Box::new(Sub(a, b))
    }
}

impl Mul {
    fn boxed(a: Box<dyn Differentiable>, b: Box<dyn Differentiable>) -> 
Box<Self> {
        Box::new(Mul(a, b))
    }
}

impl Div {
    fn boxed(a: Box<dyn Differentiable>, b: Box<dyn Differentiable>) -> 
Box<Self> {
        Box::new(Div(a, b))
    }
}

impl Differentiable for Constant {
    fn differentiate(&self) -> Box<dyn Differentiable> {
        Constant::boxed(0)
    }
}

impl Differentiable for Mul {
    fn differentiate(&self) -> Box<dyn Differentiable> {
        Add::boxed(
            Mul::boxed(self.0.clone(), self.1.differentiate()),
            Mul::boxed(self.0.differentiate(), self.1.clone()),
        )
    }
}

impl Differentiable for Div {
    fn differentiate(&self) -> Box<dyn Differentiable> {
        Div::boxed(
            Sub::boxed(Mul::boxed(self.1.differentiate(), 
self.0.clone()), Mul::boxed(self.0.clone(), self.1.differentiate())),
            Mul::boxed(self.1.clone(), self.1.clone())
        )
    }
}

impl Differentiable for Add {
    fn differentiate(&self) -> Box<dyn Differentiable> {
        Add::boxed(self.0.differentiate(), self.1.differentiate())
    }
}

impl Differentiable for Sub {
    fn differentiate(&self) -> Box<dyn Differentiable> {
        Sub::boxed(self.0.differentiate(), self.1.differentiate())
    }
}

impl Differentiable for Cos {
    fn differentiate(&self) -> Box<dyn Differentiable> {
        Mul::boxed(Constant::boxed(-1), Sin::boxed())
    }
}

impl Differentiable for Sin {
    fn differentiate(&self) -> Box<dyn Differentiable> {
        Cos::boxed()
    }
}

fn main() {
    let fun = Div::boxed(Sin::boxed(), Cos::boxed());
    println!("{:?}", fun.differentiate());
}}

struct Model;

struct Other;

struct Producer;

trait Produces {
    type Extra;
    type Output;
}

impl Produces for Producer {
    type Output = Model;
    type Extra = Other;
}

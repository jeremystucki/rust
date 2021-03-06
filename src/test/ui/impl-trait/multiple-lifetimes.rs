// Test that multiple liftimes are allowed in impl trait types.
// compile-pass

trait X<'x>: Sized {}

impl<U> X<'_> for U {}

fn multiple_lifeteimes<'a, 'b, T: 'static>(x: &'a mut &'b T) -> impl X<'b> + 'a {
    x
}

fn main() {}

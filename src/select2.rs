use ::{Selector1, Selector2};

/// Creates a new memoizing selector with a two input functions.
///
/// If the return value of `input1` and `input2` doesn't change, `selector` will
/// not be reinvoked and a cached copy of the result will be returned instead.
pub fn new<A1, A2, I1, I2, F, R>(input1: I1, input2: I2, selector: F) -> impl Selector2<A1, A2, Output = R>
    where F: FnMut(&I1::Output, &I2::Output) -> R,
          I1: Selector1<A1>,
          I1::Output: Clone + Eq,
          I2: Selector1<A2>,
          I2::Output: Clone + Eq,
          R: Clone {
    Select2 {
        selector,
        value: None,

        input1,
        output1: None,
        input2,
        output2: None,
    }
}

struct Select2<A1, A2, I1: Selector1<A1>, I2: Selector1<A2>, F, R> {
    selector: F,
    value: Option<R>,

    input1: I1,
    output1: Option<I1::Output>,
    input2: I2,
    output2: Option<I2::Output>,
}

impl<A1, A2, F, I1, I2, R> Selector2<A1, A2> for Select2<A1, A2, I1, I2, F, R>
    where F: FnMut(&I1::Output, &I2::Output) -> R,
          I1: Selector1<A1>,
          I1::Output: Clone + Eq,
          I2: Selector1<A2>,
          I2::Output: Clone + Eq,
          R: Clone {
    type Output = R;

    fn select(&mut self, arg1: A1, arg2: A2) -> Self::Output {
        let input1 = Some(self.input1.select(arg1));
        let input2 = Some(self.input2.select(arg2));
        if input1 != self.output1 || input2 != self.output2 {
            self.value = Some((self.selector)(
                input1.as_ref().unwrap(),
                input2.as_ref().unwrap(),
            ));

            self.output1 = input1;
            self.output2 = input2;
        }

        self.value.as_ref().map(Clone::clone).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use select1::{new as new1};

    #[test]
    fn smoke() {
        let mut selector = new(
            |num: u32| num + 1,
            |num: u32| num + 2,
            |num1, num2| (*num1, *num2),
        );

        assert_eq!(selector.select(1, 2), (2, 4));
        assert_eq!(selector.select(1, 2), (2, 4));
    }

    #[test]
    fn compose() {
        let base1 = new1(|num: u32| num, |num| num * 2);
        let base2 = new1(|num: u32| num, |num| num * 3);

        let mut sel = new(base1, base2, |num1, num2| num1 + num2);

        assert_eq!(sel.select(1, 1), 5);
    }

    #[test]
    fn memo() {
        let mut computations = 0;

        {
            let mut selector = new(
                |num: u32| num + 1,
                |num: u32| num + 2,
                |num1, num2| {
                    computations += 1;
                    (*num1, *num2)
                },
            );

            assert_eq!(selector.select(1, 2), (2, 4));
            assert_eq!(selector.select(1, 2), (2, 4));
        }

        assert_eq!(computations, 1);

        {
            let mut selector = new(
                |num: u32| num + 1,
                |num: u32| num + 2,
                |num1, num2| {
                    computations += 1;
                    (*num1, *num2)
                },
            );

            assert_eq!(selector.select(1, 3), (2, 5));
            assert_eq!(selector.select(1, 3), (2, 5));
        }

        assert_eq!(computations, 2);
    }
}

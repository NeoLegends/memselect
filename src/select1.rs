use ::Selector1;

/// Creates a new memoizing selector with a single input function.
///
/// If the return value of `input1` doesn't change, `selector` will not be reinvoked
/// and a cached copy of the result will be returned instead.
pub fn new<A1, I1, F, R>(input1: I1, selector: F) -> impl Selector1<A1, Output = R>
    where F: FnMut(&I1::Output) -> R,
          I1: Selector1<A1>,
          I1::Output: Clone + Eq,
          R: Clone {
    Select1 {
        selector,
        value: None,

        input1,
        output1: None,
    }
}

struct Select1<A1, I1: Selector1<A1>, F, R> {
    selector: F,
    value: Option<R>,

    input1: I1,
    output1: Option<I1::Output>,
}

impl<A1, F, I1, R> Selector1<A1> for Select1<A1, I1, F, R>
    where F: FnMut(&I1::Output) -> R,
          I1: Selector1<A1>,
          I1::Output: Clone + Eq,
          R: Clone {
    type Output = R;

    fn select(&mut self, arg1: A1) -> Self::Output {
        let input1 = Some(self.input1.select(arg1));
        if input1 != self.output1 {
            self.value = Some((self.selector)(input1.as_ref().unwrap()));
            self.output1 = input1;
        }

        self.value.as_ref().map(Clone::clone).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::Selector1;

    #[test]
    fn smoke() {
        let mut selector = new(|num: u32| num + 2, |num| num * 2);

        assert_eq!(selector.select(2), 8);
        assert_eq!(selector.select(2), 8);
    }

    #[test]
    fn compose() {
        let base = new(|num: u32| num, |num| num * 2);
        let mut sel2 = new(base, |num| num * 2);

        assert_eq!(sel2.select(1), 4);
    }

    #[test]
    fn memo() {
        let mut computations = 0;

        {
            let mut selector = new(
                |num: u32| num + 2,
                |num| {
                    computations += 1;
                    num * 2
                }
            );

            assert_eq!(selector.select(2), 8);
            assert_eq!(selector.select(2), 8);
        }

        assert_eq!(computations, 1);

        {
            let mut selector = new(
                |num: u32| num + 2,
                |num| {
                    computations += 1;
                    num * 2
                }
            );

            assert_eq!(selector.select(3), 10);
            assert_eq!(selector.select(3), 10);
        }

        assert_eq!(computations, 2);
    }
}

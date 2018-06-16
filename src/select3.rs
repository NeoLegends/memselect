use ::{Selector1, Selector3};

/// Creates a new memoizing selector with a three input functions.
///
/// If the return value of `input1`, `input2` and `input3` doesn't change, `selector`
/// will not be reinvoked and a cached copy of the result will be returned instead.
pub fn new<A1, A2, A3, I1, I2, I3, F, R>(
    input1: I1,
    input2: I2,
    input3: I3,
    selector: F,
) -> impl Selector3<A1, A2, A3, Output = R>
    where F: FnMut(&I1::Output, &I2::Output, &I3::Output) -> R,
          I1: Selector1<A1>,
          I1::Output: Clone + Eq,
          I2: Selector1<A2>,
          I2::Output: Clone + Eq,
          I3: Selector1<A3>,
          I3::Output: Clone + Eq,
          R: Clone {
    Select3 {
        selector,
        value: None,

        input1,
        output1: None,
        input2,
        output2: None,
        input3,
        output3: None,
    }
}

struct Select3<A1, A2, A3, I1: Selector1<A1>, I2: Selector1<A2>, I3: Selector1<A3>, F, R> {
    selector: F,
    value: Option<R>,

    input1: I1,
    output1: Option<I1::Output>,
    input2: I2,
    output2: Option<I2::Output>,
    input3: I3,
    output3: Option<I3::Output>,
}

impl<A1, A2, A3, F, I1, I2, I3, R> Selector3<A1, A2, A3> for Select3<A1, A2, A3, I1, I2, I3, F, R>
    where F: FnMut(&I1::Output, &I2::Output, &I3::Output) -> R,
          I1: Selector1<A1>,
          I1::Output: Clone + Eq,
          I2: Selector1<A2>,
          I2::Output: Clone + Eq,
          I3: Selector1<A3>,
          I3::Output: Clone + Eq,
          R: Clone {
    type Output = R;

    fn select(&mut self, arg1: A1, arg2: A2, arg3: A3) -> Self::Output {
        let input1 = Some(self.input1.select(arg1));
        let input2 = Some(self.input2.select(arg2));
        let input3 = Some(self.input3.select(arg3));
        if input1 != self.output1 || input2 != self.output2 || input3 != self.output3 {
            self.value = Some((self.selector)(
                input1.as_ref().unwrap(),
                input2.as_ref().unwrap(),
                input3.as_ref().unwrap(),
            ));

            self.output1 = input1;
            self.output2 = input2;
            self.output3 = input3;
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
            |num: u32| num + 3,
            |num1, num2, num3| (*num1, *num2, *num3),
        );

        assert_eq!(selector.select(1, 2, 3), (2, 4, 6));
        assert_eq!(selector.select(1, 2, 3), (2, 4, 6));
    }

    #[test]
    fn compose() {
        let base1 = new1(|num: u32| num, |num| num * 2);
        let base2 = new1(|num: u32| num, |num| num * 3);
        let base3 = new1(|num: u32| num, |num| num * 4);

        let mut sel = new(base1, base2, base3, |num1, num2, num3| num1 + num2 + num3);

        assert_eq!(sel.select(1, 1, 1), 9);
    }

    #[test]
    fn memo() {
        let mut computations = 0;

        {
            let mut selector = new(
                |num: u32| num + 1,
                |num: u32| num + 2,
                |num: u32| num + 3,
                |num1, num2, num3| {
                    computations += 1;
                    (*num1, *num2, *num3)
                },
            );

            assert_eq!(selector.select(1, 2, 3), (2, 4, 6));
            assert_eq!(selector.select(1, 2, 3), (2, 4, 6));
        }

        assert_eq!(computations, 1);

        {
            let mut selector = new(
                |num: u32| num + 1,
                |num: u32| num + 2,
                |num: u32| num + 3,
                |num1, num2, num3| {
                    computations += 1;
                    (*num1, *num2, *num3)
                },
            );

            assert_eq!(selector.select(1, 2, 4), (2, 4, 7));
            assert_eq!(selector.select(1, 2, 4), (2, 4, 7));
        }

        assert_eq!(computations, 2);
    }
}

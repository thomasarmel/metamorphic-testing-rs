use std::fmt::Debug;

pub mod hash_metamorphic;

pub trait MetamorphicTest {
    type Input: PrimitiveInput;

    type Output: PartialEq;

    type InputMutation: Mutation<Self::Input>;

    fn output_check(output: &Self::Output, reference_output: &Self::Output, should_be_equal: bool) -> Result<(), ()>;

    fn maul(mutation: &mut Self::InputMutation) -> Option<(Self::Input, bool)> { // should_be_equal
        Some((mutation.next()?, mutation.should_be_equal()))
    }

    fn call(input: &Self::Input) -> Self::Output;

    fn test(reference_output: &Self::Output, mutation: &mut Self::InputMutation) -> Option<bool> {
        let (new_input, should_be_equal) = Self::maul(mutation)?;
        let new_output = Self::call(&new_input);
        Some(Self::output_check(reference_output, &new_output, should_be_equal).is_ok())
    }

    fn test_all(reference_output: &Self::Output, mutation: &mut Self::InputMutation) -> bool {
        while let Some(res) = Self::test(reference_output, mutation) {
            if !res {
                return false;
            }
        }
        true
    }
}

pub trait Mutation<Input> : Iterator<Item = Input> {
    const OUTPUT_SHOULD_BE_EQ: bool;
    fn should_be_equal(&self) -> bool {
        Self::OUTPUT_SHOULD_BE_EQ
    }
}

pub trait PrimitiveInput : Clone + Debug {}
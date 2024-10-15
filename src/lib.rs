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

    fn test_all(mutation: &mut Self::InputMutation) -> bool {
        let interesting_input_iterator = Self::get_interesting_input_iterator();
        for input in interesting_input_iterator {
            let reference_output = Self::call(&input);
            let mut new_input_mutation = mutation.clone_with_new_original_input(&input);
            while let Some(res) = Self::test(&reference_output, &mut new_input_mutation) {
                if !res {
                    return false;
                }
            }
        }
        true
    }

    fn get_interesting_input_iterator() -> Box<dyn Iterator<Item = Self::Input>>;
}

pub trait Mutation<Input> : Iterator<Item = Input> {
    const OUTPUT_SHOULD_BE_EQ: bool;
    fn should_be_equal(&self) -> bool {
        Self::OUTPUT_SHOULD_BE_EQ
    }

    fn clone_with_new_original_input(&self, new_original_input: &Input) -> Self;
}

pub trait PrimitiveInput : Clone + Debug {}
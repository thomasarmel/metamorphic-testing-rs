pub mod hash_metamorphic;

pub trait MetamorphicTest {
    type Input: PrimitiveInput;

    type Output: PartialEq;

    type InputMutation: Mutation<Self::Input>;

    fn output_check(output: &Self::Output, reference_output: &Self::Output, should_be_equal: bool) -> Result<(), ()>;

    fn maul(input: &Self::Input, mutation: &Self::InputMutation) -> (Self::Input, bool) { // should_be_equal
        (mutation.mutate_input(input), mutation.should_be_equal())
    }

    fn call(input: &Self::Input) -> Self::Output;

    fn test(reference_input: &Self::Input, reference_output: &Self::Output, mutation: &Self::InputMutation) -> bool {
        let (new_input, should_be_equal) = Self::maul(reference_input, mutation);
        let new_output = Self::call(&new_input);
        Self::output_check(reference_output, &new_output, should_be_equal).is_ok()
    }
}

pub trait Mutation<Input> {
    const OUTPUT_SHOULD_BE_EQ: bool;
    fn mutate_input(&self, input: &Input) -> Input;

    fn should_be_equal(&self) -> bool {
        Self::OUTPUT_SHOULD_BE_EQ
    }
}

pub trait PrimitiveInput {}
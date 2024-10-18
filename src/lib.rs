pub mod hash_metamorphic;

use std::{ops::Deref, sync::Arc};

use rayon::prelude::*;

pub struct MetamorphicTestRunner<State, Input, Output> {
    /// Generate an input from a size
    gen_input: fn(usize) -> Input,
    /// Generate an initial state
    gen_state: fn() -> State,
    /// Call the function on input and state and return an output
    call: fn(State, &Input) -> Output,
    /// Check if the output match the condition
    check: fn(&Output, &Output) -> bool,
}

impl<
        State: Clone + Send,
        Input: std::fmt::Debug + Send + Clone,
        Output: std::fmt::Debug + Send + Clone,
    > MetamorphicTestRunner<State, Input, Output>
{
    fn new(
        gen_input: fn(usize) -> Input,
        gen_state: fn() -> State,
        call: fn(State, &Input) -> Output,
        check: fn(&Output, &Output) -> bool,
    ) -> Self {
        Self {
            gen_input,
            gen_state,
            call,
            check,
        }
    }

    fn run_mutator<T: Mutator<Input, State>>(
        &self,
        initial_state: &State,
        initial_input: &Input,
        reference_output: &Output,
        mutator: &T,
        max_input_size: usize,
        collect_outputs: bool,
    ) -> (Option<Vec<(Input, Output)>>, Option<Vec<Output>>) {
        let mut collected_outputs = vec![];
        let mut collected_errors = vec![];
        for size in 1..max_input_size {
            let (mutated_state, mutated_input) =
                mutator.mutate_input(initial_input, initial_state, size);
            let new_output = (self.call)(mutated_state, &mutated_input);
            if collect_outputs {
                collected_outputs.push(new_output.clone());
            }
            if !(self.check)(&new_output, reference_output) {
                collected_errors.push((mutated_input, new_output));
            }
        }

        let errors = if !collected_errors.is_empty() {
            Some(collected_errors)
        } else {
            None
        };

        let outputs = if collect_outputs {
            Some(collected_outputs)
        } else {
            None
        };

        (errors, outputs)
    }

    pub fn run_test<T: Mutator<Input, State> + Send + Sync>(
        &self,
        max_size: usize,
        test_name: &str,
        lib_name: &str,
        mutator: T,
    ) {
        let shared_mutator = Arc::new(mutator.clone());
        let number_of_errors :usize = (1..max_size)
            .into_par_iter()
            .map(|size| {
                let input = (self.gen_input)(size);
                println!(
                    "[{}] Running {} with {} bytes input size",
                    lib_name, test_name, size
                );
                let new_mutator = Arc::clone(&shared_mutator);
                let ref_output = (self.call)((self.gen_state)(), &input);
                let (errors, _) = self.run_mutator(
                    &(self.gen_state)(),
                    &input,
                    &ref_output,
                    new_mutator.deref(),
                    size,
                    false,
                );

                match errors {
                    Some(err) =>  {
                        for e in &err {
                            println!(
                                "[{}] ! ERROR ! Running {} on size {} : reference input {:?}, reference output {:?}, input {:?}, output {:?}",
                                lib_name, test_name, size, input, ref_output, e.0, e.1
                            );
                        }
                        err.len()
                    },
                    None => 0,
                }
            }).sum();

        println!(
            "[{}] !SUMMARY ! {} with 1-{} bytes : found {} errors",
            lib_name, test_name, max_size, number_of_errors
        );
    }
}

pub trait HashMetamorphic {
    type Input: std::fmt::Debug + Clone + Send;
    type Output: std::fmt::Debug + Clone + Send + PartialEq;
    type State: Clone + Send;

    const LIBNAME: &str;

    fn initial_state() -> Self::State;
    fn gen_input(size: usize) -> Self::Input;
    fn input_as_u8(input: &Self::Input) -> Vec<u8>;

    /// Return unmodified initial state and mutated as input
    fn u8_as_input(
        initial_state: &Self::State,
        _initial_input: &Self::Input,
        mutated: Vec<u8>,
    ) -> (Self::State, Self::Input);

    fn hash_update(
        initial_state: &Self::State,
        _initial_input: &Self::Input,
        first_part: &[u8],
        second_part: &[u8],
    ) -> (Self::State, Self::Input);

    fn output_as_u8(output: &Self::Output) -> Vec<u8>;
    fn compare_output(initial_output: &Self::Output, output: &Self::Output) -> bool;
    fn hash(state: Self::State, input: &Self::Input) -> Self::Output;

    fn bit_inclusion_test(max_size: usize) {
        let mutator = BitInclusionMutator::new(Self::input_as_u8, Self::u8_as_input);
        let runner = MetamorphicTestRunner::new(
            Self::gen_input,
            Self::initial_state,
            Self::hash,
            |reference_output, output| reference_output != output,
        );
        runner.run_test(max_size, "Bit Inclusion", Self::LIBNAME, mutator);
    }

    fn update_hash_test(max_size: usize) {
        let mutator = UpdateMutator::new(Self::input_as_u8, Self::hash_update);
        let runner = MetamorphicTestRunner::new(
            Self::gen_input,
            Self::initial_state,
            Self::hash,
            |reference_output, output| reference_output == output,
        );

        runner.run_test(max_size, "Update Hash", Self::LIBNAME, mutator);
    }

    fn run_tests(max_size: usize) {
        Self::bit_inclusion_test(max_size);
        Self::update_hash_test(max_size);
    }
}

pub trait KEMMetamorphic {
    type Input;
    type Output;
    type State;

    fn gen_input(msg_size: usize) -> Self::Input;
    fn get_key_from_input_as_u8(input: Self::Input) -> Vec<u8>;
    fn get_msg_from_input_as_u8(input: Self::Input) -> Vec<u8>;
    fn set_key_from_input_as_u8(input: Self::Input, key: Vec<u8>) -> Self::Input;
    fn set_msg_from_input_as_u8(input: Self::Input, msg: Vec<u8>) -> Self::Input;
    fn output_as_u8(output: Self::Output) -> Vec<u8>;
}

pub trait Mutator<I: Clone, S: Clone>: Clone {
    fn mutate_input(&self, input: &I, initial_state: &S, element_to_mutate: usize) -> (S, I);
}

#[derive(Clone)]
pub struct BitInclusionMutator<I: Clone, S: Clone> {
    extract_mutable: fn(&I) -> Vec<u8>,
    include_mutated: fn(&S, &I, Vec<u8>) -> (S, I),
}

impl<I: Clone, S: Clone> BitInclusionMutator<I, S> {
    fn new(
        extract_mutable: fn(&I) -> Vec<u8>,
        include_mutated: fn(&S, &I, Vec<u8>) -> (S, I),
    ) -> Self {
        Self {
            extract_mutable,
            include_mutated,
        }
    }
}

impl<I: Clone, S: Clone> Mutator<I, S> for BitInclusionMutator<I, S> {
    fn mutate_input(&self, input: &I, initial_state: &S, element_to_mutate: usize) -> (S, I) {
        let mut mutable_part = (self.extract_mutable)(input);
        flip_one_bit(&mut mutable_part, element_to_mutate);
        (self.include_mutated)(initial_state, input, mutable_part)
    }
}

#[derive(Clone)]
pub struct UpdateMutator<I: Clone, S: Clone> {
    extract_mutable: fn(&I) -> Vec<u8>,
    include_mutated: fn(&S, &I, &[u8], &[u8]) -> (S, I),
}

impl<I: Clone, S: Clone> UpdateMutator<I, S> {
    fn new(
        extract_mutable: fn(&I) -> Vec<u8>,
        include_mutated: fn(&S, &I, &[u8], &[u8]) -> (S, I),
    ) -> Self {
        Self {
            extract_mutable,
            include_mutated,
        }
    }
}
impl<I: Clone, S: Clone> Mutator<I, S> for UpdateMutator<I, S> {
    fn mutate_input(&self, input: &I, initial_state: &S, element_to_mutate: usize) -> (S, I) {
        let mutable_part = (self.extract_mutable)(input);
        // Splitting on index
        (self.include_mutated)(
            initial_state,
            input,
            &mutable_part[0..element_to_mutate],
            &mutable_part[element_to_mutate..],
        )
    }
}

pub fn set_one_bit(input: &mut [u8], idx: usize, value: bool) {
    if idx >= input.len() * 8 {
        panic!("index error");
    }
    if value {
        input[idx / 8] |= 1 << (7 - (idx % 8));
    } else {
        input[idx / 8] &= !(1 << (7 - (idx % 8)));
    }
}

pub fn flip_one_bit(input: &mut [u8], idx: usize) {
    if idx >= input.len() * 8 {
        panic!("index error");
    }
    input[idx / 8] ^= 1 << (7 - (idx % 8));
}

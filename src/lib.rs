pub mod hash_metamorphic;
use rayon::prelude::*;

pub trait MetamorphicTest {
    type Input: std::fmt::Debug;
    type Output: std::fmt::Debug;
    type State;

    fn gen_input(size: usize) -> Self::Input;

    fn gen_state() -> Self::State;

    fn output_check(
        output: &Self::Output,
        reference_output: &Self::Output,
        should_be_equal: bool,
    ) -> bool;

    fn call(state: Self::State, input: &Self::Input) -> Self::Output;

    fn convert_output_as_u8(output: &Self::Output) -> Vec<u8>;

    fn run_mutator<T: Mutator<Self::Input, Self::State>>(
        reference_output: &Self::Output,
        mutator: &mut T,
        collect_outputs: bool,
    ) -> (
        Option<Vec<(Self::Input, Self::Output)>>,
        Option<Vec<Vec<u8>>>,
    ) {
        let mut collected_outputs = vec![];
        let mut collected_errors = vec![];
        let should_be_equal = mutator.should_be_equal();
        for (state, input) in mutator {
            let new_output = Self::call(state, &input);
            if collect_outputs {
                collected_outputs.push(Self::convert_output_as_u8(&new_output));
            }
            if !Self::output_check(reference_output, &new_output, should_be_equal) {
                collected_errors.push((input, new_output));
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

    fn run_test<T: Mutator<Self::Input, Self::State>>(max_size: usize, mutator: &mut T) -> bool {
        for s in 1..max_size {
            let input = Self::gen_input(s);
            println!("Running bit inclusion with {} bytes input size", s);
            let ref_output = Self::call(Self::gen_state(), &input);
            mutator.initial_input(input, Self::gen_state(), s * 8);

            let (res, _) = Self::run_mutator(&ref_output, mutator, false);

            if let Some(x) = res {
                println!("Found errors : {:?}", &x);
                return false;
            }
        }

        true
    }

    fn run_tests(max_size: usize);
}

pub trait HashMetamorphic {
    type Input: std::fmt::Debug;
    type Output: std::fmt::Debug;
    type State;

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

    fn bit_inclusion_mutator(
        input: Self::Input,
        state: Self::State,
        max_index: usize,
    ) -> BitInclusionMutator<Self::Input, Self::State> {
        BitInclusionMutator::new(
            input,
            state,
            max_index,
            Self::input_as_u8,
            Self::u8_as_input,
        )
    }

    fn update_mutator(
        input: Self::Input,
        state: Self::State,
        max_index: usize,
    ) -> UpdateMutator<Self::Input, Self::State> {
        UpdateMutator::new(
            input,
            state,
            max_index,
            Self::input_as_u8,
            Self::hash_update,
        )
    }
}

impl<T: HashMetamorphic> MetamorphicTest for T {
    type Input = T::Input;
    type Output = T::Output;
    type State = T::State;

    fn output_check(
        output: &Self::Output,
        reference_output: &Self::Output,
        should_be_equal: bool,
    ) -> bool {
        let out = T::output_as_u8(output);
        let ref_out = T::output_as_u8(reference_output);
        (out == ref_out) == should_be_equal
    }

    fn call(state: Self::State, input: &Self::Input) -> Self::Output {
        T::hash(state, input)
    }

    fn convert_output_as_u8(output: &Self::Output) -> Vec<u8> {
        T::output_as_u8(output)
    }

    fn run_tests(max_size: usize) {
        // Bit inclusion test
        (1..max_size).into_par_iter().for_each(|s| {
            let input = Self::gen_input(s);
            println!(
                "[{}] Running bit inclusion with {} bytes input size",
                Self::LIBNAME,
                s
            );
            let ref_output = T::call(Self::initial_state(), &input);
            let mut bit_inclusion_mutator =
                Self::bit_inclusion_mutator(input, Self::initial_state(), s * 8);

            let (res, _) = Self::run_mutator(&ref_output, &mut bit_inclusion_mutator, false);

            if let Some(x) = res {
                println!("Found errors : {:?}", &x);
            }
        });

        // Update test
        (1..max_size).into_par_iter().for_each(|s| {
            let input = Self::gen_input(s);
            println!(
                "[{}] Running update test with {} bytes input size",
                Self::LIBNAME,
                s
            );
            let ref_output = T::call(Self::initial_state(), &input);
            let mut update_mutator = Self::update_mutator(input, Self::initial_state(), s);

            let (res, _) = Self::run_mutator(&ref_output, &mut update_mutator, false);

            if let Some(x) = res {
                println!("Found errors : {:?}", &x);
            }
        });
    }

    fn gen_input(size: usize) -> Self::Input {
        T::gen_input(size)
    }

    fn gen_state() -> Self::State {
        T::initial_state()
    }
}

pub trait DSSMetamorphic {
    type Input;
    type Output;

    fn gen_input(msg_size: usize) -> Self::Input;
    fn get_key_from_input_as_u8(input: Self::Input) -> Vec<u8>;
    fn get_msg_from_input_as_u8(input: Self::Input) -> Vec<u8>;
    fn set_key_from_input_as_u8(input: Self::Input, key: Vec<u8>) -> Self::Input;
    fn set_msg_from_input_as_u8(input: Self::Input, msg: Vec<u8>) -> Self::Input;
    fn output_as_u8(output: Self::Output) -> Vec<u8>;
}

pub trait Mutator<I, S>: Iterator<Item = (S, I)> {
    fn initial_input(&mut self, input: I, initial_state: S, max_index: usize);

    fn mutate_input(&self) -> (S, I);

    fn should_be_equal(&self) -> bool;
}

pub struct BitInclusionMutator<I, S> {
    input: I,
    initial_state: S,
    index: usize,
    max_index: usize,
    extract_mutable: fn(&I) -> Vec<u8>,
    include_mutated: fn(&S, &I, Vec<u8>) -> (S, I),
}

impl<I, S> BitInclusionMutator<I, S> {
    fn new(
        input: I,
        initial_state: S,
        max_index: usize,
        extract_mutable: fn(&I) -> Vec<u8>,
        include_mutated: fn(&S, &I, Vec<u8>) -> (S, I),
    ) -> Self {
        Self {
            input,
            initial_state,
            index: 0,
            max_index,
            extract_mutable,
            include_mutated,
        }
    }
}

impl<I, S> Mutator<I, S> for BitInclusionMutator<I, S> {
    fn mutate_input(&self) -> (S, I) {
        let mut mutable_part = (self.extract_mutable)(&self.input);
        flip_one_bit(&mut mutable_part, self.index);
        (self.include_mutated)(&self.initial_state, &self.input, mutable_part)
    }

    fn should_be_equal(&self) -> bool {
        false
    }

    fn initial_input(&mut self, input: I, initial_state: S, max_index: usize) {
        self.input = input;
        self.initial_state = initial_state;
        self.max_index = max_index;
        self.index = 0;
    }
}

impl<I, S> Iterator for BitInclusionMutator<I, S> {
    type Item = (S, I);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.max_index {
            return None;
        }
        let res = Some(self.mutate_input());
        self.index += 1;

        res
    }
}

pub struct UpdateMutator<I, S> {
    input: I,
    hasher: S,
    index: usize,
    max_index: usize,
    extract_mutable: fn(&I) -> Vec<u8>,
    include_mutated: fn(&S, &I, &[u8], &[u8]) -> (S, I),
}

impl<I, S> UpdateMutator<I, S> {
    fn new(
        input: I,
        initial_state: S,
        max_index: usize,
        extract_mutable: fn(&I) -> Vec<u8>,
        include_mutated: fn(&S, &I, &[u8], &[u8]) -> (S, I),
    ) -> Self {
        Self {
            input,
            hasher: initial_state,
            index: 0,
            max_index,
            extract_mutable,
            include_mutated,
        }
    }
}
impl<I, S> Mutator<I, S> for UpdateMutator<I, S> {
    fn initial_input(&mut self, input: I, initial_state: S, max_index: usize) {
        self.input = input;
        self.hasher = initial_state;
        self.max_index = max_index;
        self.index = 0;
    }

    fn mutate_input(&self) -> (S, I) {
        let mutable_part = (self.extract_mutable)(&self.input);
        // Splitting on index
        (self.include_mutated)(
            &self.hasher,
            &self.input,
            &mutable_part[0..self.index],
            &mutable_part[self.index..],
        )
    }

    fn should_be_equal(&self) -> bool {
        true
    }
}

impl<I, S> Iterator for UpdateMutator<I, S> {
    type Item = (S, I);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.max_index {
            return None;
        }
        let res = Some(self.mutate_input());
        self.index += 1;

        res
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
    // println!("{:?} << {}", input, idx);
    if idx >= input.len() * 8 {
        panic!("index error");
    }
    input[idx / 8] ^= 1 << (7 - (idx % 8));
    // println!("{:?}", input);
}

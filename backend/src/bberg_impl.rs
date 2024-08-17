use std::io::{self, Read, Write};

use crate::{BackendImpl, BackendImplWithSetup, Proof};
use ast::analyzed::Analyzed;
use bberg::bberg_codegen::BBergCodegen;
use number::{DegreeType, FieldElement};

/// Implementation of the `BackendImpl` trait for the `BBergCodegen`.
/// This does not produce proofs directly but handles code generation.
impl<T: FieldElement> BackendImpl<T> for BBergCodegen {
    fn new(degree: DegreeType) -> Self {
        BBergCodegen::assert_field_is_compatible::<T>();
        BBergCodegen::new(degree)
    }

    fn prove(
        &self,
        pil: &Analyzed<T>,
        fixed: &[(String, Vec<T>)],
        witness: &[(String, Vec<T>)],
        _prev_proof: Option<Proof>,
        bname: Option<String>,
    ) -> (Option<Proof>, Option<String>) {
        self.build_ast(pil, fixed, witness, bname);

        // Note: Currently, `BBergCodegen` does not produce proofs as it focuses on C++ code generation.
        // This may change in the future when the library becomes more pluggable.
        (None, None)
    }
}

/// Implementation of the `BackendImplWithSetup` trait for the `BBergCodegen`.
impl<T: FieldElement> BackendImplWithSetup<T> for BBergCodegen {
    fn new_from_setup(mut input: &mut dyn Read) -> Result<Self, io::Error> {
        BBergCodegen::assert_field_is_compatible::<T>();
        BBergCodegen::new_from_setup(&mut input)
    }

    // This method should write the setup data to the provided output stream.
    fn write_setup(&self, _output: &mut dyn Write) -> Result<(), io::Error> {
        // Placeholder implementation. Uncomment the following line when the actual method is implemented.
        // self.write_setup(&mut output)
        Ok(())
    }
}

/// Mock backend for testing purposes.
/// This mock does not perform actual proof generation or aggregation.
pub struct BBergMock;

impl<T: FieldElement> BackendImpl<T> for BBergMock {
    fn new(_degree: DegreeType) -> Self {
        Self
    }

    fn prove(
        &self,
        _pil: &Analyzed<T>,
        _fixed: &[(String, Vec<T>)],
        _witness: &[(String, Vec<T>)],
        prev_proof: Option<Proof>,
        _bname: Option<String>,
    ) -> (Option<Proof>, Option<String>) {
        if prev_proof.is_some() {
            unimplemented!("BBergMock backend does not support aggregation");
        }

        // TODO: Implement a mock prover for testing.
        unimplemented!("BBergMock backend is not implemented");
    }
}

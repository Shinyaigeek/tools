use super::*;
use rslint_parser::{parse, Syntax};
use std::path::Path;

use crate::runner::{TestCase, TestRunOutcome, TestSuite};

const CASES_PATH: &str = "xtask/coverage/Typescript/tests/cases";
const REFERENCE_PATH: &str = "xtask/coverage/Typescript/tests/baselines/reference";

#[derive(Debug)]
struct TypeScriptTestCase {
    code: String,
    path: PathBuf,
}

impl TestCase for TypeScriptTestCase {
    fn path(&self) -> &Path {
        self.path.strip_prefix(CASES_PATH).unwrap()
    }

    fn code(&self) -> &str {
        &self.code
    }

    fn run(&self) -> TestRunOutcome {
        let syntax = Syntax::default().typescript();
        let r = parse(self.code(), 0, syntax);

        let error_reference_file = Path::new(REFERENCE_PATH)
            .join(self.path.with_extension("errors.txt").file_name().unwrap());

        let expected_errors = error_reference_file.exists();

        match r.ok() {
            Err(errors) if !expected_errors => {
                TestRunOutcome::IncorrectlyErrored { errors, syntax }
            }
            Err(_) => TestRunOutcome::Passed(syntax),
            _ if expected_errors => TestRunOutcome::IncorrectlyPassed(syntax),
            _ => TestRunOutcome::Passed(syntax),
        }
    }
}

#[derive(Default)]
pub(crate) struct TypeScriptTestSuite;

impl TestSuite for TypeScriptTestSuite {
    fn name(&self) -> &str {
        "TS"
    }

    fn base_path(&self) -> &str {
        CASES_PATH
    }

    fn is_test(&self, path: &Path) -> bool {
        match path.extension() {
            None => false,
            Some(ext) => ext == "ts",
        }
    }

    fn load_test(&self, path: PathBuf) -> Option<Box<dyn TestCase>> {
        let code = check_file_encoding(&path)?;

        Some(Box::new(TypeScriptTestCase { path, code }))
    }
}

pub fn check_file_encoding(path: &std::path::Path) -> Option<String> {
    let buffer = std::fs::read(path).unwrap();
    let bom = buffer.get(0..3);
    //Utf16Be or // Utf16Le
    if let Some(&[0xfe, 0xff, _] | &[0xff, 0xfe, _]) = bom {
        None
    } else {
        std::str::from_utf8(buffer.as_slice())
            .ok()
            .map(str::to_string)
    }
}

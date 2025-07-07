use std::{
    collections::HashMap,
    env, fs,
    panic::{self, UnwindSafe},
};

use camino::Utf8PathBuf as PathBuf;
use pretty_assertions::assert_eq;

#[test]
fn err() -> crambly::Result<()> {
    nopanic(|| {
        crambly::cram_internal(
            env!(concat!("CARGO_BIN_EXE_", "crambly")),
            "err",
            env!("CARGO_TARGET_TMPDIR"),
            true,
        )
    })?;
    let dir = &PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("tests/err");
    let mut tests: HashMap<String, (Option<String>, Option<String>)> =
        HashMap::new();
    for entry in fs::read_dir(dir)? {
        let path = &PathBuf::try_from(entry?.path())?;
        if path.extension() == Some("test") {
            let _ = tests
                .entry(path.file_stem().expect("Bad test file path").to_owned())
                .or_default();
        }
        if path.extension() == Some("err") {
            let contents = fs::read_to_string(path)?;
            let (err, _) = tests
                .entry(path.file_stem().expect("Bad err file path").to_owned())
                .or_default();
            assert!(
                err.is_none(),
                "Duplicate err file for {}",
                path.file_stem().unwrap()
            );
            *err = Some(contents);
        }
        if path.extension() == Some("expected") {
            let contents = fs::read_to_string(path)?;
            let (_, expected) = tests
                .entry(
                    path.file_stem()
                        .expect("Bad expected file path")
                        .to_owned(),
                )
                .or_default();
            assert!(
                expected.is_none(),
                "Duplicate expected file for {}",
                path.file_stem().unwrap()
            );
            *expected = Some(contents);
        }
    }
    for (test, (err, expected)) in tests {
        match (err, expected) {
            (Some(err), Some(expected)) => {
                assert_eq!(expected, err, "Error in test case {test}");
                fs::remove_file(dir.join(format!("{test}.err")))?;
            }
            (None, _) => {
                panic!("Test case {test} didn't generate a .err file")
            }
            (Some(_), None) => (),
        }
    }

    Ok(())
}

fn nopanic(
    f: impl FnOnce() -> crambly::Result<()> + UnwindSafe,
) -> crambly::Result<()> {
    let hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = match panic::catch_unwind(f) {
        Ok(r) => r,
        Err(_) => Ok(()),
    };
    panic::set_hook(hook);
    result
}

//! crambly
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_results)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::let_underscore_untyped)]
#![allow(clippy::similar_names)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

use std::{collections::HashMap, env, fs, process::Command};

use anyhow::anyhow;
use camino::Utf8Path as Path;
#[cfg(test)]
use pretty_assertions as _;

use self::context::Context;

mod context;
mod test;
mod utils;
mod walk;

pub use anyhow::Result;
use test::{
    Test,
    formats::{Cram, Crambly},
};

/// Run cram tests
#[macro_export]
macro_rules! cram {
    ($exe:literal, $test_dir:expr) => {
        #[test]
        fn cram() -> $crate::Result<()> {
            $crate::cram_internal(
                env!(concat!("CARGO_BIN_EXE_", $exe)),
                $test_dir,
                env!("CARGO_TARGET_TMPDIR"),
                false,
            )
        }
    };
}

#[doc(hidden)]
#[expect(unsafe_code)]
pub fn cram_internal(
    exe: &str,
    test_dir: impl AsRef<Path>,
    tmp_dir: impl AsRef<Path>,
    no_output: bool,
) -> Result<()> {
    let Context {
        exe,
        test_dir,
        tmp_dir,
    } = Context::new(exe, test_dir, tmp_dir)?;

    walk::walk(&test_dir, |path| {
        if path.extension() != Some("test") {
            return Ok(());
        }
        let test = Test::read::<Crambly>(path)?;
        let uniq = path.strip_prefix(&test_dir)?;

        let input_base = tmp_dir.join("inputs");
        let mut vars = HashMap::new();
        for (var, path, contents) in test.inputs(&input_base, uniq) {
            fs::create_dir_all(
                path.parent()
                    .ok_or_else(|| anyhow!("Malformed input directory"))?,
            )?;
            fs::write(&path, contents)?;
            let old = vars.insert(var, path);
            debug_assert!(old.is_none());
        }

        let mut dest = tmp_dir.join("tests").join(uniq);
        let _ = dest.set_extension("t");
        fs::create_dir_all(
            dest.parent()
                .ok_or_else(|| anyhow!("Malformed test directory"))?,
        )?;
        fs::write(dest, test.render::<Cram>(vars)?)?;
        Ok(())
    })?;

    // SAFETY: This is just a test
    unsafe {
        env::set_var("EXE", exe);
    }
    let cram_status = if no_output {
        Command::new("cram").arg(&tmp_dir).output()?.status
    } else {
        Command::new("cram").arg(&tmp_dir).status()?
    };

    walk::walk(&tmp_dir, |path| {
        if path.extension() != Some("err") {
            return Ok(());
        }
        let test = Test::read::<Cram>(path)?;
        fs::write(test.err_path(), test.render::<Crambly>(())?)?;
        Ok(())
    })?;

    fs::remove_dir_all(&tmp_dir)?;
    assert!(cram_status.success());

    Ok(())
}

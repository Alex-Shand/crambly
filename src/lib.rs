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

use std::{fs, process::Command};

use anyhow::anyhow;
use camino::Utf8Path as Path;

use self::context::Context;

mod context;
mod test;
mod walk;

pub use anyhow::Result;
use test::Test;

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
            )
        }
    };
}

#[doc(hidden)]
pub fn cram_internal(
    exe: &str,
    test_dir: impl AsRef<Path>,
    tmp_dir: impl AsRef<Path>,
) -> Result<()> {
    let Context {
        exe,
        test_dir,
        tmp_dir,
    } = Context::new(exe, test_dir, tmp_dir)?;

    walk::walk(&test_dir, |path| {
        if path.extension() == Some("err") {
            return Ok(());
        }
        let stripped_path = path.strip_prefix(&test_dir)?;
        let test = Test::read(stripped_path, fs::read_to_string(path)?)?;
        let mut dest = tmp_dir.join(stripped_path);
        let _ = dest.set_extension("t");
        fs::create_dir_all(
            dest.parent()
                .ok_or_else(|| anyhow!("Malformed test directory"))?,
        )?;
        fs::write(dest, test.render())?;
        Ok(())
    })?;

    let cram_status = Command::new("cram").arg(&tmp_dir).status()?;

    walk::walk(&tmp_dir, |path| {
        if path.extension() != Some("err") {
            return Ok(());
        }
        let test = Test::read_cram(fs::read_to_string(path)?);
        let original = fs::read_to_string(test_dir.join(test.path()))?;
        fs::write(test_dir.join(test.err_path()), test.render_err(&original))?;
        Ok(())
    })?;

    fs::remove_dir_all(&tmp_dir)?;
    assert!(cram_status.success());

    Ok(())

    // // SAFETY: This is just a test
    // unsafe {
    //     env::set_var("EXE", exe);
    // }
    // let cram_status = Command::new("cram").arg(tmp).status()?;

    // walk(tmp, |path| {
    //     match path.extension().and_then(OsStr::to_str) {
    //         Some("err") => (),
    //         _ => return Ok(()),
    //     }
    //     let dest = test_dir.join(path.with_extension("").strip_prefix(tmp)?);
    //     if let Some("cram") =
    //         dest.with_extension("").extension().and_then(OsStr::to_str)
    //     {
    //         untranslate(path, dest.with_extension("err"))?;
    //     } else {
    //         let mut dest = dest.into_os_string();
    //         dest.push(".err");
    //         let _ = fs::copy(path, dest)?;
    //     }
    //     Ok(())
    // })?;

    // fs::remove_dir_all(tmp)?;
    // assert!(cram_status.success());
    // Ok(())
}

// fn translate(src: &Path, dest: PathBuf) -> Result {
//     let result = fs::read_to_string(src)?
//         .lines()
//         .map(|line| {
//             if line.starts_with('#') {
//                 format!("{line}\n")
//             } else {
//                 format!("  {line}\n")
//             }
//         })
//         .collect::<String>();

//     let mut dest = dest.into_os_string();
//     dest.push(".t");
//     fs::write(dest, result)?;

//     Ok(())
// }

// fn untranslate(src: &Path, dest: PathBuf) -> Result {
//     let result = fs::read_to_string(src)?
//         .lines()
//         .map(|line| {
//             if line.starts_with("  ") {
//                 line.chars()
//                     .skip(2)
//                     .chain(iter::once('\n'))
//                     .collect::<String>()
//             } else if !line.starts_with('#') {
//                 format!("# {line}\n")
//             } else {
//                 format!("{line}\n")
//             }
//         })
//         .collect::<String>();

//     fs::write(dest, result)?;

//     Ok(())
// }

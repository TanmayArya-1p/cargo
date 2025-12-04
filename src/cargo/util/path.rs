use crate::core::manifest::TargetKind;
use crate::util::errors::CargoResult;
use std::path::Path;

// Checks if a target path exists and is a source file, not a directory
pub fn validate_target_path_as_source_file(
    path: &Path,
    target_name: &str,
    target_kind: &TargetKind,
) -> CargoResult<()> {
    if !path.exists() {
        anyhow::bail!(
            "can't find {} `{}` at path `{}`",
            target_kind.description(),
            target_name,
            path.display()
        );
    }

    if path.is_dir() {
        let main_rs = path.join("main.rs");
        let lib_rs = path.join("lib.rs");

        let suggested_files_opt = match (main_rs.exists(), lib_rs.exists()) {
            (true, true) => Some(format!("`{}` or `{}`", main_rs.display(), lib_rs.display())),
            (true, false) => Some(format!("`{}`", main_rs.display())),
            (false, true) => Some(format!("`{}`", lib_rs.display())),
            (false, false) => None,
        };

        // If the path is likely a crate, then suggest setting the path to the entrypoint
        if let Some(suggested_files) = suggested_files_opt {
            anyhow::bail!(
                "path `{}` for {} `{}` is a directory, but a source file was expected.\n\
                help: specify the path to the intended entrypoint file instead: {}",
                path.display(),
                target_kind.description(),
                target_name,
                suggested_files,
            );
        } else {
            anyhow::bail!(
                "path `{}` for {} `{}` is a directory, but a source file was expected.",
                path.display(),
                target_kind.description(),
                target_name
            );
        }
    }

    Ok(())
}

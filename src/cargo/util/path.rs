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


        // suggest setting the path to a likely entrypoint
        let main_rs = path.join("main.rs");
        let lib_rs = path.join("lib.rs");
        
        let suggested_files_opt = match target_kind {
            TargetKind::Lib(_) => if lib_rs.exists() { Some(format!("`{}`", lib_rs.display())) } else { None },
            TargetKind::Bin => if main_rs.exists() { Some(format!("`{}`", main_rs.display())) } else { None },
            TargetKind::Test => if main_rs.exists() { Some(format!("`{}`", main_rs.display())) } else { None },
            TargetKind::ExampleBin => if main_rs.exists() { Some(format!("`{}`", main_rs.display())) } else { None },
            TargetKind::Bench => if main_rs.exists() { Some(format!("`{}`", main_rs.display())) } else { None },
            TargetKind::ExampleLib(_) => if lib_rs.exists() { Some(format!("`{}`", lib_rs.display())) } else { None },
            _ => None,
        };

        let err_msg = format!(
            "path `{}` for {} `{}` is a directory, but a source file was expected.",
            path.display(),
            target_kind.description(),
            target_name
        );

        if let Some(suggested_files) = suggested_files_opt {
            anyhow::bail!(
                "{}\n\
                help: specify the path to the intended entrypoint file instead: {}",
                err_msg,
                suggested_files,
            );
        }

        anyhow::bail!(err_msg);
    }

    Ok(())
}

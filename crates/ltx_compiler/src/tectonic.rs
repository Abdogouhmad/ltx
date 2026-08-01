use miette::Result as MResult;
use std::path::Path;
use tectonic::driver::{OutputFormat, PassSetting, ProcessingSessionBuilder};
use tectonic::status::ChatterLevel;
use tectonic::status::termcolor::TermcolorStatusBackend;
use tectonic_bundles::get_fallback_bundle;

/// tectonic function that compiles a source `.tex` file into a PDF.
///
/// # Arguments
///
/// * `input_path` - Path to the main `.tex` file to compile.
/// * `output_name` - Name of the output PDF file, without the `.pdf` extension.
/// * `output_dir` - Directory where the compiled PDF is written.
///
/// # Errors
///
/// Returns an error if the TeX Live bundle cannot be fetched or the
/// compilation itself fails.
pub fn tectonic_compile(input_path: &Path, output_name: &str, output_dir: &Path) -> MResult<()> {
    let mut status = TermcolorStatusBackend::new(ChatterLevel::Normal);

    // Fetch and cache tectonic's default TeX Live support files bundle.
    let bundle = get_fallback_bundle(tectonic::FORMAT_SERIAL, false)
        .map_err(|e| miette::miette!("failed to fetch the TeX Live support bundle: {e}"))?;

    let mut the_build = ProcessingSessionBuilder::default();

    the_build
        .bundle(bundle)
        .primary_input_path(input_path)
        .tex_input_name(&format!("{output_name}.tex"))
        .output_dir(output_dir)
        .output_format(OutputFormat::Pdf)
        .format_name("latex")
        .pass(PassSetting::Default)
        .keep_logs(true)
        .keep_intermediates(false)
        .synctex(true)
        .build_date_from_env(true);

    let mut sess = the_build
        .create(&mut status)
        .map_err(|e| miette::miette!("failed to create the tectonic session: {e}"))?;
    sess.run(&mut status)
        .map_err(|e| miette::miette!("tectonic compilation failed: {e}"))?;

    println!(
        "compiled -> {}",
        output_dir.join(format!("{output_name}.pdf")).display()
    );
    Ok(())
}

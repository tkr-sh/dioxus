use dioxus_cli_config::Platform;

use super::*;
use std::{fs::create_dir_all, io::Write, path::PathBuf};

/// Run the WASM project on dev-server
#[derive(Clone, Debug, Parser)]
#[clap(name = "serve")]
pub struct Serve {
    #[clap(flatten)]
    pub serve: ConfigOptsServe,
}

impl Serve {
    pub async fn serve(self, bin: Option<PathBuf>) -> Result<()> {
        let mut crate_config = dioxus_cli_config::CrateConfig::new(bin)?;
        let serve_cfg = self.serve.clone();

        // change the relase state.
        crate_config.with_hot_reload(self.serve.hot_reload);
        crate_config.with_cross_origin_policy(self.serve.cross_origin_policy);
        crate_config.with_release(self.serve.release);
        crate_config.with_verbose(self.serve.verbose);

        if self.serve.example.is_some() {
            crate_config.as_example(self.serve.example.unwrap());
        }

        if self.serve.profile.is_some() {
            crate_config.set_profile(self.serve.profile.unwrap());
        }

        if self.serve.features.is_some() {
            crate_config.set_features(self.serve.features.unwrap());
        }

        if let Some(target) = self.serve.target {
            crate_config.set_target(target);
        }

        crate_config.set_cargo_args(self.serve.cargo_args);

        let platform = self
            .serve
            .platform
            .unwrap_or(crate_config.dioxus_config.application.default_platform);

        match platform {
            Platform::Web => {
                // generate dev-index page
                Serve::regen_dev_page(&crate_config, self.serve.skip_assets)?;

                // start the develop server
                server::web::startup(
                    self.serve.port,
                    crate_config.clone(),
                    self.serve.open,
                    self.serve.skip_assets,
                )
                .await?;
            }
            Platform::Desktop => {
                server::desktop::startup(crate_config.clone(), &serve_cfg).await?;
            }
            Platform::Fullstack => {
                server::fullstack::startup(crate_config.clone(), &serve_cfg).await?;
            }
        }
        Ok(())
    }

    pub fn regen_dev_page(crate_config: &CrateConfig, skip_assets: bool) -> Result<()> {
        let serve_html = gen_page(crate_config, true, skip_assets);

        let dist_path = crate_config
            .crate_dir
            .join(crate_config.dioxus_config.application.out_dir.clone());
        if !dist_path.is_dir() {
            create_dir_all(&dist_path)?;
        }
        let index_path = dist_path.join("index.html");
        let mut file = std::fs::File::create(index_path)?;
        file.write_all(serve_html.as_bytes())?;

        Ok(())
    }
}

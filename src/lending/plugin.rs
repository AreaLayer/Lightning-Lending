use cln_plugin::Builder;
use cln_plugin::Plugin;

fn mainn() {
    Builder::new("lending")
        .with_plugin_path("src/lending/plugin.rs")
        .build()
        .unwrap();
}

impl Plugin for Lending {
    fn connect_handler(
        &self,
        _: &mut cln_plugin::PluginContext,
        _: &mut cln_plugin::PluginRequest,
        _: &mut cln_plugin::PluginResponse,
    ) -> Result<(), cln_plugin::PluginError> {
        Ok(())
    }
}

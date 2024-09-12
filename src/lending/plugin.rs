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

impl Lending {
    fn new() -> Self {
        Self
    }
}

#[no_mangle]
pub extern "C" fn plugin_init(
    context: *mut cln_plugin::PluginContext,
    request: *mut cln_plugin::PluginRequest,
    response: *mut cln_plugin::PluginResponse,
) -> Result<(), cln_plugin::PluginError> {
    let context = unsafe { &mut *context };
    let request = unsafe { &mut *request };
    let response = unsafe { &mut *response };
    let plugin = Lending::new();
    plugin.connect_handler(context, request, response)
    }
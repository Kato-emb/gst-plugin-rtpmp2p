use gst::glib;

mod rtpmp2pdepay;

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    rtpmp2pdepay::register(plugin)?;

    Ok(())
}

gst::plugin_define!(
    rtpmp2p,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    concat!(env!("CARGO_PKG_VERSION"), "-", env!("COMMIT_ID")),
    "MPL-2.0",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    env!("BUILD_REL_DATE")
);

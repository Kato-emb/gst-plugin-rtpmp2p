use gst::glib;
use gst::prelude::*;

mod imp;

#[cfg(test)]
mod tests;

glib::wrapper! {
    pub struct RtpMP2PDepay(ObjectSubclass<imp::RtpMP2PDepay>) @extends gst_rtp::RTPBaseDepayload, gst::Element, gst::Object;
}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "rtpmp2pdepay",
        gst::Rank::MARGINAL,
        RtpMP2PDepay::static_type(),
    )
}

use std::sync::LazyLock;

use gst::{glib, prelude::PadExtManual, subclass::prelude::*};
use gst_rtp::{prelude::RTPBaseDepayloadExtManual, subclass::prelude::*};

#[derive(Default)]
pub struct RtpMP2PDepay;

static CAT: LazyLock<gst::DebugCategory> = LazyLock::new(|| {
    gst::DebugCategory::new(
        "rtpmp2pdepay",
        gst::DebugColorFlags::empty(),
        Some("RTP MPEG-PS Depayloader"),
    )
});

#[glib::object_subclass]
impl ObjectSubclass for RtpMP2PDepay {
    const NAME: &'static str = "RtpMP2PDepay";
    type Type = super::RtpMP2PDepay;
    type ParentType = gst_rtp::RTPBaseDepayload;
}

impl ObjectImpl for RtpMP2PDepay {}
impl GstObjectImpl for RtpMP2PDepay {}

impl ElementImpl for RtpMP2PDepay {
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: LazyLock<gst::subclass::ElementMetadata> = LazyLock::new(|| {
            gst::subclass::ElementMetadata::new(
                "RTP MPEG-PS Depayloader",
                "Codec/Depayloader/Network/RTP",
                "Depayload an MPEG Program Stream from RTP packets (RFC 2250)",
                "Ryouhei Kato <r-kato@musen.co.jp>",
            )
        });

        Some(&*ELEMENT_METADATA)
    }

    fn pad_templates() -> &'static [gst::PadTemplate] {
        static PAD_TEMPLATES: LazyLock<Vec<gst::PadTemplate>> = LazyLock::new(|| {
            let sink_pad_template = gst::PadTemplate::new(
                "sink",
                gst::PadDirection::Sink,
                gst::PadPresence::Always,
                &gst::Caps::builder("application/x-rtp")
                    .field("media", "video")
                    .field("clock-rate", 90000i32)
                    .field("encoding-name", "MP2P")
                    .build(),
            )
            .unwrap();

            let src_pad_template = gst::PadTemplate::new(
                "src",
                gst::PadDirection::Src,
                gst::PadPresence::Always,
                &gst::Caps::builder("video/mpeg")
                    .field("mpegversion", 2i32)
                    .field("systemstream", true)
                    .build(),
            )
            .unwrap();

            vec![src_pad_template, sink_pad_template]
        });

        PAD_TEMPLATES.as_ref()
    }
}

impl RTPBaseDepayloadImpl for RtpMP2PDepay {
    fn set_caps(&self, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        gst_rtp_mp2p_depay_setcaps(self, caps)
    }

    fn process_rtp_packet(
        &self,
        rtp_buffer: &gst_rtp::RTPBuffer<gst_rtp::rtp_buffer::Readable>,
    ) -> Option<gst::Buffer> {
        gst_rtp_mp2p_depay_process(self, rtp_buffer)
    }
}

fn gst_rtp_mp2p_depay_setcaps(
    depayload: &RtpMP2PDepay,
    caps: &gst::Caps,
) -> Result<(), gst::LoggableError> {
    let src_caps = gst::Caps::builder("video/mpeg")
        .field("mpegversion", 2i32)
        .field("systemstream", true)
        .build();

    if !depayload
        .obj()
        .src_pad()
        .push_event(gst::event::Caps::new(&src_caps))
    {
        return Err(gst::loggable_error!(CAT, "Failed to push caps event"));
    }

    depayload.parent_set_caps(caps)
}

fn gst_rtp_mp2p_depay_process(
    depayload: &RtpMP2PDepay,
    rtp_buffer: &gst_rtp::RTPBuffer<gst_rtp::rtp_buffer::Readable>,
) -> Option<gst::Buffer> {
    let Ok(outbuf) = rtp_buffer.payload_buffer() else {
        gst::warning!(
            CAT,
            imp = depayload,
            "Failed to get payload buffer from RTP packet"
        );
        return None;
    };

    let size = outbuf.size();
    // Note: RFC 2250 allows empty payloads, but they don't carry useful information, so we can skip them.
    if size == 0 {
        gst::warning!(CAT, imp = depayload, "Received empty payload, skipping");
        return None;
    }

    gst::trace!(CAT, imp = depayload, "pushing buffer of size {size}");

    Some(outbuf)
}

use gst_check::Harness;

fn init() {
    use std::sync::Once;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        gst::init().unwrap();
        crate::plugin_register_static().expect("rtpmp2p test");
    });
}

fn make_mp2p_caps() -> gst::Caps {
    gst::Caps::builder("application/x-rtp")
        .field("media", "video")
        .field("payload", 96i32)
        .field("clock-rate", 90000i32)
        .field("encoding-name", "MP2P")
        .build()
}

struct TestPacket {
    payload: Vec<u8>,
    pts: gst::ClockTime,
    marker: bool,
    rtp_timestamp: u32,
}

fn push_test_packets(h: &mut Harness, packets: &[TestPacket]) {
    for (idx, pkt) in packets.iter().enumerate() {
        let rtp_buf = rtp_types::RtpPacketBuilder::new()
            .payload_type(96)
            .sequence_number(idx as u16)
            .timestamp(pkt.rtp_timestamp)
            .marker_bit(pkt.marker)
            .payload(pkt.payload.as_slice())
            .write_vec()
            .unwrap();

        let mut buf = gst::Buffer::from_mut_slice(rtp_buf);
        buf.get_mut().unwrap().set_pts(pkt.pts);

        h.push(buf).unwrap();
    }

    h.push_event(gst::event::Eos::new());
}

#[test]
fn test_element_create() {
    init();

    gst::ElementFactory::make("rtpmp2pdepay")
        .build()
        .expect("Failed to create rtpmp2pdepay element");
}

#[test]
fn test_depay_single_packet() {
    init();

    let mut h = Harness::new("rtpmp2pdepay");
    h.play();
    h.set_src_caps(make_mp2p_caps());

    let payload = vec![0x00, 0x00, 0x01, 0xBA, 0x44, 0x00, 0x04, 0x00, 0x04, 0x01];

    push_test_packets(
        &mut h,
        &[TestPacket {
            payload: payload.clone(),
            pts: gst::ClockTime::from_seconds(0),
            marker: true,
            rtp_timestamp: 0,
        }],
    );

    let buffer = h.pull().unwrap();
    assert_eq!(buffer.pts(), Some(gst::ClockTime::from_seconds(0)));

    let map = buffer.into_mapped_buffer_readable().unwrap();
    assert_eq!(map.as_slice(), payload.as_slice());
}

#[test]
fn test_depay_multiple_packets() {
    init();

    let mut h = Harness::new("rtpmp2pdepay");
    h.play();
    h.set_src_caps(make_mp2p_caps());

    let payloads: Vec<Vec<u8>> = vec![
        vec![0x00, 0x00, 0x01, 0xBA, 0x44, 0x00, 0x04, 0x00, 0x04, 0x01],
        vec![0x00, 0x00, 0x01, 0xE0, 0x00, 0x10, 0x80, 0x80],
        vec![0x00, 0x00, 0x01, 0xBB, 0x00, 0x06, 0x80, 0x04, 0x01],
    ];

    let packets: Vec<TestPacket> = payloads
        .iter()
        .enumerate()
        .map(|(i, p)| TestPacket {
            payload: p.clone(),
            pts: gst::ClockTime::from_mseconds(i as u64 * 40),
            marker: true,
            rtp_timestamp: i as u32 * 3600,
        })
        .collect();

    push_test_packets(&mut h, &packets);

    for (i, expected_payload) in payloads.iter().enumerate() {
        let buffer = h.pull().unwrap();
        assert_eq!(
            buffer.pts(),
            Some(gst::ClockTime::from_mseconds(i as u64 * 40))
        );

        let map = buffer.into_mapped_buffer_readable().unwrap();
        assert_eq!(map.as_slice(), expected_payload.as_slice());
    }
}

#[test]
fn test_depay_empty_payload() {
    init();

    let mut h = Harness::new("rtpmp2pdepay");
    h.play();
    h.set_src_caps(make_mp2p_caps());

    push_test_packets(
        &mut h,
        &[TestPacket {
            payload: vec![],
            pts: gst::ClockTime::from_seconds(0),
            marker: true,
            rtp_timestamp: 0,
        }],
    );

    // Empty payload should not produce an output buffer
    let result = h.try_pull();
    assert!(result.is_none());
}

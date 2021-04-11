use bytes::{Bytes, BytesMut};
use criterion::{criterion_group, criterion_main, Criterion};
use webrtc_rtp::{header::*, packet::*, packetizer::*};

fn benchmark_packet(c: &mut Criterion) {
    let pkt = Packet {
        header: Header {
            extension: true,
            csrc: vec![1, 2],
            extension_profile: EXTENSION_PROFILE_TWO_BYTE,
            extensions: vec![
                Extension {
                    id: 1,
                    payload: Bytes::from_static(&[3, 4]),
                },
                Extension {
                    id: 2,
                    payload: Bytes::from_static(&[5, 6]),
                },
            ],
            ..Default::default()
        },
        payload: Bytes::from_static(&[0xFFu8; 15]), //vec![0x07, 0x08, 0x09, 0x0a], //MTU=1500
        ..Default::default()
    };
    let mut raw = BytesMut::new();
    let _ = pkt.marshal_to(&mut raw).unwrap();
    let raw = raw.freeze();
    let p = Packet::unmarshal(&raw).unwrap();
    if pkt != p {
        panic!(
            "marshal or unmarshal not correct: \npkt: {:?} \nvs \np: {:?}",
            pkt, p
        );
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    let mut buf = BytesMut::with_capacity(pkt.marshal_size());
    c.bench_function("Benchmark MarshalTo", |b| {
        b.iter(|| {
            buf.clear();
            let _ = pkt.marshal_to(&mut buf).unwrap();
        })
    });

    c.bench_function("Benchmark Marshal", |b| {
        b.iter(|| {
            let _ = pkt.marshal().unwrap();
        })
    });

    c.bench_function("Benchmark Unmarshal ", |b| {
        b.iter(|| {
            let _ = Packet::unmarshal(&raw).unwrap();
        })
    });
}

criterion_group!(benches, benchmark_packet);
criterion_main!(benches);

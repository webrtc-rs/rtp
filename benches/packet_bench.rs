use criterion::{criterion_group, criterion_main, Criterion};
use std::io::{BufReader, BufWriter};
use webrtc_rtp::{header::*, packet::*};

fn benchmark_packet(c: &mut Criterion) {
    let pkt = Packet {
        header: Header {
            extension: true,
            csrc: vec![1, 2],
            extension_profile: EXTENSION_PROFILE_TWO_BYTE,
            extensions: vec![
                Extension {
                    id: 1,
                    payload: vec![3, 4],
                },
                Extension {
                    id: 2,
                    payload: vec![5, 6],
                },
            ],
            ..Default::default()
        },
        payload: vec![0xFFu8; 1500], //vec![0x07, 0x08, 0x09, 0x0a], //MTU=1500
        ..Default::default()
    };
    let mut raw: Vec<u8> = vec![];
    {
        let mut writer = BufWriter::<&mut Vec<u8>>::new(raw.as_mut());
        pkt.marshal(&mut writer).unwrap();
    }
    let mut reader = BufReader::new(raw.as_slice());
    let p = Packet::unmarshal(&mut reader).unwrap();
    if pkt != p {
        panic!(
            "marshal or unmarshal not correct: \npkt: {:?} \nvs \np: {:?}",
            pkt, p
        );
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////

    c.bench_function("Benchmark Marshal", |b| {
        b.iter(|| {
            let mut buf: Vec<u8> = vec![];
            {
                let mut writer = BufWriter::<&mut Vec<u8>>::new(buf.as_mut());
                pkt.marshal(&mut writer).unwrap();
            }
        })
    });

    c.bench_function("Benchmark Unmarshal ", |b| {
        b.iter(|| {
            let mut reader = BufReader::new(raw.as_slice());
            let _ = Packet::unmarshal(&mut reader).unwrap();
        })
    });
}

criterion_group!(benches, benchmark_packet);
criterion_main!(benches);

use super::*;

#[test]
fn test_opus_unmarshal() -> Result<(), Error> {
    let mut pck = OpusPacket::default();

    // Empty packet
    let empty_bytes = Bytes::from_static(&[]);
    let result = pck.depacketize(&empty_bytes);
    assert!(result.is_err(), "Result should be err in case of error");

    // Normal packet
    let raw_bytes = Bytes::from_static(&[0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x90]);
    pck.depacketize(&raw_bytes)?;
    assert_eq!(&raw_bytes, &pck.payload, "Payload must be same");

    Ok(())
}

#[test]
fn test_opus_payload() -> Result<(), Error> {
    let pck = OpusPayloader;
    let empty = Bytes::from_static(&[]);
    let payload = Bytes::from_static(&[0x90, 0x90, 0x90]);

    // Positive MTU, empty payload
    let result = pck.payload(1, &empty)?;
    assert!(result.is_empty(), "Generated payload should be empty");

    // Positive MTU, small payload
    let result = pck.payload(1, &payload)?;
    assert_eq!(result.len(), 1, "Generated payload should be the 1");

    // Positive MTU, small payload
    let result = pck.payload(2, &payload)?;
    assert_eq!(result.len(), 1, "Generated payload should be the 1");

    Ok(())
}

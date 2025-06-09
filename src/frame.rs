use bitflags::bitflags;
use hpack::Encoder;

pub enum FrameType {
    DATA = 0x00,
    HEADERS = 0x01,
    PRIORITY = 0x02,
    RSTSTREAM = 0x03,
    SETTINGS = 0x04,
    PUSHPROMISE = 0x05,
    PING = 0x06,
    GOAWAY = 0x07,
    WINDOWUPDATE = 0x08,
    CONTINUATION = 0x09,
}

impl TryFrom<u8> for FrameType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(FrameType::DATA),
            0x01 => Ok(FrameType::HEADERS),
            0x02 => Ok(FrameType::PRIORITY),
            0x03 => Ok(FrameType::RSTSTREAM),
            0x04 => Ok(FrameType::SETTINGS),
            0x05 => Ok(FrameType::PUSHPROMISE),
            0x06 => Ok(FrameType::PING),
            0x07 => Ok(FrameType::GOAWAY),
            0x08 => Ok(FrameType::WINDOWUPDATE),
            0x09 => Ok(FrameType::CONTINUATION),
            _ => Err("Invalid frame type"),
        }
    }
}

impl TryInto<u8> for FrameType {
    type Error = &'static str;

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            FrameType::DATA => Ok(0x00),
            FrameType::HEADERS => Ok(0x01),
            FrameType::PRIORITY => Ok(0x02),
            FrameType::RSTSTREAM => Ok(0x03),
            FrameType::SETTINGS => Ok(0x04),
            FrameType::PUSHPROMISE => Ok(0x05),
            FrameType::PING => Ok(0x06),
            FrameType::GOAWAY => Ok(0x07),
            FrameType::WINDOWUPDATE => Ok(0x08),
            FrameType::CONTINUATION => Ok(0x09),
        }
    }
}

bitflags! {
    #[derive(PartialEq)]
    pub struct FrameFlags: u8 {
        const ENDSTREAM = 0x01;
        const ENDHEADERS = 0x04;
        const PADDED = 0x08;
        const PRIORITY = 0x20;
        const NONE = 0x00;
    }
}

impl std::fmt::Display for FrameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameType::DATA => write!(f, "DATA"),
            FrameType::HEADERS => write!(f, "HEADERS"),
            FrameType::PRIORITY => write!(f, "PRIORITY"),
            FrameType::RSTSTREAM => write!(f, "RSTSTREAM"),
            FrameType::SETTINGS => write!(f, "SETTINGS"),
            FrameType::PUSHPROMISE => write!(f, "PUSHPROMISE"),
            FrameType::PING => write!(f, "PING"),
            FrameType::GOAWAY => write!(f, "GOAWAY"),
            FrameType::WINDOWUPDATE => write!(f, "WINDOWUPDATE"),
            FrameType::CONTINUATION => write!(f, "CONTINUATION"),
        }
    }
}

// impl TryFrom<u8> for FrameFlags {
//     type Error = &'static str;

//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         match value {
//             0x01 => Ok(FrameFlags::ENDSTREAM),
//             0x04 => Ok(FrameFlags::ENDHEADERS),
//             0x08 => Ok(FrameFlags::PADDED),
//             0x20 => Ok(FrameFlags::PRIORITY),
//             _ => Ok(FrameFlags::NONE),
//         }
//     }
// }

// impl TryInto<u8> for FrameFlags {
//     type Error = &'static str;

//     fn try_into(self) -> Result<u8, Self::Error> {
//         match self {
//             FrameFlags::ENDSTREAM => Ok(0x01),
//             FrameFlags::ENDHEADERS => Ok(0x04),
//             FrameFlags::PADDED => Ok(0x08),
//             FrameFlags::PRIORITY => Ok(0x20),
//             FrameFlags::NONE => Ok(0x00),
//         }
//     }
// }

pub struct FrameWriter {
    pub frame_type: FrameType,
    pub flags: FrameFlags,
    pub stream_id: u32,
    pub payload_len: u32,
    pub payload: Vec<u8>,
}

impl FrameWriter {
    pub fn new(frame_type: FrameType, flags: FrameFlags, stream_id: u32, payload: Vec<u8>) -> Self {
        FrameWriter {
            frame_type,
            flags,
            stream_id,
            payload_len: payload.len() as u32,
            payload,
        }
    }

    pub fn new_frame_writer(
        frame_type: FrameType,
        flags: FrameFlags,
        stream_id: u32
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut headers = FrameHeaders::new();
        headers.add_header(":method".to_string(), "GET".to_string());
        headers.add_header(":path".to_string(), "/".to_string());
        headers.add_header(":scheme".to_string(), "https".to_string());
        headers.add_header(":authority".to_string(), "example.com".to_string());

        let payload = headers.serialize();

        // println!("[*] Serialized headers: {:?}", payload);

        Ok(FrameWriter {
            frame_type,
            flags,
            stream_id,
            payload_len: payload.len() as u32,
            payload: payload,
        })
    }

    // pub fn payload_from_frame_type(
    //     frame_type: FrameType
    // ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    //     let payload = match frame_type {
    //         FrameType::HEADERS => {
    //             let mut headers = FrameHeaders::new();
    //             headers.add_header(":method".to_string(), "GET".to_string());
    //             headers.add_header(":path".to_string(), "/".to_string());
    //             headers.add_header(":scheme".to_string(), "https".to_string());
    //             headers.add_header(":authority".to_string(), "example.com".to_string());

    //             headers.serialize()
    //         }

    //         _ => Vec::new(),
    //     };

    //     Ok(payload)
    // }

    pub fn serialize(self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut serialized = Vec::with_capacity(9 + (self.payload_len as usize));

        serialized.extend_from_slice(&self.payload_len.to_be_bytes()[1..]);
        serialized.push(FrameType::try_into(self.frame_type)?);
        serialized.push(self.flags.bits());
        serialized.extend_from_slice(
            &[
                (self.stream_id >> 24) as u8,
                (self.stream_id >> 16) as u8,
                (self.stream_id >> 8) as u8,
                self.stream_id as u8,
            ]
        );
        // serialized.extend_from_slice(&self.stream_id.to_be_bytes()[1..]);
        serialized.extend_from_slice(&self.payload);

        Ok(serialized)
    }
}

pub struct FrameReader {
    pub frame_type: FrameType,
    pub flags: FrameFlags,
    pub stream_id: u32,
    pub payload_len: u32,
    pub payload: Vec<u8>,
}

impl FrameReader {
    // pub async fn parse_frame(buf: &[u8]) -> Result<FrameReader, Box<dyn std::error::Error>> {
    //     FrameReader::deserialize(buf).await
    // }

    pub async fn read_frame(buf: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        if buf.len() < 9 {
            return Err("Buffer too small".into());
        }

        let payload_len = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);

        let frame_type = FrameType::try_from(buf[3])?;
        let flags = FrameFlags::from_bits_truncate(buf[4]);
        let stream_id = u32::from_be_bytes([buf[5], buf[6], buf[7], buf[8]]) & 0x7fff_ffff;
        let payload = buf[9..9 + (payload_len as usize)].to_vec();

        Ok(FrameReader {
            frame_type,
            flags,
            stream_id,
            payload_len,
            payload,
        })
    }

    pub async fn parse_settings_payload(&self) -> Vec<Setting> {
        let payload = &self.payload;

        if self.payload_len == 0 {
            return Vec::new();
        }

        let mut i = 0;
        let mut settings = Vec::new();
        while i < payload.len() {
            if i + 6 > payload.len() {
                break;
            }

            let id = u16::from_be_bytes([payload[i], payload[i + 1]]);
            let value = u32::from_be_bytes([
                payload[i + 2],
                payload[i + 3],
                payload[i + 4],
                payload[i + 5],
            ]);

            settings.push(Setting::new(id, value));

            i += 6;
        }

        settings
    }
}

pub struct HeadersBuilder {
    headers: Vec<(String, String)>,
}

impl HeadersBuilder {
    pub fn new() -> Self {
        HeadersBuilder { headers: Vec::new() }
    }

    pub fn add_header(mut self, name: String, value: String) -> Self {
        self.headers.push((name, value));
        self
    }

    pub fn build(
        self,
        stream_id: u32
        // flags: FrameFlags
    ) -> Result<FrameWriter, Box<dyn std::error::Error>> {
        let headers = FrameHeaders::from_pairs(self.headers);
        let payload = headers.serialize();

        Ok(
            FrameWriter::new(
                FrameType::HEADERS,
                FrameFlags::ENDHEADERS | FrameFlags::ENDSTREAM,
                stream_id,
                payload
            )
        )
    }
}

pub struct FrameHeaders {
    headers: Vec<(String, String)>,
}

impl FrameHeaders {
    pub fn new() -> Self {
        FrameHeaders { headers: Vec::new() }
    }

    pub fn from_pairs(headers: Vec<(String, String)>) -> Self {
        FrameHeaders { headers }
    }

    pub fn add_header(&mut self, name: String, value: String) {
        self.headers.push((name, value));
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut encoder = Encoder::new();
        let headers: Vec<(&[u8], &[u8])> = self.headers
            .iter()
            .map(|(name, value)| (name.as_bytes(), value.as_bytes()))
            .collect();
        encoder.encode(headers)
    }
}

pub struct Setting {
    pub id: u16,
    pub value: u32,
}

impl Setting {
    pub fn new(id: u16, value: u32) -> Self {
        Setting { id, value }
    }
}

pub struct Settings {
    pub settings: Vec<Setting>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            settings: vec![
                Setting::new(0x03, 100), // Max Concurrent Streams
                Setting::new(0x04, 65535), // Initial Window Size
                Setting::new(0x05, 16384), // Max Frame Size
                Setting::new(0x06, 0) // Max Header List Size
            ],
        }
    }
}

impl Settings {
    pub async fn new() -> Self {
        Settings {
            settings: Vec::new(),
        }
    }

    pub async fn write_settings_ack() -> Vec<u8> {
        vec![0, 0, 0, 4, 1, 0, 0, 0, 0]
    }

    pub async fn from_pairs(pairs: Vec<(u16, u32)>) -> Self {
        let mut settings = Settings::new().await;

        for (id, value) in pairs {
            settings.settings.push(Setting::new(id, value));
        }

        settings
    }

    pub async fn serialize(self) -> Vec<u8> {
        let mut payload = Vec::new();

        for setting in self.settings {
            payload.extend_from_slice(&setting.id.to_be_bytes());
            payload.extend_from_slice(&setting.value.to_be_bytes());
        }

        payload
    }
}

pub struct SettingsBuilder {
    settings: Vec<(u16, u32)>,
}
impl SettingsBuilder {
    pub fn new() -> Self {
        SettingsBuilder { settings: Vec::new() }
    }

    pub fn add_setting(mut self, id: u16, value: u32) -> Self {
        self.settings.push((id, value));
        self
    }

    pub async fn build(self) -> FrameWriter {
        let settings = Settings::from_pairs(self.settings).await;
        let payload = settings.serialize().await;

        FrameWriter::new(FrameType::SETTINGS, FrameFlags::NONE, 0, payload)
    }
}

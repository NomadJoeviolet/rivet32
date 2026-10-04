//! Referee protocol profile. Lengths follow the supported wire structures,
//! including 0303=12, 0305=48 and 0307=105 (their source comments are stale).
//! No struct casts or native bitfield layout is used on the wire.
use crate::{Error, exact, le16, le32};
use embodied_algorithms::crc::{crc8_dji, crc16_dji};
pub const MAX_PAYLOAD: usize = 300;
pub const MAX_FRAME: usize = MAX_PAYLOAD + 9;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Frame<'a> {
    pub sequence: u8,
    pub command: u16,
    pub payload: &'a [u8],
}
impl<'a> Frame<'a> {
    pub fn decode(d: &'a [u8]) -> Result<Self, Error> {
        if d.len() < 9 {
            return Err(Error::Length);
        }
        if d[0] != 0xa5 {
            return Err(Error::Header);
        }
        if crc8_dji(&d[..4]) != d[4] {
            return Err(Error::Checksum);
        }
        let size = le16(d, 1) as usize;
        exact(d, size + 9)?;
        if crc16_dji(&d[..d.len() - 2]) != le16(d, d.len() - 2) {
            return Err(Error::Checksum);
        }
        Ok(Self {
            sequence: d[3],
            command: le16(d, 5),
            payload: &d[7..d.len() - 2],
        })
    }
}
pub fn encode_frame(
    sequence: u8,
    command: u16,
    payload: &[u8],
    out: &mut [u8],
) -> Result<usize, Error> {
    if payload.len() > u16::MAX as usize {
        return Err(Error::Length);
    }
    let n = payload.len() + 9;
    if out.len() < n {
        return Err(Error::Capacity);
    }
    out[0] = 0xa5;
    out[1..3].copy_from_slice(&(payload.len() as u16).to_le_bytes());
    out[3] = sequence;
    out[4] = crc8_dji(&out[..4]);
    out[5..7].copy_from_slice(&command.to_le_bytes());
    out[7..n - 2].copy_from_slice(payload);
    let crc = crc16_dji(&out[..n - 2]);
    out[n - 2..n].copy_from_slice(&crc.to_le_bytes());
    Ok(n)
}

/// Byte-oriented framing with bounded storage. Fragmentation/coalescing, noise,
/// bad header lengths, bad CRCs, and overlapping SOF candidates recover in-stream.
pub struct Stream {
    buffer: [u8; MAX_FRAME],
    len: usize,
    pub rejected_frames: u64,
}
impl Default for Stream {
    fn default() -> Self {
        Self::new()
    }
}
impl Stream {
    pub const fn new() -> Self {
        Self {
            buffer: [0; MAX_FRAME],
            len: 0,
            rejected_frames: 0,
        }
    }
    pub fn reset(&mut self) {
        self.len = 0;
    }
    fn discard(&mut self, n: usize) {
        self.buffer.copy_within(n..self.len, 0);
        self.len -= n;
    }
    pub fn feed(&mut self, input: &[u8], mut emit: impl FnMut(Frame<'_>)) {
        for &b in input {
            if self.len == MAX_FRAME {
                self.discard(1);
                self.rejected_frames = self.rejected_frames.saturating_add(1);
            }
            self.buffer[self.len] = b;
            self.len += 1;
            loop {
                if self.len == 0 {
                    break;
                }
                if self.buffer[0] != 0xa5 {
                    self.discard(1);
                    continue;
                }
                if self.len < 5 {
                    break;
                }
                let payload = le16(&self.buffer, 1) as usize;
                if crc8_dji(&self.buffer[..4]) != self.buffer[4] || payload > MAX_PAYLOAD {
                    self.discard(1);
                    self.rejected_frames = self.rejected_frames.saturating_add(1);
                    continue;
                }
                let total = payload + 9;
                if self.len < total {
                    break;
                }
                match Frame::decode(&self.buffer[..total]) {
                    Ok(frame) => {
                        emit(frame);
                        self.discard(total)
                    }
                    Err(_) => {
                        self.discard(1);
                        self.rejected_frames = self.rejected_frames.saturating_add(1);
                    }
                }
            }
        }
    }
}

trait Wire: Copy + Default {
    fn read(d: &[u8], at: &mut usize) -> Result<Self, Error>;
    fn write(self, d: &mut [u8], at: &mut usize);
}
macro_rules! scalar {
    ($t:ty,$n:expr) => {
        impl Wire for $t {
            fn read(d: &[u8], at: &mut usize) -> Result<Self, Error> {
                let mut b = [0; $n];
                b.copy_from_slice(&d[*at..*at + $n]);
                *at += $n;
                Ok(<$t>::from_le_bytes(b))
            }
            fn write(self, d: &mut [u8], at: &mut usize) {
                d[*at..*at + $n].copy_from_slice(&self.to_le_bytes());
                *at += $n;
            }
        }
    };
}
scalar!(u8, 1);
scalar!(i8, 1);
scalar!(u16, 2);
scalar!(i16, 2);
scalar!(u32, 4);
scalar!(u64, 8);
impl Wire for f32 {
    fn read(d: &[u8], at: &mut usize) -> Result<Self, Error> {
        let v = f32::from_bits(u32::read(d, at)?);
        if v.is_finite() {
            Ok(v)
        } else {
            Err(Error::NonFinite)
        }
    }
    fn write(self, d: &mut [u8], at: &mut usize) {
        self.to_bits().write(d, at)
    }
}
impl<T: Wire, const N: usize> Wire for [T; N]
where
    [T; N]: Default,
{
    fn read(d: &[u8], at: &mut usize) -> Result<Self, Error> {
        let mut v = [T::default(); N];
        for x in &mut v {
            *x = T::read(d, at)?;
        }
        Ok(v)
    }
    fn write(self, d: &mut [u8], at: &mut usize) {
        for x in self {
            x.write(d, at);
        }
    }
}

macro_rules! payload{($name:ident,$len:expr,{$($field:ident:$ty:ty),*$(,)?})=>{
 #[derive(Clone,Copy,Debug,PartialEq)]pub struct $name{$(pub $field:$ty),*}
 impl $name{pub const LEN:usize=$len;pub fn decode(data:&[u8])->Result<Self,Error>{exact(data,$len)?;let mut at=0;let value=Self{$($field:<$ty as Wire>::read(data,&mut at)?),*};debug_assert_eq!(at,$len);Ok(value)}pub fn encode(&self)->[u8;$len]{let mut data=[0;$len];let mut at=0;$(self.$field.write(&mut data,&mut at);)*debug_assert_eq!(at,$len);data}}
};}
payload!(GameStatus,11,{game_type_and_process:u8,stage_remaining_seconds:u16,sync_timestamp:u64});
impl GameStatus {
    pub const fn game_type(self) -> u8 {
        self.game_type_and_process & 15
    }
    pub const fn process(self) -> u8 {
        self.game_type_and_process >> 4
    }
}
payload!(GameResult,1,{winner:u8});
payload!(RobotHp,20,{ally_1:u16,ally_2:u16,ally_3:u16,ally_4:u16,damage_difference:i16,ally_7:u16,ally_outpost:u16,ally_base:u16,enemy_outpost:u16,enemy_base:u16});
payload!(FieldEvent,4,{events:u32});
payload!(Warning,3,{level:u8,offending_robot:u8,count:u8});
payload!(Dart,3,{remaining_seconds:u8,target_bits:u16});
impl Dart {
    pub const fn last_target(self) -> u8 {
        (self.target_bits & 7) as u8
    }
    pub const fn hit_count(self) -> u8 {
        ((self.target_bits >> 3) & 7) as u8
    }
    pub const fn selected_target(self) -> u8 {
        ((self.target_bits >> 6) & 7) as u8
    }
}
payload!(RobotPerformance,17,{id:u8,level:u8,current_hp:u16,max_hp:u16,cooling_rate:u16,heat_limit:u16,power_limit:u16,bullet_speed_limit:f32,power_outputs:u8});
payload!(PowerAndHeat,14,{reserved_1:u16,reserved_2:u16,reserved_3:f32,buffer_energy:u16,heat_17mm:u16,heat_42mm:u16});
payload!(RobotPosition,12,{x:f32,y:f32,angle:f32});
payload!(RobotGain,8,{recovery:u8,cooling:u16,defence:u8,vulnerability:u8,attack:u16,remaining_energy:u8});
payload!(Damage,1,{armor_and_reason:u8});
impl Damage {
    pub const fn armor(self) -> u8 {
        self.armor_and_reason & 15
    }
    pub const fn reason(self) -> u8 {
        self.armor_and_reason >> 4
    }
}
payload!(ShootStatus,7,{projectile_type:u8,shooter:u8,frequency:u8,initial_speed:f32});
payload!(ProjectileAllowance,8,{projectiles_17mm:u16,projectiles_42mm:u16,gold:u16,fortress_projectiles:u16});
payload!(Rfid,5,{status:u32,status_2:u8});
payload!(DartCommand,6,{status:u8,reserved:u8,target_change_seconds:u16,last_launch_seconds:u16});
payload!(GroundPositions,40,{positions:[[f32;2];5]});
payload!(RadarMark,2,{progress:u16});
payload!(SentryDecision,14,{info:u32,info_2:u16,info_3:u64});
payload!(RadarDecision,1,{bits:u8});
impl RadarDecision {
    pub const fn double_chance(self) -> u8 {
        self.bits & 3
    }
    pub const fn double_active(self) -> bool {
        self.bits & 4 != 0
    }
    pub const fn encryption_level(self) -> u8 {
        (self.bits >> 3) & 3
    }
    pub const fn can_change_password(self) -> bool {
        self.bits & 32 != 0
    }
}
payload!(MapInteraction,12,{target_x:f32,target_y:f32,keyboard:u8,target_robot:u8,source:u16});
payload!(RadarPositions,48,{opponents:[[u16;2];6],allies:[[u16;2];6]});
payload!(CustomClient,8,{keys:u16,x_and_left:u16,y_and_right:u16,reserved:u16});
impl CustomClient {
    pub const fn x(self) -> u16 {
        self.x_and_left & 0xfff
    }
    pub const fn y(self) -> u16 {
        self.y_and_right & 0xfff
    }
    pub const fn left(self) -> u8 {
        (self.x_and_left >> 12) as u8
    }
    pub const fn right(self) -> u8 {
        (self.y_and_right >> 12) as u8
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SentryPath {
    pub intention: u8,
    pub start_x_dm: u16,
    pub start_y_dm: u16,
    pub delta_x: [i8; 49],
    pub delta_y: [i8; 49],
    pub sender: u16,
}
impl SentryPath {
    pub fn decode(d: &[u8]) -> Result<Self, Error> {
        exact(d, 105)?;
        let mut x = [0; 49];
        let mut y = [0; 49];
        for i in 0..49 {
            x[i] = d[5 + i] as i8;
            y[i] = d[54 + i] as i8;
        }
        Ok(Self {
            intention: d[0],
            start_x_dm: le16(d, 1),
            start_y_dm: le16(d, 3),
            delta_x: x,
            delta_y: y,
            sender: le16(d, 103),
        })
    }
    pub fn encode(&self) -> [u8; 105] {
        let mut d = [0; 105];
        d[0] = self.intention;
        d[1..3].copy_from_slice(&self.start_x_dm.to_le_bytes());
        d[3..5].copy_from_slice(&self.start_y_dm.to_le_bytes());
        for i in 0..49 {
            d[5 + i] = self.delta_x[i] as u8;
            d[54 + i] = self.delta_y[i] as u8;
        }
        d[103..].copy_from_slice(&self.sender.to_le_bytes());
        d
    }
}
payload!(MapData,34,{sender:u16,receiver:u16,data:[u8;30]});

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DoubleArm14 {
    pub joint_angles_milliradians: [[i16; 7]; 2],
    pub control_flags: u16,
}
impl DoubleArm14 {
    pub fn decode(d: &[u8]) -> Result<Self, Error> {
        exact(d, 30)?;
        let mut angles = [[0; 7]; 2];
        for (i, angle) in angles.iter_mut().flatten().enumerate() {
            *angle = le16(d, i * 2) as i16;
        }
        Ok(Self {
            joint_angles_milliradians: angles,
            control_flags: le16(d, 28),
        })
    }
    pub fn encode(self) -> [u8; 30] {
        let mut d = [0; 30];
        for (i, angle) in self.joint_angles_milliradians.iter().flatten().enumerate() {
            d[i * 2..i * 2 + 2].copy_from_slice(&angle.to_le_bytes());
        }
        d[28..].copy_from_slice(&self.control_flags.to_le_bytes());
        d
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CustomControllerProfile {
    DoubleArm14,
    Raw30,
}

/// Validated interaction envelope. Body retains custom robot-to-robot payloads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Interaction {
    pub subcommand: u16,
    pub sender: u16,
    pub receiver: u16,
    body: [u8; 112],
    len: u8,
}
impl Interaction {
    pub fn decode(d: &[u8]) -> Result<Self, Error> {
        if !(6..=118).contains(&d.len()) {
            return Err(Error::Length);
        }
        let subcommand = le16(d, 0);
        let body = &d[6..];
        let expected = match subcommand {
            0x100 => Some(2),
            0x101 => Some(15),
            0x102 => Some(30),
            0x103 => Some(75),
            0x104 => Some(105),
            0x110 => Some(45),
            0x120 => Some(4),
            0x121 => Some(8),
            _ => None,
        };
        if let Some(size) = expected {
            exact(body, size)?;
        }
        match subcommand {
            0x100 if body[0] > 2 || body[1] > 9 => return Err(Error::Range),
            0x101..=0x104 => {
                for graphic in body.as_chunks::<15>().0 {
                    Graphic::decode(graphic)?;
                }
            }
            0x110 => {
                let graphic = Graphic::decode(&body[..15])?;
                if graphic.kind != 7 || graphic.details_b > 30 {
                    return Err(Error::Range);
                }
            }
            _ => {}
        }
        let mut bytes = [0; 112];
        bytes[..body.len()].copy_from_slice(body);
        Ok(Self {
            subcommand,
            sender: le16(d, 2),
            receiver: le16(d, 4),
            body: bytes,
            len: body.len() as u8,
        })
    }
    pub fn body(&self) -> &[u8] {
        &self.body[..self.len as usize]
    }
    pub fn encode(&self, out: &mut [u8]) -> Result<usize, Error> {
        encode_interaction(
            self.subcommand,
            self.sender,
            self.receiver,
            self.body(),
            out,
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SentryCommand {
    pub confirmed_resurrection: bool,
    pub immediate_resurrection: bool,
    pub projectile_allowance: u16,
    pub projectile_conversion_count: u8,
    pub hp_conversion_count: u8,
    pub stance: u8,
    pub activate_buff: bool,
}
impl SentryCommand {
    pub fn encode(self) -> Result<[u8; 4], Error> {
        if self.projectile_allowance > 2047
            || self.projectile_conversion_count > 15
            || self.hp_conversion_count > 15
            || self.stance > 7
        {
            return Err(Error::Range);
        }
        let bits = u32::from(self.confirmed_resurrection)
            | u32::from(self.immediate_resurrection) << 1
            | (self.projectile_allowance as u32) << 2
            | (self.projectile_conversion_count as u32) << 13
            | (self.hp_conversion_count as u32) << 17
            | (self.stance as u32) << 21
            | u32::from(self.activate_buff) << 24;
        Ok(bits.to_le_bytes())
    }
    pub fn decode(d: &[u8]) -> Result<Self, Error> {
        exact(d, 4)?;
        let b = le32(d, 0);
        Ok(Self {
            confirmed_resurrection: b & 1 != 0,
            immediate_resurrection: b & 2 != 0,
            projectile_allowance: ((b >> 2) & 2047) as u16,
            projectile_conversion_count: ((b >> 13) & 15) as u8,
            hp_conversion_count: ((b >> 17) & 15) as u8,
            stance: ((b >> 21) & 7) as u8,
            activate_buff: b & (1 << 24) != 0,
        })
    }
}
payload!(RadarCommand,8,{confirm_double:u8,password_command:u8,password:[u8;6]});
pub fn encode_interaction(
    subcommand: u16,
    sender: u16,
    receiver: u16,
    body: &[u8],
    out: &mut [u8],
) -> Result<usize, Error> {
    if body.len() > 112 {
        return Err(Error::Length);
    }
    let n = body.len() + 6;
    if out.len() < n {
        return Err(Error::Capacity);
    }
    out[..2].copy_from_slice(&subcommand.to_le_bytes());
    out[2..4].copy_from_slice(&sender.to_le_bytes());
    out[4..6].copy_from_slice(&receiver.to_le_bytes());
    out[6..n].copy_from_slice(body);
    Interaction::decode(&out[..n])?;
    Ok(n)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Data {
    GameStatus(GameStatus),
    GameResult(GameResult),
    RobotHp(RobotHp),
    FieldEvent(FieldEvent),
    Warning(Warning),
    Dart(Dart),
    RobotPerformance(RobotPerformance),
    PowerAndHeat(PowerAndHeat),
    RobotPosition(RobotPosition),
    RobotGain(RobotGain),
    Damage(Damage),
    ShootStatus(ShootStatus),
    ProjectileAllowance(ProjectileAllowance),
    Rfid(Rfid),
    DartCommand(DartCommand),
    GroundPositions(GroundPositions),
    RadarMark(RadarMark),
    SentryDecision(SentryDecision),
    RadarDecision(RadarDecision),
    Interaction(Interaction),
    DoubleArm14(DoubleArm14),
    CustomRaw([u8; 30]),
    MapInteraction(MapInteraction),
    RadarPositions(RadarPositions),
    CustomClient(CustomClient),
    SentryPath(SentryPath),
    MapData(MapData),
    RobotCustom([u8; 30]),
    RobotCustom2([u8; 300]),
    RobotCustom3([u8; 30]),
}
fn array<const N: usize>(d: &[u8]) -> Result<[u8; N], Error> {
    exact(d, N)?;
    let mut result = [0; N];
    result.copy_from_slice(d);
    Ok(result)
}
impl Data {
    pub fn decode(command: u16, d: &[u8], profile: CustomControllerProfile) -> Result<Self, Error> {
        Ok(match command {
            0x0001 => Self::GameStatus(GameStatus::decode(d)?),
            0x0002 => Self::GameResult(GameResult::decode(d)?),
            0x0003 => Self::RobotHp(RobotHp::decode(d)?),
            0x0101 => Self::FieldEvent(FieldEvent::decode(d)?),
            0x0104 => Self::Warning(Warning::decode(d)?),
            0x0105 => Self::Dart(Dart::decode(d)?),
            0x0201 => Self::RobotPerformance(RobotPerformance::decode(d)?),
            0x0202 => Self::PowerAndHeat(PowerAndHeat::decode(d)?),
            0x0203 => Self::RobotPosition(RobotPosition::decode(d)?),
            0x0204 => Self::RobotGain(RobotGain::decode(d)?),
            0x0206 => Self::Damage(Damage::decode(d)?),
            0x0207 => Self::ShootStatus(ShootStatus::decode(d)?),
            0x0208 => Self::ProjectileAllowance(ProjectileAllowance::decode(d)?),
            0x0209 => Self::Rfid(Rfid::decode(d)?),
            0x020a => Self::DartCommand(DartCommand::decode(d)?),
            0x020b => Self::GroundPositions(GroundPositions::decode(d)?),
            0x020c => Self::RadarMark(RadarMark::decode(d)?),
            0x020d => Self::SentryDecision(SentryDecision::decode(d)?),
            0x020e => Self::RadarDecision(RadarDecision::decode(d)?),
            0x0301 => Self::Interaction(Interaction::decode(d)?),
            0x0302 => match profile {
                CustomControllerProfile::DoubleArm14 => Self::DoubleArm14(DoubleArm14::decode(d)?),
                CustomControllerProfile::Raw30 => Self::CustomRaw(array(d)?),
            },
            0x0303 => Self::MapInteraction(MapInteraction::decode(d)?),
            0x0305 => Self::RadarPositions(RadarPositions::decode(d)?),
            0x0306 => Self::CustomClient(CustomClient::decode(d)?),
            0x0307 => Self::SentryPath(SentryPath::decode(d)?),
            0x0308 => Self::MapData(MapData::decode(d)?),
            0x0309 => Self::RobotCustom(array(d)?),
            0x0310 => Self::RobotCustom2(array(d)?),
            0x0311 => Self::RobotCustom3(array(d)?),
            _ => return Err(Error::Unsupported),
        })
    }
    /// Encode the typed payload and return (command ID, payload length).
    pub fn encode(&self, out: &mut [u8]) -> Result<(u16, usize), Error> {
        fn copy(id: u16, data: &[u8], out: &mut [u8]) -> Result<(u16, usize), Error> {
            // Public payload fields may have been constructed manually. Reuse
            // the same wire validation before publishing an outbound payload.
            Data::decode(id, data, CustomControllerProfile::DoubleArm14)?;
            if out.len() < data.len() {
                return Err(Error::Capacity);
            }
            out[..data.len()].copy_from_slice(data);
            Ok((id, data.len()))
        }
        match self {
            Self::GameStatus(v) => copy(1, &v.encode(), out),
            Self::GameResult(v) => copy(2, &v.encode(), out),
            Self::RobotHp(v) => copy(3, &v.encode(), out),
            Self::FieldEvent(v) => copy(0x101, &v.encode(), out),
            Self::Warning(v) => copy(0x104, &v.encode(), out),
            Self::Dart(v) => copy(0x105, &v.encode(), out),
            Self::RobotPerformance(v) => copy(0x201, &v.encode(), out),
            Self::PowerAndHeat(v) => copy(0x202, &v.encode(), out),
            Self::RobotPosition(v) => copy(0x203, &v.encode(), out),
            Self::RobotGain(v) => copy(0x204, &v.encode(), out),
            Self::Damage(v) => copy(0x206, &v.encode(), out),
            Self::ShootStatus(v) => copy(0x207, &v.encode(), out),
            Self::ProjectileAllowance(v) => copy(0x208, &v.encode(), out),
            Self::Rfid(v) => copy(0x209, &v.encode(), out),
            Self::DartCommand(v) => copy(0x20a, &v.encode(), out),
            Self::GroundPositions(v) => copy(0x20b, &v.encode(), out),
            Self::RadarMark(v) => copy(0x20c, &v.encode(), out),
            Self::SentryDecision(v) => copy(0x20d, &v.encode(), out),
            Self::RadarDecision(v) => copy(0x20e, &v.encode(), out),
            Self::Interaction(v) => Ok((0x301, v.encode(out)?)),
            Self::DoubleArm14(v) => copy(0x302, &v.encode(), out),
            Self::CustomRaw(v) => copy(0x302, v, out),
            Self::MapInteraction(v) => copy(0x303, &v.encode(), out),
            Self::RadarPositions(v) => copy(0x305, &v.encode(), out),
            Self::CustomClient(v) => copy(0x306, &v.encode(), out),
            Self::SentryPath(v) => copy(0x307, &v.encode(), out),
            Self::MapData(v) => copy(0x308, &v.encode(), out),
            Self::RobotCustom(v) => copy(0x309, v, out),
            Self::RobotCustom2(v) => copy(0x310, v, out),
            Self::RobotCustom3(v) => copy(0x311, v, out),
        }
    }
}

/// Graphic's three little-endian bitfield words, as specified by the referee protocol.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Graphic {
    pub name: [u8; 3],
    pub operation: u8,
    pub kind: u8,
    pub layer: u8,
    pub color: u8,
    pub details_a: u16,
    pub details_b: u16,
    pub width: u16,
    pub start_x: u16,
    pub start_y: u16,
    pub details_c: u16,
    pub details_d: u16,
    pub details_e: u16,
}
impl Graphic {
    pub fn encode(self) -> Result<[u8; 15], Error> {
        if self.operation > 3
            || self.kind > 7
            || self.layer > 9
            || self.color > 8
            || self.details_a > 511
            || self.details_b > 511
            || self.width > 1023
            || self.start_x > 2047
            || self.start_y > 2047
            || self.details_c > 1023
            || self.details_d > 2047
            || self.details_e > 2047
        {
            return Err(Error::Range);
        }
        let a = self.operation as u32
            | (self.kind as u32) << 3
            | (self.layer as u32) << 6
            | (self.color as u32) << 10
            | (self.details_a as u32) << 14
            | (self.details_b as u32) << 23;
        let b = self.width as u32 | (self.start_x as u32) << 10 | (self.start_y as u32) << 21;
        let c =
            self.details_c as u32 | (self.details_d as u32) << 10 | (self.details_e as u32) << 21;
        let mut d = [0; 15];
        d[..3].copy_from_slice(&self.name);
        d[3..7].copy_from_slice(&a.to_le_bytes());
        d[7..11].copy_from_slice(&b.to_le_bytes());
        d[11..].copy_from_slice(&c.to_le_bytes());
        Ok(d)
    }
    pub fn decode(d: &[u8]) -> Result<Self, Error> {
        exact(d, 15)?;
        let a = le32(d, 3);
        let b = le32(d, 7);
        let c = le32(d, 11);
        let result = Self {
            name: [d[0], d[1], d[2]],
            operation: (a & 7) as u8,
            kind: ((a >> 3) & 7) as u8,
            layer: ((a >> 6) & 15) as u8,
            color: ((a >> 10) & 15) as u8,
            details_a: ((a >> 14) & 511) as u16,
            details_b: (a >> 23) as u16,
            width: (b & 1023) as u16,
            start_x: ((b >> 10) & 2047) as u16,
            start_y: (b >> 21) as u16,
            details_c: (c & 1023) as u16,
            details_d: ((c >> 10) & 2047) as u16,
            details_e: (c >> 21) as u16,
        };
        result.encode()?;
        Ok(result)
    }
    /// Number graphics carry the signed integer bits in the final word. Float
    /// figures use an integer scaled by 1000, as on the original wire protocol.
    pub fn set_number(&mut self, value: i32) {
        let v = value as u32;
        self.details_c = (v & 1023) as u16;
        self.details_d = ((v >> 10) & 2047) as u16;
        self.details_e = (v >> 21) as u16;
    }
    pub const fn number(self) -> i32 {
        (self.details_c as u32 | (self.details_d as u32) << 10 | (self.details_e as u32) << 21)
            as i32
    }
}
pub fn encode_graphics(
    sender: u16,
    receiver: u16,
    graphics: &[Graphic],
    out: &mut [u8],
) -> Result<usize, Error> {
    let command = match graphics.len() {
        1 => 0x101,
        2 => 0x102,
        5 => 0x103,
        7 => 0x104,
        _ => return Err(Error::Length),
    };
    let mut body = [0; 105];
    for (i, g) in graphics.iter().enumerate() {
        body[i * 15..i * 15 + 15].copy_from_slice(&g.encode()?);
    }
    encode_interaction(command, sender, receiver, &body[..graphics.len() * 15], out)
}
pub fn encode_string(
    sender: u16,
    receiver: u16,
    mut graphic: Graphic,
    text: &[u8],
    out: &mut [u8],
) -> Result<usize, Error> {
    if text.len() > 30 {
        return Err(Error::Length);
    }
    graphic.kind = 7;
    graphic.details_b = text.len() as u16;
    let mut body = [0; 45];
    body[..15].copy_from_slice(&graphic.encode()?);
    body[15..15 + text.len()].copy_from_slice(text);
    encode_interaction(0x110, sender, receiver, &body, out)
}
pub fn delete_layer(
    sender: u16,
    receiver: u16,
    layer: Option<u8>,
    out: &mut [u8],
) -> Result<usize, Error> {
    if layer.is_some_and(|l| l > 9) {
        return Err(Error::Range);
    }
    encode_interaction(
        0x100,
        sender,
        receiver,
        &[if layer.is_some() { 1 } else { 2 }, layer.unwrap_or(0)],
        out,
    )
}

/// Decoded-data receiver with explicit 0302 profile and timestamped last-good state.
pub struct Receiver {
    profile: CustomControllerProfile,
    state: crate::Receiver<Data>,
}
impl Receiver {
    pub const fn new(profile: CustomControllerProfile, timeout_us: u64) -> Self {
        Self {
            profile,
            state: crate::Receiver::new(timeout_us),
        }
    }
    pub fn receive(&mut self, frame: Frame<'_>, now_us: u64) -> Result<Data, Error> {
        let data = Data::decode(frame.command, frame.payload, self.profile)?;
        self.state.accept(data, now_us)?;
        Ok(data)
    }
    pub fn data(&self) -> Option<&Data> {
        self.state.current()
    }
    pub fn connection(&self, now_us: u64) -> crate::Connection {
        self.state.connection(now_us)
    }
}

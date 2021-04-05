use crate::{deserialize::*, serialize::*};

use time::{OffsetDateTime, PrimitiveDateTime, UtcOffset};

// The DateTime struct represents a DateTime as received in IRC
//
// DateTime is, like all other struct based types, serialized sequentially.
// #[derive(Clone, Debug, std::cmp::PartialEq)]
// pub struct DateTime {
//     /// Day in Julian calendar, unknown if signed or unsigned
//     julian_day: i32,
//     /// Milliseconds since start of day
//     millis_of_day: i32,
//     /// Timezone of DateTime, 0x00 is local, 0x01 is UTC
//     zone: u8,
// }

pub type DateTime = OffsetDateTime;
pub use time::{Date, Time};

/// TimeSpec specifies whether the time is a local time, daylightsaving local time or a form of UTC Offset
#[repr(i8)]
#[derive(Copy, Clone, Debug, std::cmp::PartialEq)]
pub enum TimeSpec {
    LocalUnknown = -0x01,
    LocalStandard = 0x00,
    LocalDST = 0x01,
    UTC = 0x02,
    OffsetFromUTC = 0x03,
}

impl From<i8> for TimeSpec {
    fn from(val: i8) -> Self {
        match val {
            -0x01 => TimeSpec::LocalUnknown,
            0x00 => TimeSpec::LocalStandard,
            0x01 => TimeSpec::LocalDST,
            0x02 => TimeSpec::UTC,
            0x03 => TimeSpec::OffsetFromUTC,
            _ => unimplemented!(),
        }
    }
}

impl Serialize for OffsetDateTime {
    fn serialize(&self) -> Result<Vec<u8>, failure::Error> {
        let mut values: Vec<u8> = Vec::new();

        values.extend(i32::serialize(&(self.date().julian_day() as i32))?);

        let time: i32 = {
            let hour: i32 = self.time().hour() as i32;
            let minute: i32 = self.time().minute() as i32;
            let second: i32 = self.time().second() as i32;
            let milli: i32 = self.time().millisecond() as i32;

            milli + (second * 1000) + (minute * 60000) + (hour * 60 * 60000)
        };

        values.extend(i32::serialize(&time)?);
        values.extend(u8::serialize(&(TimeSpec::OffsetFromUTC as u8))?);
        values.extend(i32::serialize(&self.offset().as_seconds())?);

        Ok(values)
    }
}

impl Deserialize for OffsetDateTime {
    fn parse(b: &[u8]) -> Result<(usize, Self), failure::Error> {
        let (_, julian_day) = i32::parse(&b[0..4])?;
        let (_, millis_of_day) = i32::parse(&b[4..8])?;
        let (_, zone) = u8::parse(&b[8..9])?;

        let mut pos = 9;

        let zone = TimeSpec::from(zone as i8);

        // Default to unix epoch when one of these is set to -1
        if julian_day == -1 || millis_of_day == -1 {
            return Ok((pos, OffsetDateTime::unix_epoch()));
        }

        let offset: UtcOffset;
        match zone {
            TimeSpec::LocalUnknown | TimeSpec::LocalStandard | TimeSpec::LocalDST => {
                offset = UtcOffset::try_current_local_offset().unwrap_or_else(|_| {
                    log::warn!("could not get local offset defaulting to utc");
                    UtcOffset::UTC
                })
            }
            TimeSpec::UTC => offset = UtcOffset::UTC,
            TimeSpec::OffsetFromUTC => {
                let (_, tmp_offset) = i32::parse(&b[9..13])?;
                pos += 4;
                offset = UtcOffset::seconds(tmp_offset)
            }
        }

        let date = Date::from_julian_day(julian_day as i64);

        let hour = millis_of_day / 60 / 60000;
        let minute = (millis_of_day - (hour * 60 * 60000)) / 60000;
        let seconds = (millis_of_day - (hour * 60 * 60000) - (minute * 60000)) / 1000;
        let millis = millis_of_day - (hour * 60 * 60000) - (minute * 60000) - (seconds * 1000);

        let time =
            Time::try_from_hms_milli(hour as u8, minute as u8, seconds as u8, millis as u16)?;
        let primitivedatetime = PrimitiveDateTime::new(date, time);
        let datetime = primitivedatetime.assume_offset(offset);

        Ok((pos, datetime))
    }
}

impl Serialize for Date {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, failure::Error> {
        let mut values: Vec<u8> = Vec::new();

        values.extend(i32::serialize(&(self.julian_day() as i32))?);

        Ok(values)
    }
}

impl Deserialize for Date {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), failure::Error> {
        let (_, julian_day) = i32::parse(&b[0..4])?;
        let date = Date::from_julian_day(julian_day as i64);

        Ok((4, date))
    }
}

impl Serialize for Time {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, failure::Error> {
        let mut values: Vec<u8> = Vec::new();

        let time: i32 = {
            let hour: i32 = self.hour() as i32;
            let minute: i32 = self.minute() as i32;
            let second: i32 = self.second() as i32;
            let milli: i32 = self.millisecond() as i32;

            milli + (second * 1000) + (minute * 60000) + (hour * 60 * 60000)
        };

        values.extend(i32::serialize(&time)?);

        Ok(values)
    }
}

impl Deserialize for Time {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), failure::Error> {
        let (_, millis_of_day) = i32::parse(&b[0..4])?;

        let hour = millis_of_day / 60 / 60000;
        let minute = (millis_of_day - (hour * 60 * 60000)) / 60000;
        let seconds = (millis_of_day - (hour * 60 * 60000) - (minute * 60000)) / 1000;
        let millis = millis_of_day - (hour * 60 * 60000) - (minute * 60000) - (seconds * 1000);

        let time =
            Time::try_from_hms_milli(hour as u8, minute as u8, seconds as u8, millis as u16)?;

        Ok((4, time))
    }
}

#[test]
pub fn datetime_serialize() {
    let datetime = DateTime::parse("2020-02-19 13:00 +0200", "%Y-%m-%d %R %z").unwrap();

    let sers = datetime.serialize().unwrap();
    let bytes = vec![0, 37, 133, 19, 2, 202, 28, 128, 3, 0, 0, 28, 32];

    assert_eq!(sers, bytes)
}

#[test]
pub fn datetime_deserialize() {
    let datetime = DateTime::parse("2020-02-19 13:00 +0200", "%Y-%m-%d %R %z").unwrap();

    let bytes = vec![0, 37, 133, 19, 2, 202, 28, 128, 3, 0, 0, 28, 32];
    let (_, res): (usize, DateTime) = Deserialize::parse(&bytes).unwrap();

    assert_eq!(res, datetime)
}

#[test]
pub fn datetime_deserialize_epoch() {
    let datetime = DateTime::unix_epoch();

    let bytes = vec![0, 37, 133, 19, 0xff, 0xff, 0xff, 0xff, 3, 0, 0, 28, 32];
    let (_, res): (usize, DateTime) = Deserialize::parse(&bytes).unwrap();

    let bytes = vec![0xff, 0xff, 0xff, 0xff, 2, 202, 28, 128, 3, 0, 0, 28, 32];
    let (_, res2): (usize, DateTime) = Deserialize::parse(&bytes).unwrap();

    assert_eq!(res, datetime);
    assert_eq!(res2, datetime)
}

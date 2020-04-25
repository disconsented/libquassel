use crate::protocol::primitive::deserialize::Deserialize;
use crate::protocol::primitive::serialize::Serialize;

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct DateTime {
    julian_day: i32,    // Day in Julian calendar, unknown if signed or unsigned
    millis_of_day: i32, // Milliseconds since start of day
    zone: u8,           // Timezone of DateTime, 0x00 is local, 0x01 is UTC
}

impl Serialize for DateTime {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, failure::Error> {
        let mut values: Vec<u8> = Vec::new();

        values.append(&mut i32::serialize(&self.julian_day)?);
        values.append(&mut i32::serialize(&self.millis_of_day)?);
        values.append(&mut u8::serialize(&(self.zone))?);

        Ok(values)
    }
}

impl Deserialize for DateTime {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), failure::Error>
    where
        Self: Sized,
    {
        let (_, julian_day) = i32::parse(&b[0..4])?;
        let (_, millis_of_day) = i32::parse(&b[4..8])?;
        let (_, zone) = u8::parse(&b[8..9])?;

        return Ok((
            9,
            DateTime {
                julian_day,
                millis_of_day,
                zone,
            },
        ));
    }
}

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct Date {
    julian_day: i32, // Day in Julian calendar, unknown if signed or unsigned
}

impl Serialize for Date {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, failure::Error> {
        let mut values: Vec<u8> = Vec::new();

        values.append(&mut i32::serialize(&self.julian_day)?);

        Ok(values)
    }
}

impl Deserialize for Date {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), failure::Error>
    where
        Self: Sized,
    {
        let (_, julian_day) = i32::parse(&b[0..4])?;

        return Ok((9, Date { julian_day }));
    }
}

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct Time {
    millis_of_day: i32, // Milliseconds since start of day
}

impl Serialize for Time {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, failure::Error> {
        let mut values: Vec<u8> = Vec::new();

        values.append(&mut i32::serialize(&self.millis_of_day)?);

        Ok(values)
    }
}

impl Deserialize for Time {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), failure::Error>
    where
        Self: Sized,
    {
        let (_, millis_of_day) = i32::parse(&b[0..4])?;

        return Ok((4, Time { millis_of_day }));
    }
}

use std::fmt;
use std::str::FromStr;
use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Discord epoch in milliseconds: 2015-01-01T00:00:00Z
pub const DISCORD_EPOCH: u64 = 1420070400000;

/// 64-bit Discord Snowflake identifier.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Snowflake(pub u64);

impl Snowflake {
    #[inline]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    #[inline]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    /// Extract creation timestamp in milliseconds since Unix epoch
    #[inline]
    pub const fn timestamp_millis(&self) -> u64 {
        (self.0 >> 22) + DISCORD_EPOCH
    }

    /// Extract creation DateTime in UTC
    pub fn timestamp(&self) -> DateTime<Utc> {
        Utc.timestamp_millis_opt(self.timestamp_millis() as i64)
            .single()
            .unwrap_or_else(Utc::now)
    }

    /// Internal worker ID that generated this snowflake (bits 17-21)
    #[inline]
    pub const fn worker_id(&self) -> u8 {
        ((self.0 & 0x3E0000) >> 17) as u8
    }

    /// Internal process ID that generated this snowflake (bits 12-16)
    #[inline]
    pub const fn process_id(&self) -> u8 {
        ((self.0 & 0x1F000) >> 12) as u8
    }

    /// Sequence increment (bits 0-11)
    #[inline]
    pub const fn increment(&self) -> u16 {
        (self.0 & 0xFFF) as u16
    }
}

impl fmt::Display for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Snowflake({})", self.0)
    }
}

impl From<u64> for Snowflake {
    fn from(val: u64) -> Self {
        Self(val)
    }
}

impl From<Snowflake> for u64 {
    fn from(s: Snowflake) -> Self {
        s.0
    }
}

impl FromStr for Snowflake {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s.parse::<u64>()?;
        Ok(Self(val))
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Discord serializes 64-bit snowflakes as string to avoid JavaScript float precision loss
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeVisitor;

        impl<'de> serde::de::Visitor<'de> for SnowflakeVisitor {
            type Value = Snowflake;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a snowflake as a string or integer")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse::<u64>()
                    .map(Snowflake)
                    .map_err(serde::de::Error::custom)
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Snowflake(v))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v >= 0 {
                    Ok(Snowflake(v as u64))
                } else {
                    Err(serde::de::Error::custom("snowflake cannot be negative"))
                }
            }
        }

        deserializer.deserialize_any(SnowflakeVisitor)
    }
}

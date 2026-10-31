//! Bounded parsing of one HTTP byte range, without allocating a media buffer.
//!
//! Supports `bytes=first-last`, `bytes=first-` and `bytes=-suffix` for resource
//! lengths representable as `u64`. Multiple ranges, other units, oversized
//! values and malformed syntax are explicitly unsupported. This is not a full
//! HTTP server: conditional requests and response status policy belong to the
//! caller. See RFC 9110, sections 14.1.2 and 14.2, for the wider protocol.

use std::fmt;

/// Why a single range could not be selected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RangeError {
    /// Syntax, unit, size limit or integer representation is unsupported.
    Malformed,
    /// More than one byte range was supplied.
    MultipleRanges,
    /// The requested interval does not intersect a nonempty resource.
    Unsatisfiable,
}

impl fmt::Display for RangeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Malformed => "malformed or unsupported range",
            Self::MultipleRanges => "multiple ranges are not supported by this example",
            Self::Unsatisfiable => "range does not intersect the resource",
        })
    }
}

impl std::error::Error for RangeError {}

/// A nonempty inclusive range validated against its resource length.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ByteRange {
    first: u64,
    last: u64,
    complete_length: u64,
}

impl ByteRange {
    /// First selected byte, counted from zero.
    pub const fn first(self) -> u64 {
        self.first
    }

    /// Last selected byte, inclusive.
    pub const fn last(self) -> u64 {
        self.last
    }

    /// Number of selected bytes; always at least one.
    pub const fn byte_len(self) -> u64 {
        self.last - self.first + 1
    }

    /// Format Content-Range using the same length used for validation.
    pub fn content_range(self) -> String {
        format!(
            "bytes {}-{}/{}",
            self.first, self.last, self.complete_length
        )
    }
}

/// Parse one range against a known resource length.
///
/// The result stores only integer bounds. Streaming or slicing the media is a
/// separate operation. Bounds above the end are clamped when the first byte is
/// satisfiable. A zero suffix or an empty resource is unsatisfiable.
pub fn parse_single_range(header: &str, complete_length: u64) -> Result<ByteRange, RangeError> {
    if header.len() > 128
        || header
            .bytes()
            .any(|byte| (byte < 32 && byte != b'\t') || byte == 127)
    {
        return Err(RangeError::Malformed);
    }
    let (unit, bounds) = header.trim().split_once('=').ok_or(RangeError::Malformed)?;
    if !unit.eq_ignore_ascii_case("bytes") {
        return Err(RangeError::Malformed);
    }
    if bounds.contains(',') {
        return Err(RangeError::MultipleRanges);
    }
    let (first, last) = bounds.split_once('-').ok_or(RangeError::Malformed)?;
    if first.is_empty() && last.is_empty() {
        return Err(RangeError::Malformed);
    }
    let start = if first.is_empty() {
        None
    } else {
        Some(decimal(first)?)
    };
    let end = if last.is_empty() {
        None
    } else {
        Some(decimal(last)?)
    };
    if complete_length == 0 {
        return Err(RangeError::Unsatisfiable);
    }
    let (first, last) = match (start, end) {
        (None, Some(0)) => return Err(RangeError::Unsatisfiable),
        (None, Some(suffix)) => (complete_length.saturating_sub(suffix), complete_length - 1),
        (Some(first), requested_last) => {
            if first >= complete_length || requested_last.is_some_and(|last| last < first) {
                return Err(RangeError::Unsatisfiable);
            }
            (
                first,
                requested_last
                    .unwrap_or(complete_length - 1)
                    .min(complete_length - 1),
            )
        }
        (None, None) => return Err(RangeError::Malformed),
    };
    Ok(ByteRange {
        first,
        last,
        complete_length,
    })
}

fn decimal(value: &str) -> Result<u64, RangeError> {
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(RangeError::Malformed);
    }
    value.parse().map_err(|_| RangeError::Malformed)
}

use paulinsider_web_foundations::media_range::{parse_single_range, RangeError};

#[test]
fn common_video_requests_select_the_exact_bytes() {
    for (header, first, last) in [
        ("bytes=0-31", 0, 31),
        ("bytes=9-23", 9, 23),
        ("bytes=81-", 81, 90),
        ("bytes=-7", 84, 90),
        ("bytes=80-999", 80, 90),
        ("bytes=-999", 0, 90),
        ("BYTES=0-0", 0, 0),
    ] {
        let range = parse_single_range(header, 91).expect(header);
        assert_eq!((range.first(), range.last()), (first, last));
        assert_eq!(range.byte_len(), last - first + 1);
        assert_eq!(range.content_range(), format!("bytes {first}-{last}/91"));
    }
}

#[test]
fn impossible_intervals_and_empty_resources_are_rejected() {
    for header in ["bytes=5-9", "bytes=4-2", "bytes=-0", "bytes=99-"] {
        assert_eq!(
            parse_single_range(header, 5),
            Err(RangeError::Unsatisfiable),
            "{header}"
        );
    }
    for header in ["bytes=0-0", "bytes=0-", "bytes=-1"] {
        assert_eq!(
            parse_single_range(header, 0),
            Err(RangeError::Unsatisfiable)
        );
    }
}

#[test]
fn malformed_and_multiple_ranges_are_distinguished() {
    for header in [
        "",
        "items=0-1",
        "bytes=-",
        "bytes=+1-2",
        "bytes=0-1-2",
        "bytes=0- 1",
        "bytes=1.5-2",
        "bytes=one-two",
        "bytes=0-1\r\n",
        "bytes=18446744073709551616-",
    ] {
        assert_eq!(
            parse_single_range(header, 10),
            Err(RangeError::Malformed),
            "{header}"
        );
    }
    assert_eq!(
        parse_single_range("bytes=0-1,3-4", 10),
        Err(RangeError::MultipleRanges)
    );
    assert_eq!(
        parse_single_range(&format!("bytes={}-", "1".repeat(129)), 10),
        Err(RangeError::Malformed)
    );
}

#[test]
fn maximum_representable_lengths_do_not_overflow() {
    let full = parse_single_range("bytes=0-", u64::MAX).unwrap();
    assert_eq!(full.byte_len(), u64::MAX);
    assert_eq!(full.last(), u64::MAX - 1);
    let suffix = parse_single_range("bytes=-18446744073709551615", u64::MAX).unwrap();
    assert_eq!(suffix, full);
    assert_eq!(
        parse_single_range("bytes=18446744073709551615-", u64::MAX),
        Err(RangeError::Unsatisfiable)
    );
}

#[test]
fn bounded_intervals_match_real_slices_across_small_resource_sizes() {
    for size in 1..=32_u64 {
        let bytes: Vec<u8> = (0..size).map(|n| n as u8).collect();
        for first in 0..=40_u64 {
            for last in first..=40_u64 {
                let parsed = parse_single_range(&format!("bytes={first}-{last}"), size);
                if first >= size {
                    assert_eq!(parsed, Err(RangeError::Unsatisfiable));
                } else {
                    let range = parsed.unwrap();
                    let selected = &bytes[range.first() as usize..=range.last() as usize];
                    assert_eq!(
                        selected,
                        &bytes[first as usize..=(last.min(size - 1)) as usize]
                    );
                    assert_eq!(selected.len() as u64, range.byte_len());
                }
            }
        }
    }
}

#[test]
fn suffixes_and_open_ranges_stay_within_the_resource() {
    for size in 1..=32_u64 {
        for suffix in 1..=40_u64 {
            let range = parse_single_range(&format!("bytes=-{suffix}"), size).unwrap();
            assert_eq!(range.byte_len(), suffix.min(size));
            assert_eq!(range.last(), size - 1);
        }
        for first in 0..size {
            let range = parse_single_range(&format!("bytes={first}-"), size).unwrap();
            assert_eq!(range.byte_len(), size - first);
            assert_eq!(range.last(), size - 1);
        }
    }
}

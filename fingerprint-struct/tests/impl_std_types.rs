use std::{
    marker::PhantomData,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    ops::Bound,
    time::Duration,
};

#[cfg(all(feature = "std", feature = "os"))]
use std::time::SystemTime;

use fingerprint_struct::Fingerprint;
use mock_digest::MockDigest;

fn assert_same_fingerprint<A: Fingerprint, B: Fingerprint>(a: A, b: B) {
    let mut hasher_a = MockDigest::default();
    a.fingerprint(&mut hasher_a);

    let mut hasher_b = MockDigest::default();
    b.fingerprint(&mut hasher_b);

    assert_eq!(hasher_a.as_ref(), hasher_b.as_ref());
}

#[test]
fn fingerprint_option() {
    assert_same_fingerprint(Some(42), (0u8, 42));
    assert_same_fingerprint(None::<u8>, 1u8);
}

#[test]
fn fingerprint_result() {
    assert_same_fingerprint(Ok::<u8, u16>(42), (0u8, 42u8));
    assert_same_fingerprint(Err::<u8, u16>(42), (1u8, 42u16));
}

#[test]
fn fingerprint_phantom_data() {
    struct Unfingerprintable;

    assert_same_fingerprint(PhantomData::<Unfingerprintable>, ());
}

#[test]
fn fingerprint_range() {
    assert_same_fingerprint(5..10, (5, 10));
}

#[test]
fn fingerprint_range_inclusive() {
    assert_same_fingerprint(5..=10, (5, 10));
}

#[test]
fn fingerprint_bound() {
    assert_same_fingerprint(Bound::Included(1), (0u8, 1));
    assert_same_fingerprint(Bound::Excluded(1), (1u8, 1));
    assert_same_fingerprint(Bound::Unbounded::<i32>, 2u8);
}

#[test]
fn fingerprint_duration() {
    assert_same_fingerprint(Duration::new(123, 456), 123000000456u128);
}

#[test]
#[cfg_attr(not(feature = "os"), ignore)]
fn fingerprint_system_time() {
    #[cfg(feature = "os")]
    {
        assert_same_fingerprint(
            SystemTime::UNIX_EPOCH + Duration::new(1, 0),
            (0u8, Duration::new(1, 0)),
        );
        assert_same_fingerprint(
            SystemTime::UNIX_EPOCH - Duration::new(1, 0),
            (1u8, Duration::new(1, 0)),
        );
    }
}

#[test]
fn fingerprint_ip_address() {
    let address = Ipv4Addr::new(192, 168, 0, 1);
    assert_same_fingerprint(IpAddr::V4(address), (4u8, address));

    let address = Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 8);
    assert_same_fingerprint(IpAddr::V6(address), (6u8, address));
}

#[test]
fn fingerprint_ipv4_address() {
    assert_same_fingerprint(Ipv4Addr::new(192, 168, 0, 1), (192u8, 168u8, 0u8, 1u8));
}

#[test]
fn fingerprint_ipv6_address() {
    assert_same_fingerprint(
        Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 8),
        0x00010002000300040005000600070008u128.to_be_bytes(),
    );
}

#[test]
fn fingerprint_socket_address() {
    let address = SocketAddrV4::new(Ipv4Addr::new(192, 168, 0, 1), 8080);
    assert_same_fingerprint(SocketAddr::V4(address), (4u8, address));

    let address = SocketAddrV6::new(Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 8), 8080, 1, 2);
    assert_same_fingerprint(SocketAddr::V6(address), (6u8, address));
}

#[test]
fn fingerprint_socketv4_address() {
    assert_same_fingerprint(
        SocketAddrV4::new(Ipv4Addr::new(192, 168, 0, 1), 8080),
        ((192u8, 168u8, 0u8, 1u8), 8080u16),
    );
}

#[test]
fn fingerprint_socketv6_address() {
    assert_same_fingerprint(
        SocketAddrV6::new(Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 8), 8080, 1, 2),
        (
            0x00010002000300040005000600070008u128.to_be_bytes(),
            8080u16,
            1,
            2,
        ),
    );
}

/// [RFC 7518, Section 3.2](https://datatracker.ietf.org/doc/html/rfc7518#section-3.2)
///
/// > A key of the same size as the hash output (for instance, 256 bits for
/// > "HS256") or larger MUST be used with this algorithm.  (This
/// > requirement is based on Section 5.3.4 (Security Effect of the HMAC
/// > Key) of NIST SP 800-117 [NIST.800-107], which states that the
/// > effective security strength is the minimum of the security strength
/// > of the key and two times the size of the internal hash value.)
///
/// [NIST.800-107]: http://csrc.nist.gov/publications/nistpubs/800-107-rev1/sp800-107-rev1.pdf
pub const HS256_SECRET_KEY_LEN: usize = 64;

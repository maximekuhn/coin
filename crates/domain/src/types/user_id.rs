use crate::id_type;

id_type!(UserId);

#[cfg(test)]
mod tests {
    use super::{Error, UserId};

    #[rstest::rstest]
    #[case("019b14ca-c11a-7882-ac00-0e88e8ba5e84")]
    #[case("019b14cac11a7882ac000e88e8ba5e84")]
    #[case("      019b14cac11a7882ac000e88e8ba5e84")]
    #[case("019b14ca-c11a-7882-ac00-0e88e8ba5e84     ")]
    fn valid_user_id(#[case] input: &str) {
        let user_id: UserId = input.parse().unwrap();
        assert_eq!(
            input.trim().replace('-', ""),
            user_id.value().to_string().replace('-', "")
        );
    }

    #[rstest::rstest]
    #[case("", Error::Malformed)]
    #[case("00000000-0000-0000-0000-000000000000", Error::ZerosOnly)]
    #[case("00000000000000000000000000000000", Error::ZerosOnly)]
    #[case("ffffffff-ffff-ffff-ffff-ffffffffffff", Error::OnesOnly)]
    #[case("ffffffffffffffffffffffffffffffff", Error::OnesOnly)]
    fn invalid_user_id(#[case] input: &str, #[case] expected_err: Error) {
        let err = input.parse::<UserId>().unwrap_err();
        assert_eq!(expected_err, err);
    }

    #[test]
    fn new_random_works() {
        let id = UserId::new_random();
        assert!(!id.value().is_nil());
        assert!(!id.value().is_max());
    }
}

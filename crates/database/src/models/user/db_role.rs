use domain::types::role::Role;

pub struct DbRole(pub u8);

impl From<&Role> for DbRole {
    fn from(r: &Role) -> Self {
        Self(match r {
            Role::User => 10,
            Role::Moderator => 20,
            Role::Admin => 30,
        })
    }
}

impl TryInto<Role> for DbRole {
    type Error = crate::Error;

    fn try_into(self) -> Result<Role, Self::Error> {
        match self.0 {
            10 => Ok(Role::User),
            20 => Ok(Role::Moderator),
            30 => Ok(Role::Admin),
            other => Err(crate::Error::CorruptedData {
                msg: format!("unknown role: '{}'", other),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DbRole;
    use domain::types::role::Role;

    #[rstest::rstest]
    #[case(Role::User, 10)]
    #[case(Role::Moderator, 20)]
    #[case(Role::Admin, 30)]
    fn from_domain_to_db(#[case] role: Role, #[case] expected_db_value: u8) {
        let db_role = DbRole::from(&role);
        assert_eq!(expected_db_value, db_role.0);
    }

    #[rstest::rstest]
    #[case(10, Role::User)]
    #[case(20, Role::Moderator)]
    #[case(30, Role::Admin)]
    fn from_db_to_domain_ok(#[case] db_role: u8, #[case] expected_role: Role) {
        let role: Role = DbRole(db_role).try_into().unwrap();
        assert_eq!(expected_role, role);
    }

    #[test]
    fn from_db_to_domain_invalid() {
        let invalid_db_role = 27;
        let err = TryInto::<Role>::try_into(DbRole(invalid_db_role)).unwrap_err();
        let err_msg = err.to_string();
        assert_eq!("database corrupted data: unknown role: '27'", err_msg);
    }
}

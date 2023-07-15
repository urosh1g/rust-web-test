/*
 *  Ne koristi se nigde, prvo sam koristio jsonwebtoken
 *  biblioteku, i imao auth.rs koji generise / verifikuje tokene
 *  ali nisam uspeo da napravim middleware, nisam bas pohvatao
 *  sve unutar Transform i Service traits iz actix-web
 *  pa sam rucno proveravao u handlerima iz cookie-a
 *  da li je token validan sto je ruzno
 *  probao sam actix_jwt_auth_middleware
 *  ali sam uspeo samo ceo scope da bude protected
 *  umesto pojedinih handlera sto mi se ne svidja
 *  i na kraju ostadoh bez ikakve auth i authz
 */
use super::user::User;
use actix_jwt_auth_middleware::FromRequest;
use chrono::{self, offset::Utc};

#[derive(serde::Serialize, serde::Deserialize, Clone, FromRequest)]
pub struct UserClaims {
    pub iat: usize,
    pub sub: i32,
    pub email: String,
    pub aud: String,
    pub exp: usize,
    pub iss: String,
}

impl UserClaims {
    // na zalost std::convert::From consumuje parametar
    // a meni treba samo ref na njega
    // pa nisam implementirao trait
    pub fn from(user: &User) -> Self {
        Self {
            sub: user.user_id,
            email: user.email.clone(),
            exp: (Utc::now() + chrono::Duration::minutes(60)).timestamp() as usize,
            aud: String::from("audience"),
            iss: String::from("uros"),
            iat: Utc::now().timestamp() as usize,
        }
    }
}

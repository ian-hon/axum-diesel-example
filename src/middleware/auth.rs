use anyhow::{Context as _, ensure};
use axum::extract::{Request, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::middleware::Next;
use axum::response::{Response, Result};
use biscuit::jwa::SignatureAlgorithm;
use biscuit::{ClaimPresenceOptions, JWT, Presence, Validation, ValidationOptions, jws};
use chrono::TimeDelta;
use jiff::SpanRelativeTo;
use secrecy::ExposeSecret as _;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::{
    AccessTokenAudience, AccessTokenClientId, AccessTokenExpiration, AccessTokenIssuer,
    JwsSigningSecret,
};

const BEARER_PREFIX: &str = "Bearer ";

#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    /// [RFC 9068, Section 2.2](https://datatracker.ietf.org/doc/html/rfc9068#section-2.2)
    ///
    /// > In cases of access tokens obtained through grants where a resource
    /// > owner is involved, such as the authorization code grant, the value of
    /// > "sub" SHOULD correspond to the subject identifier of the resource
    /// > owner.
    pub subject: Uuid,
}

/// [RFC 9068, Section 2.2](https://datatracker.ietf.org/doc/html/rfc9068#section-2.2)
#[derive(Debug, Deserialize, Serialize)]
pub struct JwtAccessTokenClaims {
    /// [RFC 8693, Section 4.3](https://datatracker.ietf.org/doc/html/rfc8693#section-4.3)
    ///
    /// > The client_id claim carries the client identifier of the OAuth 2.0
    /// > [[RFC6749]] client that requested the token.
    ///
    /// [[RFC6749]]: https://datatracker.ietf.org/doc/html/rfc6749
    pub client_id: String,
}

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum TokenUse {
    Id,
    Access,
}

/// Authenticates the user with a nested JWT, containing an OAuth 2.0 JWT access
/// token.
///
/// * [RFC 6750](https://datatracker.ietf.org/doc/html/rfc6750)
/// * [RFC 9068](https://datatracker.ietf.org/doc/html/rfc9068)
#[allow(clippy::too_many_arguments)]
pub async fn authenticate_with_jwt_access_token(
    State(jws_signing_secret): State<JwsSigningSecret>,
    State(access_token_expiration): State<AccessTokenExpiration>,
    State(access_token_issuer): State<AccessTokenIssuer>,
    State(access_token_audience): State<AccessTokenAudience>,
    State(access_token_client_id): State<AccessTokenClientId>,
    request_headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    // [RFC 6750, Section 3.1](https://datatracker.ietf.org/doc/html/rfc6750#section-3.1)
    //
    // > If the request lacks any authentication information (e.g., the client
    // > was unaware that authentication is necessary or attempted using an
    // > unsupported authentication method), the resource server SHOULD NOT
    // > include an error code or other error information.
    let Some(authorization_header_value) = request_headers.get(header::AUTHORIZATION) else {
        return Err((StatusCode::UNAUTHORIZED, [(
            header::WWW_AUTHENTICATE,
            "Bearer",
        )]))?;
    };

    // [RFC 6750, Section 3.1](https://datatracker.ietf.org/doc/html/rfc6750#section-3.1)
    //
    // > The request is missing a required parameter, includes an
    // > unsupported parameter or parameter value, repeats the same
    // > parameter, uses more than one method for including an access
    // > token, or is otherwise malformed.  The resource server SHOULD
    // > respond with the HTTP 400 (Bad Request) status code.
    let Ok(authorization_header_value) = authorization_header_value.to_str() else {
        return Err((StatusCode::BAD_REQUEST, [(
            header::WWW_AUTHENTICATE,
            "Bearer error=\"invalid_request\",error_description=\"The Authorization header value \
             contains invalid ASCII\"",
        )]))?;
    };

    // [RFC 6750, Section 3.1](https://datatracker.ietf.org/doc/html/rfc6750#section-3.1)
    //
    // > If the request lacks any authentication information (e.g., the client
    // > was unaware that authentication is necessary or attempted using an
    // > unsupported authentication method), the resource server SHOULD NOT
    // > include an error code or other error information.
    let Some(bearer_token) = authorization_header_value
        .split_at_checked(BEARER_PREFIX.len())
        .and_then(|(scheme, token)| {
            if scheme.eq_ignore_ascii_case(BEARER_PREFIX) {
                Some(token.trim_start_matches(' '))
            } else {
                None
            }
        })
    else {
        return Err((StatusCode::UNAUTHORIZED, [(
            header::WWW_AUTHENTICATE,
            "Bearer",
        )]))?;
    };

    let mut access_token: JWT<JwtAccessTokenClaims, biscuit::Empty> =
        JWT::new_encoded(bearer_token);

    // [RFC 6750, Section 3.1](https://datatracker.ietf.org/doc/html/rfc6750#section-3.1)
    //
    // > The access token provided is expired, revoked, malformed, or
    // > invalid for other reasons.  The resource SHOULD respond with
    // > the HTTP 401 (Unauthorized) status code.  The client MAY
    // > request a new access token and retry the protected resource
    // > request.
    if let Err(err) = decode_access_token(&mut access_token, &jws_signing_secret) {
        return Err((StatusCode::UNAUTHORIZED, [(
            header::WWW_AUTHENTICATE,
            format!("Bearer error=\"invalid_token\",error_description=\"{err}\""),
        )]))?;
    }

    // [RFC 6750, Section 3.1](https://datatracker.ietf.org/doc/html/rfc6750#section-3.1)
    //
    // > The access token provided is expired, revoked, malformed, or
    // > invalid for other reasons.  The resource SHOULD respond with
    // > the HTTP 401 (Unauthorized) status code.  The client MAY
    // > request a new access token and retry the protected resource
    // > request.
    if let Err(err) = validate_access_token(
        &access_token,
        access_token_expiration,
        access_token_issuer,
        access_token_audience,
        access_token_client_id,
    ) {
        return Err((StatusCode::UNAUTHORIZED, [(
            header::WWW_AUTHENTICATE,
            format!("Bearer error=\"invalid_token\",error_description=\"{err}\""),
        )]))?;
    }

    let claims = access_token
        .payload()
        .expect("`access_token` should have been decoded");

    // [RFC 9068, Section 2.2](https://datatracker.ietf.org/doc/html/rfc9068#section-2.2)
    //
    // > In cases of access tokens obtained through grants where a resource owner is
    // > involved, such as the authorization code grant, the value of "sub" SHOULD
    // > correspond to the subject identifier of the resource owner.
    let subject = claims
        .registered
        .subject
        .as_ref()
        .expect("\"sub\" claim should be present in access token");
    let subject = match Uuid::try_parse(subject) {
        Ok(subject) => subject,
        Err(_err) => {
            return Err((StatusCode::UNAUTHORIZED, [(
                header::WWW_AUTHENTICATE,
                "Bearer error=\"invalid_token\",error_description=\"The subject identifier is not \
                 a valid UUID\"",
            )]))?;
        },
    };

    let authenticated_user = AuthenticatedUser { subject };

    request.extensions_mut().insert(authenticated_user);

    let response = next.run(request).await;

    Ok(response)
}

/// [RFC 7519, Section 7.2](https://datatracker.ietf.org/doc/html/rfc7519#section-7.2)
/// [RFC 7515, Section 5.2](https://datatracker.ietf.org/doc/html/rfc7515#section-5.2)
fn decode_access_token(
    access_token: &mut JWT<JwtAccessTokenClaims, biscuit::Empty>,
    jws_signing_secret: &JwsSigningSecret,
) -> Result<(), anyhow::Error> {
    // This is an early return if the token is already decoded
    if let JWT::Decoded { .. } = *access_token {
        return Ok(());
    }

    *access_token = access_token
        .decode(
            &jws::Secret::Bytes(jws_signing_secret.0.expose_secret().to_vec()),
            SignatureAlgorithm::HS256,
        )
        .context("failed to decode access token")?;

    Ok(())
}

/// [RFC 9068, Section 2.2](https://datatracker.ietf.org/doc/html/rfc9068#section-2.2)
fn validate_access_token(
    access_token: &JWT<JwtAccessTokenClaims, biscuit::Empty>,
    access_token_expiration: AccessTokenExpiration,
    access_token_issuer: AccessTokenIssuer,
    access_token_audience: AccessTokenAudience,
    access_token_client_id: AccessTokenClientId,
) -> Result<(), anyhow::Error> {
    let header = access_token
        .header()
        .expect("`access_token` should have been decoded");

    ensure!(
        header.registered.media_type == Some("at+jwt".to_owned())
            || header.registered.media_type == Some("application/at+jwt".to_owned()),
        "access token \"typ\" header parameter mismatch"
    );

    let claims = access_token
        .payload()
        .expect("`access_token` should have been decoded");

    let access_token_max_age = access_token_expiration
        .0
        .to_duration(SpanRelativeTo::days_are_24_hours())
        .map_err(anyhow::Error::from)
        .and_then(|duration| std::time::Duration::try_from(duration).map_err(anyhow::Error::from))
        .and_then(|duration| TimeDelta::from_std(duration).map_err(anyhow::Error::from))
        .expect("converting `access_token_expiration` should not fail");

    access_token
        .validate(ValidationOptions {
            claim_presence_options: ClaimPresenceOptions {
                issuer: Presence::Required,
                expiry: Presence::Required,
                audience: Presence::Required,
                subject: Presence::Required,
                issued_at: Presence::Required,
                id: Presence::Required,
                ..Default::default()
            },
            issuer: Validation::Validate(access_token_issuer.0.as_str().to_owned()),
            expiry: Validation::Validate(()),
            audience: Validation::Validate(access_token_audience.0.as_str().to_owned()),
            issued_at: Validation::Validate(access_token_max_age),
            ..Default::default()
        })
        .context("failed to validate access token")?;

    ensure!(
        claims.private.client_id == access_token_client_id.0.to_string(),
        "access token \"client_id\" claim mismatch"
    );

    Ok(())
}

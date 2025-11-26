# WORK IN PROGRESS

This crate will be an implementation of an openid-connect (OIDC) client in rust. For right now I do
not see fully maintaining it or implementing every flow as I simply do not need it.

## Todo List

- [ ] Create a handler for `Discovery`

## So why a new crate?

There does not seem to be much of a platform for OIDC in rust. Maybe i'm wrong or over thinking it,
but the few crates that I have found despite being helpful are a bit too rigid. Of course the
rigidity is very helpful for a spec, however as it seems to me google does not fully implement the
`iss` claim on the id_token correctly. The response can be one of two values
`https://accounts.google.com` or `accounts.google.com`. This is very frustrating because most of the
crates use `url::Url` which is fantastic aside from that issuer claim. So maybe I will fail and
maybe this is all in vain, but maybe not...

## Useful links

- [discovery-spec](https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata)
- [google-discovery](https://accounts.google.com/.well-known/openid-configuration)
- [go-oidc](https://github.com/coreos/go-oidc/tree/v3)
- [auth0-discovery-help](https://auth0.com/docs/get-started/applications/configure-applications-with-oidc-discovery)

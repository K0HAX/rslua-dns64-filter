use netaddr2::{Error as NetError};
use std::fmt;
use std::net::*;
use mlua::prelude::*;
use mlua::ExternalError;
use hickory_resolver::Resolver;

#[derive(Debug, Clone)]
enum MyClientError {
    NetError(String),
    ResolverCreationError,
    DnsLookupError
}

impl std::fmt::Display for MyClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyClientError::NetError(e) => {
                write!(f, "{}", e)
            },
            MyClientError::ResolverCreationError => {
                write!(f, "Could not create resolver")
            },
            MyClientError::DnsLookupError => {
                write!(f, "DNS Lookup Failed")
            }
        }
    }
}

impl std::error::Error for MyClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            MyClientError::NetError(ref _e) => Some(self),
            MyClientError::ResolverCreationError => Some(self),
            MyClientError::DnsLookupError => Some(self),
        }
    }
}

impl From<MyClientError> for LuaError {
    fn from(err: MyClientError) -> LuaError {
        err.into_lua_err()
    }
}

impl From<NetError> for MyClientError {
    fn from(err: NetError) -> MyClientError {
        match err {
            NetError::ParseError(e) => MyClientError::NetError(e)
        }
    }
}

fn normalize_fqdn(fqdn_in: String) -> String {
    let last_char: String = {
        let split_pos = fqdn_in.char_indices().nth_back(0).unwrap().0;
        (&fqdn_in[split_pos..]).to_string()
    };
    if last_char == "." {
        return fqdn_in;
    }
    let fqdn: String = format!("{}.", fqdn_in);
    return fqdn;
}

fn check_record(_: &Lua, in_name: String) -> LuaResult<bool> {
    let (resolver_config, mut resolver_opts) = hickory_resolver::system_conf::read_system_conf().unwrap();
    resolver_opts.validate = false;
    let resolver = Resolver::new(resolver_config, resolver_opts).map_err(|_| MyClientError::ResolverCreationError)?;
    let normalized_name: String = normalize_fqdn(in_name);
    let response = resolver.ipv4_lookup(normalized_name).map_err(|e| {
        eprintln!("DNS Lookup Error: {:#?}", e);
        MyClientError::DnsLookupError
    })?;
    for answer in response.iter() {
        let ip: Ipv4Addr = answer.0;
        eprintln!("Answer: {:#?}", ip);
        return Ok(ip.is_private());
    }
    Ok(false)
}

#[mlua::lua_module]
fn dns64_filter(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("check_record", lua.create_function(check_record)?)?;
    Ok(exports)
}

#[cfg(test)]
mod tests {
    // Importing names from outer (from mod tests' perspective) scope.
    use super::*;

    #[test]
    fn external_duckduckgo() {
        let lua: Lua = Lua::new();
        let dns_name = "duckduckgo.com".to_string();
        let result = check_record(&lua, dns_name).unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn external_google() {
        let lua: Lua = Lua::new();
        let dns_name = "google.com".to_string();
        let result = check_record(&lua, dns_name).unwrap();
        assert_eq!(result, false);
    }
}

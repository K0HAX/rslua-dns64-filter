use netaddr2::{Contains, Error as NetError, NetAddr};
use std::error::Error;
use std::net::*;
use mlua::prelude::*;
use hickory_resolver::Resolver;
use hickory_resolver::config::*;

fn compare_subnet(addr: &str, subnet: &str) -> Result<bool, Box<dyn Error>> {
    match subnet.parse::<NetAddr>() {
        Ok(NetAddr::V4(subnet4)) => {
            if let Ok(addr) = addr.parse::<Ipv4Addr>() {
                let is_in = subnet4.contains(&addr);
                Ok(is_in)
            } else {
                Ok(false)
            }
        }
        Ok(NetAddr::V6(subnet6)) => {
            if let Ok(addr) = addr.parse::<Ipv6Addr>() {
                let is_in = subnet6.contains(&addr);
                Ok(is_in)
            } else {
                Ok(false)
            }
        }
        Err(NetError::ParseError(e)) => Err(e.into()),
    }
}


fn check_record(_: &Lua, ip: String) -> LuaResult<bool> {
    let v4_private_networks: Vec<&'static str> = Vec::from(["10.0.0.0/8", "192.168.0.0/16", "172.16.0.0/12"]);
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let response = resolver.lookup_ip(&ip).expect("something wrong with resolver");
    //let address = response.iter().next().expect("no address returned!");
    let mut result = false;
    for address in response.iter() {
        if address.is_ipv4() {
            for net in v4_private_networks.iter() {
                let this_result = compare_subnet(&address.to_string(), net).expect("Failed to compare subnets!");
                if this_result == true {
                    result = true;
                }
            }
        }
    }
    Ok(result)
}

#[mlua::lua_module]
fn dns64_filter(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("check_record", lua.create_function(check_record)?)?;
    Ok(exports)
}

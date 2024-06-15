dnsfilter_check = require("dns64_filter")

local mainprefix = "fd1c:5cfc:918c::"
local mainzone = newDN("0.0.0.0.0.0.0.0.0.0.0.0.c.8.1.9.c.f.c.5.c.1.d.f.ip6.arpa.")

local globalprefix = "64:ff9b::"
local globalzone = newDN("0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.b.9.f.f.4.6.0.0.ip6.arpa.")

function preresolve(dq)
	if dq.qtype == pdns.PTR then
		if dq.qname:isPartOf(mainzone) then
			dq.followupFunction = "getFakePTRRecords"
			dq.followupPrefix = mainprefix
			dq.followupName = dq.qname
			return true
		end
		if dq.qname:isPartOf(globalzone) then
			dq.followupFunction = "getFakePTRRecords"
			dq.followupPrefix = globalprefix
			dq.followupName = dq.qname
			return true
		end
	end

	return false
end

function nodata ( dq )
        if dq.qtype == pdns.AAAA then
                dq.followupFunction = "getFakeAAAARecords"
                dq.followupName = dq.qname
                dq.variable = true
                local err, i = pcall(dnsfilter_check.check_record,dq.qname:toString())
                if err == false then
                        pdnslog("Error parsing record")
                        return false
                end
                if i and not dq.qname:equal("ipv4only.arpa") then
                        dq.followupPrefix=mainprefix
                else
                        dq.followupPrefix=globalprefix
                end
                return true
        end
        return false
end

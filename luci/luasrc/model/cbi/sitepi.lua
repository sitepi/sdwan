local m, s, o

m = Map("sitepi", translate("SitePi SDWAN"),
    translate("Configure SitePi SDWAN client instances. Please configure WireGuard interface settings in Network -> Interfaces."))

-- Global Settings
s = m:section(TypedSection, "sitepi", translate("Global Settings"))
s.anonymous = true
s.addremove = false

o = s:option(Flag, "enabled", translate("Enable"))
o.rmempty = false
o.default = "0"

-- Instance List
s = m:section(TypedSection, "instance", translate("Instances"))
s.anonymous = true
s.addremove = true
s.template = "cbi/tblsection"
s.sortable = true

o = s:option(Flag, "enabled", translate("Enable"))
o.rmempty = false
o.default = "0"
o.width = "10%"

o = s:option(Value, "name", translate("Name"))
o.rmempty = false
o.validate = function(self, value, section)
    if value and #value > 0 then
        if not value:match("^[a-zA-Z0-9_]+$") then
            return nil, translate("Name must only contain alphanumeric characters and underscore")
        end
        -- 检查名称唯一性
        local count = 0
        m.uci:foreach("sitepi", "instance", function(s)
            if s[".name"] ~= section and s.name == value then
                count = count + 1
            end
        end)
        if count > 0 then
            return nil, translate("Instance name must be unique")
        end
        return value
    end
    return nil, translate("Name cannot be empty")
end
o.width = "15%"

o = s:option(Value, "host", translate("Server Host"))
o.rmempty = true
o.placeholder = "sdwan.sitepi.cn"
o.datatype = "host"
o.validate = function(self, value, section)
    if value and #value > 0 then
        if not value:match("^[a-zA-Z0-9_.-]+$") then
            return nil, translate("Invalid host format")
        end
        return value
    end
    return ""
end
o.width = "20%"

o = s:option(Value, "interface", translate("Interface"))
o.rmempty = false
o.placeholder = "wg0"
o.validate = function(self, value, section)
    if value and #value > 0 then
        if not value:match("^[a-zA-Z0-9_]+$") then
            return nil, translate("Interface name must only contain alphanumeric characters and underscore")
        end
        -- 检查接口名称唯一性
        local count = 0
        m.uci:foreach("sitepi", "instance", function(s)
            if s[".name"] ~= section and s.interface == value then
                count = count + 1
            end
        end)
        if count > 0 then
            return nil, translate("Interface name must be unique")
        end
        return value
    end
    return nil, translate("Interface name cannot be empty")
end
o.width = "15%"

o = s:option(Value, "network_id", translate("Network ID"))
o.rmempty = true
o.placeholder = translate("Optional")
o.width = "15%"

o = s:option(Value, "description", translate("Description"))
o.rmempty = true
o.placeholder = translate("Optional description")
o.width = "25%"

return m 
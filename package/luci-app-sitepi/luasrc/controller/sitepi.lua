module("luci.controller.sitepi", package.seeall)

function index()
    if not nixio.fs.access("/etc/config/sitepi") then
        return
    end

    local page = entry({"admin", "services", "sitepi"}, alias("admin", "services", "sitepi", "networks"), _("SitePi SDWAN"))
    page.dependent = true
    page.acl_depends = { "luci-app-sitepi" }
    
    entry({"admin", "services", "sitepi", "networks"}, cbi("sitepi"), _("Networks"), 10)
    entry({"admin", "services", "sitepi", "status"}, template("sitepi/status"), _("Status"), 20)
    entry({"admin", "services", "sitepi", "status_data"}, call("action_status_data"))
    entry({"admin", "services", "sitepi", "network_action"}, call("action_network_action"))
end

function action_status_data()
    local util = require "luci.util"
    local sys  = require "luci.sys"
    local json = require "luci.jsonc"
    local uci = require "luci.model.uci".cursor()
    
    local data = { 
        instances = {},
        service_running = sys.call("/etc/init.d/sitepi status >/dev/null 2>&1") == 0
    }
    
    -- 使用 pcall 来安全地执行 UCI 遍历
    local success, err = pcall(function()
        uci:foreach("sitepi", "network", function(s)
            if s.interface then  -- 只检查 interface 字段
                local instance = {
                    name = s[".name"],  -- 使用配置节的名称
                    interface = s.interface,
                    server = s.server or "",
                    provision = s.provision or "", 
                    description = s.description or "",
                    enabled = (s.enabled == "1"),
                    route = (s.route == "1"),
                    running = false,
                    peers = {},
                    status = "stopped"
                }
                
                -- 检查实例是否运行
                instance.running = (sys.call("pgrep -f 'sitepi.*"..s.interface.."' >/dev/null 2>&1") == 0)
                
                -- 只有当接口启用且运行时才检查 WireGuard 状态
                if instance.running then
                    instance.status = "running"
                    -- 安全地执行 wg show 命令
                    local wg_status = util.trim(util.exec("wg show "..util.shellquote(s.interface).." dump 2>/dev/null"))
                    
                    if wg_status and #wg_status > 0 then
                        for line in wg_status:gmatch("[^\r\n]+") do
                            local fields = {}
                            for field in line:gmatch("[^\t]+") do
                                table.insert(fields, field)
                            end
                            
                            if #fields >= 4 then
                                local peer = {
                                    public_key = fields[1],
                                    endpoint = fields[3] ~= "(none)" and fields[3] or "-",
                                    ipaddr = fields[2] ~= "(none)" and fields[2] or "-",
                                    latest_handshake = "Never",
                                    transfer_rx = tonumber(fields[5]) or 0,
                                    transfer_tx = tonumber(fields[6]) or 0
                                }
                                
                                -- 处理握手时间
                                local handshake_time = tonumber(fields[4])
                                if handshake_time and handshake_time > 0 then
                                    peer.latest_handshake = os.date("%Y-%m-%d %H:%M:%S", 
                                        os.time() - handshake_time)
                                end
                                
                                table.insert(instance.peers, peer)
                            end
                        end
                    end
                end
                
                table.insert(data.instances, instance)
            end
        end)
    end)
    
    if not success then
        -- 如果发生错误，返回错误信息
        data.error = tostring(err)
        data.status = "error"
    end
    
    -- 确保响应总是返回有效的 JSON
    luci.http.prepare_content("application/json")
    local response = json.stringify(data)
    if response then
        luci.http.write(response)
    else
        luci.http.write('{"error":"JSON encoding failed","status":"error"}')
    end
end

function action_network_action()
    local uci = require "luci.model.uci".cursor()
    local network = luci.http.formvalue("network")
    local action = luci.http.formvalue("action")
    
    if not network or not action then
        luci.http.status(400, "Bad Request")
        return
    end
    
    local result = { success = false }
    
    if action == "restart" then
        local init = "/etc/init.d/sitepi"
        if nixio.fs.access(init) then
            result.success = os.execute(init.." restart") == 0
        end
    elseif action == "stop" then
        local init = "/etc/init.d/sitepi"
        if nixio.fs.access(init) then
            result.success = os.execute(init.." stop") == 0
        end
    end
    
    luci.http.prepare_content("application/json")
    luci.http.write(json.stringify(result))
end 
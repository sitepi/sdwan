include $(TOPDIR)/rules.mk

PKG_NAME:=luci-app-sitepi
PKG_VERSION:=1.0.0
PKG_RELEASE:=1

PKG_LICENSE:=MIT
PKG_MAINTAINER:=SitePi Team

LUCI_TITLE:=LuCI support for SitePi SDWAN
LUCI_DEPENDS:=+sitepi +wireguard-tools +curl +luci-base +luci-compat
LUCI_PKGARCH:=all

define Package/$(PKG_NAME)/conffiles
/etc/config/sitepi
endef

define Package/$(PKG_NAME)/preinst
#!/bin/sh
if [ -z "$${IPKG_INSTROOT}" ]; then
	rm -rf /tmp/luci-indexcache /tmp/luci-modulecache
fi
exit 0
endef

define Package/$(PKG_NAME)/postinst
#!/bin/sh
if [ -z "$${IPKG_INSTROOT}" ]; then
	rm -rf /tmp/luci-indexcache /tmp/luci-modulecache
	chmod 755 /etc/init.d/sitepi >/dev/null 2>&1
	/etc/init.d/sitepi enable >/dev/null 2>&1
	/etc/init.d/rpcd restart >/dev/null 2>&1
fi
exit 0
endef

define Package/$(PKG_NAME)/postrm
#!/bin/sh
if [ -z "$${IPKG_INSTROOT}" ]; then
	rm -rf /tmp/luci-indexcache /tmp/luci-modulecache
fi
exit 0
endef

define Package/$(PKG_NAME)/install
	$(INSTALL_DIR) $(1)/usr/lib/lua/luci/controller
	$(INSTALL_DIR) $(1)/usr/lib/lua/luci/model/cbi
	$(INSTALL_DIR) $(1)/usr/lib/lua/luci/view/sitepi
	$(INSTALL_DIR) $(1)/usr/share/rpcd/acl.d
	$(INSTALL_DIR) $(1)/etc/uci-defaults
	$(INSTALL_DIR) $(1)/etc/config
	
	$(INSTALL_DATA) ./luasrc/controller/sitepi.lua $(1)/usr/lib/lua/luci/controller/
	$(INSTALL_DATA) ./luasrc/model/cbi/sitepi.lua $(1)/usr/lib/lua/luci/model/cbi/
	$(INSTALL_DATA) ./luasrc/view/sitepi/status.htm $(1)/usr/lib/lua/luci/view/sitepi/
	$(INSTALL_DATA) ./root/usr/share/rpcd/acl.d/luci-app-sitepi.json $(1)/usr/share/rpcd/acl.d/
	$(INSTALL_BIN) ./root/etc/uci-defaults/luci-app-sitepi $(1)/etc/uci-defaults/
	$(INSTALL_CONF) ./root/etc/config/sitepi $(1)/etc/config/
endef

include $(TOPDIR)/feeds/luci/luci.mk

# call BuildPackage - OpenWrt buildroot signature 
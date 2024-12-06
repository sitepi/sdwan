include $(TOPDIR)/rules.mk

LUCI_NAME:=sitepi
LUCI_TITLE:=LuCI support for SitePi SDWAN
LUCI_DEPENDS:=+luci-base +sitepi
LUCI_PKGARCH:=all
LUCI_DESCRIPTION:=LuCI support for SitePi SDWAN client

PKG_NAME:=luci-app-$(LUCI_NAME)
PKG_VERSION:=0.0.3
PKG_RELEASE:=1

PKG_MAINTAINER:=SitePi Team
PKG_LICENSE:=MIT

# 确保在 Services 菜单中显示
LUCI_CATEGORY:=Services

include $(TOPDIR)/feeds/luci/luci.mk

# Install ACL file
define Package/$(PKG_NAME)/postinst
#!/bin/sh
[ -n "$${IPKG_INSTROOT}" ] || {
    rm -f /tmp/luci-indexcache
    rm -f /tmp/luci-modulecache/*
    chmod 755 /etc/init.d/sitepi >/dev/null 2>&1
    /etc/init.d/sitepi enable >/dev/null 2>&1
    exit 0
}
endef

define Package/$(PKG_NAME)/install
    $(call Package/luci-base/install,$(1))
    $(INSTALL_DIR) $(1)/usr/share/rpcd/acl.d
    $(INSTALL_DATA) ./root/usr/share/rpcd/acl.d/luci-app-sitepi.json $(1)/usr/share/rpcd/acl.d/
endef

# call BuildPackage - OpenWrt buildroot signature 
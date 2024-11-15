include $(TOPDIR)/rules.mk

PKG_NAME:=sitepi
PKG_VERSION:=0.1.0
PKG_RELEASE:=1

PKG_LICENSE:=MIT
PKG_LICENSE_FILES:=LICENSE
PKG_MAINTAINER:=SitePi Team

include $(INCLUDE_DIR)/package.mk

define Package/sitepi
  SECTION:=net
  CATEGORY:=Network
  TITLE:=SitePi SDWAN Client
  URL:=https://github.com/sitepi/sdwan
  DEPENDS:=+wireguard-tools +curl
  PKGARCH:=all
endef

define Package/sitepi/description
  A lightweight and efficient Software-Defined Wide Area Network (SD-WAN) client implementation by SitePi.
endef

define Package/sitepi/conffiles
/etc/config/sitepi
endef

define Package/sitepi/preinst
#!/bin/sh
if [ -z "$${IPKG_INSTROOT}" ]; then
	# 停止现有服务
	/etc/init.d/sitepi stop >/dev/null 2>&1
fi
exit 0
endef

define Package/sitepi/postinst
#!/bin/sh
if [ -z "$${IPKG_INSTROOT}" ]; then
	# 设置权限
	chmod 755 /usr/bin/sitepi
	chmod 755 /etc/init.d/sitepi
	
	# 启用服务
	/etc/init.d/sitepi enable >/dev/null 2>&1
	
	# 如果全局启用，则启动服务
	if [ -f /etc/config/sitepi ]; then
		enabled=$$(uci -q get sitepi.global.enabled)
		[ "$$enabled" = "1" ] && /etc/init.d/sitepi start >/dev/null 2>&1
	fi
fi
exit 0
endef

define Package/sitepi/prerm
#!/bin/sh
if [ -z "$${IPKG_INSTROOT}" ]; then
	# 停止服务
	/etc/init.d/sitepi stop >/dev/null 2>&1
	/etc/init.d/sitepi disable >/dev/null 2>&1
fi
exit 0
endef

define Package/sitepi/install
	$(INSTALL_DIR) $(1)/usr/bin
	$(INSTALL_BIN) ./files/sitepi $(1)/usr/bin/sitepi
	
	$(INSTALL_DIR) $(1)/etc/config
	$(INSTALL_CONF) ./files/sitepi.config $(1)/etc/config/sitepi
	
	$(INSTALL_DIR) $(1)/etc/init.d
	$(INSTALL_BIN) ./files/sitepi.init $(1)/etc/init.d/sitepi
	
	$(INSTALL_DIR) $(1)/etc/hotplug.d/iface
	$(INSTALL_DATA) ./files/99-sitepi $(1)/etc/hotplug.d/iface/99-sitepi
endef

$(eval $(call BuildPackage,sitepi)) 
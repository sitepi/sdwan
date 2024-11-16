include $(TOPDIR)/rules.mk

PKG_NAME:=sitepi
PKG_VERSION:=0.1.0
PKG_RELEASE:=1

PKG_MAINTAINER:=SitePi Team
PKG_LICENSE:=MIT

include $(INCLUDE_DIR)/package.mk

define Package/$(PKG_NAME)
  SECTION:=net
  CATEGORY:=Network
  TITLE:=SitePi SDWAN Client
  DEPENDS:=+curl +wireguard-tools +ca-certificates
  PKGARCH:=all
endef

define Package/sitepi/description
  A lightweight and efficient Software-Defined Wide Area Network (SD-WAN) client implementation by SitePi.
endef

define Package/$(PKG_NAME)/install
	$(INSTALL_DIR) $(1)/usr/bin
	$(INSTALL_BIN) $(PKG_BUILD_DIR)/files/sitepi $(1)/usr/bin/
	$(INSTALL_DIR) $(1)/etc/config
	$(INSTALL_CONF) $(PKG_BUILD_DIR)/files/sitepi.config $(1)/etc/config/sitepi
	$(INSTALL_DIR) $(1)/etc/init.d
	$(INSTALL_BIN) $(PKG_BUILD_DIR)/files/sitepi.init $(1)/etc/init.d/sitepi
endef

define Build/Prepare
	mkdir -p $(PKG_BUILD_DIR)/files
	$(CP) ./files/* $(PKG_BUILD_DIR)/files/
endef

define Build/Configure
endef

define Build/Compile
endef

$(eval $(call BuildPackage,$(PKG_NAME))) 
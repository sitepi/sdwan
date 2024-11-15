include $(TOPDIR)/rules.mk

LUCI_NAME:=sitepi
LUCI_TITLE:=LuCI support for SitePi SDWAN
LUCI_DEPENDS:=+luci-base +sitepi
LUCI_PKGARCH:=all
LUCI_DESCRIPTION:=LuCI support for SitePi SDWAN client

PKG_NAME:=luci-app-$(LUCI_NAME)
PKG_VERSION:=0.1.0
PKG_RELEASE:=1

PKG_MAINTAINER:=SitePi Team
PKG_LICENSE:=MIT

# 确保在 Services 菜单中显示
LUCI_CATEGORY:=Services

include $(TOPDIR)/feeds/luci/luci.mk

# call BuildPackage - OpenWrt buildroot signature 
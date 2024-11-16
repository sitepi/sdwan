
#### Build from source
1. Add feed to OpenWrt
```bash
echo "src-git sitepi https://github.com/sitepi/sdwan.git" >> feeds.conf.default
./scripts/feeds update -a
./scripts/feeds install -a
```

2. Configure and build
```bash
make menuconfig
# Go to Network -> sitepi
# Go to LuCI -> Applications -> luci-app-sitepi
make package/sitepi/compile V=s
make package/luci-app-sitepi/compile V=s
```

The compiled packages will be in `bin/packages/ARCH/base/`.

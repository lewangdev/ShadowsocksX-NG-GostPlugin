# gost-ss-local

ShadowsocksX-NG 的 gost 插件脚本，方便在 ShadowsocksX-NG 中使用 gost 

## 为啥编写这个脚本

自从查资料代理工具换成 [gost](https://github.com/ginuerzh/gost) 之后，由于 MacOS 上没有 gost 专用的智能代理（也就是该翻的时候翻，不用翻的时候不翻）桌面客户端，所以需要用 gost 在本地把 https 代理转成 ss 后再继续使用 ShadowsocksX-NG。 虽然可以用 launchctl 启动一个 gost 后台服务，但是用起来还是不太方便。 于是看了下 ShadowsocksX-NG 是如何工作的，发现 ShadowsocksX-NG 会在本地启动 ss-local 进程跑一个 sock5 服务，而且 ShadowsocksX-NG 实现智能代理的逻辑与 ss-local 并没有太多的关系，只要在本地能提供一个 sock5 服务就够了。 

于是写了以下脚本替换了 ShadowsocksX-NG 安装目录下的 ss-local(替换之前要备份一下这个文件)

```bash
#!/bin/sh -

#/opt/gost/gost -L=sock5://:1086 -F=https://USER:PASSWD@1.1.1.1:1234

# Use gost.json
#{
#    "Retries": 3,
#    "Debug": false,
#    "ServeNodes": [
#        "socks5://127.0.0.1:1086"
#    ],
#    "ChainNodes": [
#        "https://USER:PASSWD@1.1.1.1:1234"
#    ]
#}
/opt/gost/gost -C /opt/gost/gost.json
```

替换之后，ShadowsocksX-NG 可以正常工作，但是这样 ShadowsocksX-NG 的其它设置就无法工作了，所以用 python 把功能加强了一下，编写了这个脚本。


## 安装

1. 下载 [gost](https://github.com/ginuerzh/gost/releases/download/v2.11.1/gost-darwin-amd64-2.11.1.gz) 并解压到目录 `"${HOME}/Library/Application Support/ShadowsocksX-NG/gost"`， 确保 `"${HOME}/Library/Application Support/ShadowsocksX-NG/gost"` 目录下可执行文件名称为 `gost`
2. 备份 `/Applications/ShadowsocksX-NG.app/Contents/Resources/ss-local` 为 `/Applications/ShadowsocksX-NG.app/Contents/Resources/ss-local.bak` 
3. 用 `https://raw.githubusercontent.com/thisiswangle/gost-ss-local/master/ss-local` 替换 `/Applications/ShadowsocksX-NG.app/Contents/Resources/ss-local` 
4. 关闭 ShadowsocksX-NG 应用，再打开即可正常使用

## 设置

ShadowsocksX-NG 客户端的配置并不能与 gost 的配置对应上，gost-ss-local 使用了 Address， Port， Password， Plugin，Plugin Opts 来设置 gost。在 Plugin 使用 gost 之后，各参数设置说明如下: 

0. Plugin，如何希望使用 gost，那么 Plugin 需要填写 gost，例如 `gost`，不填或填些其它内容，则与 ShadowsocksX-NG 原行为一致
1. Address, 表示 gost 的服务器地址，可以是 IP 或域名, 例如填写 `2.3.4.5`
2. Port, 表示 gost 的端口, 例如填写 `8443`
3. Password, 由于 ShadowsocksX-NG 不能设置用户名，密码这里需要填写 gost 的用户名和密码，格式为 `USER:PASSWD`, 例如填写 `user@letmein88`
4. Plugin Opts, 如果填写了插件参数，则前 1-3 的设置无效，并且会把 Plugin Opts 填写的内容直接全部传给 gost 命令。

1-3 的设置要求 gost 服务器端为 https 代理，如果为其它类型的代理，可以通过设置 Plugin Opts 的参数来设置


## 说明

目前脚本只测试了 gost，对于 [SIP003 Plugin](https://github.com/shadowsocks/ShadowsocksX-NG/wiki/SIP003-Plugin) 是否影响没有做过测试，如果有问题，欢迎 PR。

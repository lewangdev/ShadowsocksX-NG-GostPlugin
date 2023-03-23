# ShadowsocksX-NG Gost Plugin

ShadowsocksX-NG 的 gost 插件脚本，方便在 ShadowsocksX-NG 中使用 gost

## 原由

自从查资料上网工具换成 [gost](https://github.com/ginuerzh/gost) 之后，由于 MacOS 上没有 gost 专用的智能代理（也就是该翻的时候翻，不用翻的时候不翻）桌面客户端，所以需要用 gost 在本地把 wss 代理转成 ss 后再继续使用 ShadowsocksX-NG。 虽然可以用 launchctl 启动一个 gost 后台服务，但是用起来还是不太方便。

ShadowsocksX-NG 是这样工作的： ShadowsocksX-NG 会在本地启动 ss-local 进程跑一个 socks5 服务，而且 ShadowsocksX-NG 实现智能代理的逻辑与 ss-local 并没有太多的关系，所以只要在想办法本地能提供一个 socks5 服务就够了。

最近看了一下 SIP003 插件的规范, 而 [gost](https://github.com/ginuerzh/gost) 的代理链功能就这个规范差不多, ShadowsocksX-NG 客户端也是支持 SIP003 规范插件的, 于是写了几行胶水代码, 让 ShadowsocksX-NG 客户端也支持 gost 。

## 安装插件前的准备

1. 安装好 [ShadowsocksX-NG](https://github.com/shadowsocks/ShadowsocksX-NG/releases/download/v1.10.1/ShadowsocksX-NG.dmg) 并至少启动过一次

## 手动安装插件

手动安装过程包括以下几个步骤，主要是文件的替换：

1. 下载 [gost](https://github.com/ginuerzh/gost/releases/download/v2.11.1/gost-darwin-amd64-2.11.1.gz) 并解压到目录 `"${HOME}/Library/Application Support/ShadowsocksX-NG/gost"`， 确保 `"${HOME}/Library/Application Support/ShadowsocksX-NG/gost"` 目录下可执行文件名称为 `gost`
2. 下载 `https://github.com/lewangdev/ShadowsocksX-NG-GostPlugin/releases/download/v0.0.2/gost-plugin.gz` 到 `${HOME}/Library/Application Support/ShadowsocksX-NG/plugins`, 并且解压为 `gost-plugin`
3. 退出 ShadowsocksX-NG 应用，再打开即可正常使用

## 通过自动安装脚本安装插件

把上面的手动安装的过程变成自动安装的脚本

```bash
curl -L https://github.com/lewangdev/ShadowsocksX-NG-GostPlugin/raw/main/gost-plugin-installer | bash
```

## 设置

> 如果使用 cloudflare 进行代理, 建议添加 2 个服务器, 一个是直连服务器, 另外一个通过 cloudflare 代理

1. Address, 表示 gost 的服务器地址，可以是 IP 或域名, 例如填写 `1.2.3.4`
2. Port, 表示 gost 的服务器端口, 例如填写 `443`
3. Encryption, 固定为 `chacha20-ietf-poly1305`
4. Password, 固定为 `123456`
5. Plugin Opts, 为 gost 的用户名和密码, 格式为 `username:password`

<div align="center">
  <img width="90%" src="https://user-images.githubusercontent.com/1455685/227181149-46f1ffbd-e3be-4b8d-a25b-de500f99ca85.png">
</div>

## 远程在服务器上安装 gost

[请使用 gost-install.ipynb 远程安装](https://github.com/lewangdev/gost-install.ipynb)

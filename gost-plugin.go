package main

import (
	"fmt"
	"os"
	"os/exec"
)

func main() {
	ssLocalHost := os.Getenv("SS_LOCAL_HOST")
	ssLocalPort := os.Getenv("SS_LOCAL_PORT")
	ssRemoteHost := os.Getenv("SS_REMOTE_HOST")
	ssRemotePort := os.Getenv("SS_REMOTE_PORT")
	ssPluginOptions := os.Getenv("SS_PLUGIN_OPTIONS")

	cmdArgsL := fmt.Sprintf("ss://chacha20-ietf-poly1305:123456@%s:%s", ssLocalHost, ssLocalPort)
	cmdArgsF := fmt.Sprintf("wss://%s@%s:%s", ssPluginOptions, ssRemoteHost, ssRemotePort)
	cmd := exec.Command("./gost/gost", "-L", cmdArgsL, "-F", cmdArgsF)
	err := cmd.Run()
	if err != nil {
		fmt.Println("运行命令失败：", err)
		return
	}
}

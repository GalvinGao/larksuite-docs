---
document_id: '7633446179720727991'
directory_id: '7634017946344050100'
title: 一键创建应用（Go）
full_path: /mcp_open_tools/integrating-agents-with-feishu/create-an-app-in-one-click-go
breadcrumb:
- Lark CLI
- One-click create agent app
- Create an app in one click (Go)
document_type: GuideDocumentType
updated_at: 2026-06-05T03:29:57Z
source_url: https://open.larksuite.com/document/mcp_open_tools/integrating-agents-with-feishu/create-an-app-in-one-click-go
---

# 一键创建应用（Go）

Go SDK 提供了 `RegisterApp` 方法，基于 OAuth 2.0 Device Authorization Grant（RFC 8628）协议实现一键创建应用。

调用该方法会返回一个验证链接，用户在Lark或 Lark 中打开该链接（或扫码）完成授权后，即可自动注册应用并获取凭据（App ID 和 App Secret），无需手动前往开发者后台创建。

查看源码：[oapi-sdk-go](https://github.com/larksuite/oapi-sdk-go#%E4%B8%80%E9%94%AE%E5%88%9B%E5%BB%BA%E5%BA%94%E7%94%A8--one-click-app-registration)

## 前提条件

- 安装 `oapi-sdk-go`，且 SDK 版本为最新 v3 版本。

  ```shell
  go get github.com/larksuite/oapi-sdk-go/v3@latest
  ```

## 快速开始示例

```go
package main

import (
	"context"
	"errors"
	"fmt"
	"time"

	lark "github.com/larksuite/oapi-sdk-go/v3"
	"github.com/larksuite/oapi-sdk-go/v3/scene/registration"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Minute)
	defer cancel()

	result, err := registration.RegisterApp(ctx, &registration.Options{
		OnQRCode: func(info *registration.QRCodeInfo) {
			fmt.Printf("open or scan this url: %s\n", info.URL)
			fmt.Printf("the link expires in %d seconds\n", info.ExpireIn)
		},
		OnStatusChange: func(info *registration.StatusChangeInfo) {
			// status: polling | slow_down | domain_switched
			fmt.Printf("registration status: %s", info.Status)
			if info.Interval > 0 {
				fmt.Printf(", next poll after %d seconds", info.Interval)
			}
			fmt.Println()
		},
	})
	if err != nil {
		var regErr *registration.RegisterAppError
		if errors.As(err, &regErr) {
			fmt.Printf("register app failed: code=%s, description=%s\n", regErr.Code, regErr.Description)
			return
		}
		panic(err)
	}

	fmt.Println("App ID:", result.ClientID)
	fmt.Println("App Secret:", result.ClientSecret)

	client := lark.NewClient(result.ClientID, result.ClientSecret)
	_ = client
}
```

## RegisterApp 参数

`RegisterApp` 的函数签名为：

```go
func RegisterApp(ctx context.Context, opts *Options) (*RegisterAppResult, error)
```

`Options` 结构体字段如下：

| 参数 | 描述 | 类型 | 必填 | 默认 |
| --- | --- | --- | --- | --- |
| Domain | 自定义Lark认证域名（需包含协议前缀） | string | 否 | <code>https://accounts.larksuite.com</code> |
| LarkDomain | 自定义 Lark 认证域名（需包含协议前缀），检测到 Lark 租户时自动切换 | string | 否 | <code>https://accounts.larksuite.com</code> |
| Source | 来源标识，拼入二维码 URL 的 <code>source</code> 参数，格式为 <code>go-sdk/{Source}</code> | string | 否 | - |
| OnQRCode | 验证链接就绪时的回调，参数为 <code>*QRCodeInfo</code>。可将 URL 渲染为二维码供用户扫码，或直接作为链接展示 | func(*QRCodeInfo) | 是 | - |
| OnStatusChange | 轮询状态变化时的回调，参数为 <code>*StatusChangeInfo</code>。status 取值：<code>polling</code>、<code>slow_down</code>、<code>domain_switched</code> | func(*StatusChangeInfo) | 否 | - |
| AppPreset | 预设应用信息，仅用于初始化创建页；用户仍可在页面修改，最终以页面提交为准。 | *registration.AppPreset | 否 | - |
| AppPreset.Avatar | 应用头像 URL，支持 1-6 个；第一个默认选中。传原始 URL，SDK 会编码。头像展示、图片可访问性、GIF 取帧等由创建页处理。 | []string | 否 | - |
| AppPreset.Name | 应用名称，支持 <code>{user}</code> 占位符；传原始值，SDK 会编码。 | string | 否 | - |
| AppPreset.Desc | 应用描述，支持 <code>{user}</code> 占位符；传原始值，SDK 会编码。 | string | 否 | - |


此外，第一个参数 `ctx context.Context` 可用于控制超时和取消操作。例如通过 `context.WithTimeout` 设置超时时间，或通过 `context.WithCancel` 取消轮询。

## 回调参数类型

**QRCodeInfo**：

| 字段 | 类型 | 描述 |
| --- | --- | --- |
| URL | string | 二维码 URL，用户扫码或在浏览器中打开此链接完成授权 |
| ExpireIn | int | 链接过期时间（秒），默认 600 |

**StatusChangeInfo**：

| 字段 | 类型 | 描述 |
| --- | --- | --- |
| Status | string | 状态值，取值见下方说明 |
| Interval | int | 当 Status 为 `slow_down` 时，返回新的轮询间隔（秒） |

**状态常量**：

| 常量 | 值 | 描述 |
| --- | --- | --- |
| StatusPolling | `polling` | 授权等待中，继续轮询 |
| StatusSlowDown | `slow_down` | 服务端要求降低轮询频率，间隔增加 5 秒 |
| StatusDomainSwitched | `domain_switched` | 检测到 Lark 租户，域名已自动切换 |

## 返回值

`RegisterAppResult` 结构体字段如下：

| 字段 | 类型 | 描述 |
| --- | --- | --- |
| ClientID | string | 应用的 App ID |
| ClientSecret | string | 应用的 App Secret |
| UserInfo | *UserInfo（可选） | 扫码授权的用户信息 |
| UserInfo.OpenID | string（可选） | 扫码用户的 open_id |
| UserInfo.TenantBrand | string（可选） | 租户品牌，取值为 `"Lark"` 或 `"lark"` |

## 错误处理

`RegisterApp` 返回的 error 可以通过 `errors.As` 进行类型断言，以区分不同的错误场景：

| 错误类型 | code | 描述 |
| --- | --- | --- |
| `*AccessDeniedError` | `access_denied` | 用户拒绝了授权 |
| `*ExpiredError` | `expired_token` | 二维码过期或轮询超时 |
| `*RegisterAppError` | `invalid_response` | 服务端响应异常 |
| `*RegisterAppError` | 其他 | 服务端返回的其他错误 |

所有错误类型均包含 `Code` 和 `Description` 字段，其中基础错误类型 `RegisterAppError` 实现了 `error` 接口。

**错误处理示例**：

```go
result, err := registration.RegisterApp(ctx, opts)
if err != nil {
    var accessDenied *registration.AccessDeniedError
    var expired *registration.ExpiredError
    switch {
    case errors.As(err, &accessDenied):
        fmt.Printf("用户拒绝授权: %s\n", accessDenied.Description)
    case errors.As(err, &expired):
        fmt.Printf("二维码已过期: %s\n", expired.Description)
    default:
        fmt.Printf("注册失败: %v\n", err)
    }
}
```

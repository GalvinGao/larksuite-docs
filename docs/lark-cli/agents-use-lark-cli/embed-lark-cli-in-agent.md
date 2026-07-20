---
document_id: '7633446179720760759'
directory_id: '7634017946344066484'
title: 自研 Agent 接入 Lark CLI
full_path: /mcp_open_tools/feishu-cli/embed-feishu-cli-in-agent
breadcrumb:
- Lark CLI
- Agents use Lark CLI
- Embed Lark CLI in agent
document_type: GuideDocumentType
updated_at: 2026-05-20T03:30:49Z
source_url: https://open.larksuite.com/document/mcp_open_tools/feishu-cli/embed-feishu-cli-in-agent
---

# 自研 Agent 接入Lark CLI

## 适用场景

Lark CLI 内置了一套**扩展机制**，让你在不修改 CLI 源码的情况下，替换或增强 CLI 的核心行为。

如果你是企业内部开发者或 ISV，希望**在自己的 Agent 或应用中直接内嵌Lark** **CLI** **能力**，并需要：
- 从自己的凭证系统（数据库、Vault、配置中心）获取 AppID / Token 等
- 对所有 HTTP 请求统一加监控、日志，或改写请求目标等

你只需要：写一个 Go 包，实现指定接口，在 wrapper main 里 import 它，重新编译——就得到一个**行为完全不同的增强版** **CLI**。
  
目前支持两个扩展点：

- **Credential — 凭证来源**： CLI 需要 AppID，以及真实 AppSecret 或可用的 UAT/TAT 来调用Lark API。默认从环境变量或 keychain 读取；只有在换取 tenant_access_token 时才必须依赖 AppSecret。通过 Credential 扩展，你可以让它从数据库、Vault、配置中心等任意来源获取。

- **Transport — HTTP 请求拦截**： CLI 的每个 HTTP 请求都会经过 Transport 层。通过 Transport 扩展，你可以在请求发出前注入 Header、改写请求目标（Host / URL）、记录日志，在请求完成后统计耗时、上报监控。
  

## 扩展1 ：Credential -凭证来源

### 它是什么

Credential 扩展让你**自定义凭证的来源**。CLI 默认会从环境变量（`LARKSUITE_CLI_APP_ID`、`LARKSUITE_CLI_APP_SECRET`、`LARKSUITE_CLI_USER_ACCESS_TOKEN`、`LARKSUITE_CLI_TENANT_ACCESS_TOKEN`）或本地 keychain 读取凭证。通过注册自定义 Provider，你可以让 CLI 从数据库、Vault、配置中心等任意来源获取凭证。本文为了直观，采取从本地读取凭证来进行演示。

### 怎么用

#### 第 1 步：创建项目

```
mkdir my-cli && cd my-cli
go mod init my-cli
```

#### 第 2 步：编写 Provider

创建 `mycred/mycred.go`，实现 `credential.Provider` 接口：

```go
package mycred
import (
    "bytes"
    "context"
    "encoding/json"
    "fmt"
    "net/http"
    "github.com/larksuite/cli/extension/credential"
)
const (
    appID     = "cli_aXXXXXXXXXXXXXXXX" // 替换为真实 AppID
    appSecret = "XXXXXXXXXXXXXXXXXXXXXXXX" // 替换为真实 AppSecret
)
type Provider struct{}
func (p *Provider) Name() string { return "mycred" }
// ResolveAccount 返回应用凭证
// 返回 &Account{} → 命中，CLI 用这组凭证
// 返回 nil, nil   → 跳过，交给下一个 Provider
// 返回 nil, err   → 报错，终止
func (p *Provider) ResolveAccount(ctx context.Context) (*credential.Account, error) {
    // 这里替换成你自己的逻辑：查数据库、调 Vault、读配置中心...
    return &credential.Account{
        AppID:     appID,
        AppSecret: appSecret,
        Brand:     credential.BrandLark,
        DefaultAs: credential.IdentityUser,
    }, nil
}
// ResolveToken 返回访问令牌
// 重要：一旦你的 Provider 接管了 Account，Token 也由你负责。
// 返回 nil, nil 不会 fallback 到默认换 Token 逻辑，而是直接报错。
func (p *Provider) ResolveToken(ctx context.Context, req credential.TokenSpec) (*credential.Token, error) {
    switch req.Type {
    case credential.TokenTypeTAT:
        // bot 身份：用 AppID + AppSecret 向Lark换取 tenant_access_token
        token, err := exchangeTenantAccessToken(appID, appSecret)
        if err != nil {
            return nil, err
        }
        return &credential.Token{Value: token, Source: "mycred:tat"}, nil
    case credential.TokenTypeUAT:
        // user 身份：从任意来源获取 user_access_token
        // 例如：从数据库查询、从 Redis 缓存读取、从 OAuth 回调获取、
        // 从配置文件读取，甚至直接写死一个用于测试...
        uat := getUserAccessToken()
        return &credential.Token{Value: uat, Source: "mycred:uat"}, nil
    }
    return nil, nil
}
// getUserAccessToken 从你的来源获取 UAT
// 这里你可以对接任何存储：数据库、Redis、文件、环境变量等
func getUserAccessToken() string {
    // 示例：从环境变量获取
    // return os.Getenv("MY_USER_ACCESS_TOKEN")
    // 示例：从数据库查询
    // return db.Query("SELECT uat FROM tokens WHERE user_id = ?", userID)
    // 示例：写死一个用于本地测试（从 lark-cli auth login 后获取）
    return "u-XXXXXXXX"
}
// exchangeTenantAccessToken 用 AppID + AppSecret 向Lark换取 tenant_access_token
func exchangeTenantAccessToken(appID, appSecret string) (string, error) {
    body, _ := json.Marshal(map[string]string{
        "app_id":     appID,
        "app_secret": appSecret,
    })
    resp, err := http.Post(
        //此处以feishu Brand作为演示，如果为LARK品牌host切换为：open.larksuite.com
        "https://open.larksuite.com/open-apis/auth/v3/tenant_access_token/internal",
        "application/json", bytes.NewReader(body),
    )
    if err != nil {
        return "", fmt.Errorf("exchange tat: %w", err)
    }
    defer resp.Body.Close()
    var result struct {
        Code              int    `json:"code"`
        TenantAccessToken string `json:"tenant_access_token"`
    }
    if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
        return "", err
    }
    if result.Code != 0 {
        return "", fmt.Errorf("exchange tat failed, code=%d", result.Code)
    }
    return result.TenantAccessToken, nil
}
func init() {
    credential.Register(&Provider{})
}
```

#### 第 3 步：编写 wrapper main

创建 `main.go`：

```go
package main
import (
    "os"
    "github.com/larksuite/cli/cmd"
    _ "my-cli/mycred"  // blank import 触发 init() → 注册你的 Provider
)
func main() {
    os.Exit(cmd.Execute())
}
```

#### 第 4 步：编译验证

```
go mod tidy
go build -o my-lark-cli .

# 用 --dry-run 验证凭证来源（不会真正发请求）

./my-lark-cli api GET /open-apis/calendar/v4/calendars --dry-run
```
输出：
```
{
  "appId": "cli_a1234567890",
  "as": "user"
}
```
`appId` 显示的是你设置的值，说明自定义 Provider 生效了。

### 原理

Credential 采用**链式数组**存储多个 Provider，按注册顺序依次查询，第一个返回非 nil 的胜出：
```
import 顺序:  mycred → env
注册结果:     providers = [mycred, env]
CLI 需要凭证时:
  ┌─ mycred.ResolveAccount()
  │   查到了? → 返回 Account → 命中! 后面全跳过
  │   没查到? → 返回 nil    → 继续
  │
  ├─ env.ResolveAccount()
  │   环境变量有? → 返回 Account → 命中
  │   没有?      → 返回 nil    → 继续
  │
  └─ defaultAcct (keychain 兜底)
```

:::note
**延迟执行**：Provider 在启动时只是注册，不会立即被调用。只有当用户执行的命令真正需要凭证时，才会触发链式遍历。结果通过 `sync.Once` 缓存，整个进程只解析一次。
:::

#### 核心源码

| 文件                                 | 作用                                      |
| ---------------------------------- | --------------------------------------- |
| `extension/credential/types.go`    | Provider 接口、Account / Token 类型定义        |
| `extension/credential/registry.go` | `Register()` 追加到全局数组，`Providers()` 返回快照 |
| `extension/credential/env/env.go`  | 内置实现：从环境变量读取凭证                          |


## 扩展 2：Transport — HTTP 请求拦截

### 它是什么

Transport 扩展让你**拦截** **CLI** **发出的每一个 HTTP 请求**。你可以在请求发出前修改 Header、改写请求目标（Host / URL）、打印日志，在请求完成后记录耗时、上报监控——而 CLI 的业务逻辑完全不需要改动。

### 怎么用

#### 第 1 步：编写 Interceptor

创建 `tracing/tracing.go`，实现 `transport.Provider` 和 `transport.Interceptor` 接口：

```go
package tracing
import (
    "context"
    "fmt"
    "net/http"
    "os"
    "time"
    "github.com/larksuite/cli/extension/transport"
)
type Provider struct{}
func (p *Provider) Name() string { return "tracing" }
func (p *Provider) ResolveInterceptor(ctx context.Context) transport.Interceptor {
    return &tracingInterceptor{}
}
type tracingInterceptor struct{}
// PreRoundTrip 在每个请求发出前被调用
// 返回的函数在请求完成后被调用
func (i *tracingInterceptor) PreRoundTrip(req *http.Request) func(*http.Response, error) {
    // ===== 请求前 =====
    start := time.Now()
    fmt.Fprintf(os.Stderr, "🚀 请求开始: %s %s\n", req.Method, req.URL)
    // ===== 请求后（返回的闭包）=====
    return func(resp *http.Response, err error) {
        elapsed := time.Since(start)
        if err != nil {
            fmt.Fprintf(os.Stderr, "❌ 请求失败: %s %s err=%v (耗时 %v)\n",
                req.Method, req.URL, err, elapsed)
        } else {
            fmt.Fprintf(os.Stderr, "✅ 请求完成: %s %s 状态码=%d (耗时 %v)\n",
                req.Method, req.URL, resp.StatusCode, elapsed)
        }
    }
}
func init() {
    transport.Register(&Provider{})
}
```

#### 第 2 步：在 main.go 中引入

```go
package main
import (
    "os"
    "github.com/larksuite/cli/cmd"
    _ "my-cli/tracing"  // blank import 注册 Transport Interceptor
    _ "github.com/larksuite/cli/extension/credential/env"
)
func main() {
    os.Exit(cmd.Execute())
}
```

#### 第 3 步：编译验证

```go
go build -o my-lark-cli .
```

- 验证 UAT（user 身份）：

```
./my-lark-cli api GET /open-apis/calendar/v4/calendars --as user
```
输出：
```
🚀 请求开始: GET https://open.larksuite.com/open-apis/authen/v1/user_info
✅ 请求完成: GET https://open.larksuite.com/open-apis/authen/v1/user_info 状态码=200 (耗时 644ms)
🚀 请求开始: GET https://open.larksuite.com/open-apis/calendar/v4/calendars
✅ 请求完成: GET https://open.larksuite.com/open-apis/calendar/v4/calendars 状态码=200 (耗时 1070ms)
{ "code": 0, "data": { "calendar_list": [...] } }
```
- 验证 TAT（bot 身份）：

```
./my-lark-cli api GET /open-apis/calendar/v4/calendars --as bot
```

输出：

```
🚀 请求开始: GET https://open.larksuite.com/open-apis/calendar/v4/calendars
✅ 请求完成: GET https://open.larksuite.com/open-apis/calendar/v4/calendars 状态码=200 (耗时 736ms)
{ "code": 0, "data": { ... } }
```

### 原理

Transport 采用**单 Provider** 模式，注册后会被包装为一个 HTTP 中间件，插入到 CLI 的请求链中：
```
CLI 发 HTTP 请求
      │
      ▼
你的 PreRoundTrip(req)         ← 请求前: 加 Header / 改写 Host / 打日志 / 开始计时
      │  返回一个 postFn
      ▼
CLI 内置 transport 链           ← Retry → SecurityHeader → 实际网络请求
      │
      ▼
postFn(resp, err)              ← 请求后: 记录耗时 / 上报监控
```

CLI 在构建 HTTP Client 时，会调用 `wrapWithExtension()` 把你的 Interceptor 包装成中间件。之后 CLI 发出的**每一个请求**（包括换 Token、调业务 API）都会经过这个中间件。

:::note
**安全保障**：CLI 内置安全层会统一重写基础安全 Header（如 `X-Cli-*`、`User-Agent`）。`Authorization` 不属于这层统一覆盖：它由请求构建阶段设置；如果发生跨 host redirect，会在 redirect 策略中被剥离。因此你可以添加自定义 Header，但 ==**不要依赖**== Transport 扩展去覆盖凭证头。
:::

#### 核心源码

| 文件                                | 作用                                          |
| --------------------------------- | ------------------------------------------- |
| `extension/transport/types.go`    | Provider / Interceptor 接口定义                 |
| `extension/transport/registry.go` | `Register()` 注册单个 Provider（last-write-wins） |


## 完整示例

将两个扩展组合在一起的 wrapper main：

```go
package main
import (
    "os"
    "github.com/larksuite/cli/cmd"
    // 自定义扩展（写在前面 = 凭证优先级更高）
    _ "my-cli/mycred"    // 自定义凭证来源
    _ "my-cli/tracing"   // HTTP 请求拦截
    // 原版 CLI 兜底
    _ "github.com/larksuite/cli/extension/credential/env"
)
func main() {
    os.Exit(cmd.Execute())
}
```
:::note
**对比原版** **CLI** **的 main.go**，只多了 2 行 import。业务代码 `cmd.Execute()` 完全不变，编译出来就是一个功能完整的增强版 CLI。
:::

### 项目结构

```
my-cli/
├── go.mod             # require github.com/larksuite/cli v1.0.6

├── main.go            # Wrapper main

├── mycred/
│   └── mycred.go      # Credential: 自定义凭证来源

└── tracing/
    └── tracing.go     # Transport: HTTP 请求拦截

```

### 构建和运行

```
mkdir my-cli && cd my-cli
go mod init my-cli

# 编写 main.go、mycred/mycred.go、tracing/tracing.go（见上方代码）

go get github.com/larksuite/cli@v1.0.6
go mod tidy
go build -o my-lark-cli .

# 验证凭证扩展（--dry-run 不发真实请求，只看用了哪组凭证）

./my-lark-cli api GET /open-apis/calendar/v4/calendars --dry-run
```

输出：

```
{
  "appId": "cli_aXXXXXXXXXXXXXXXX",
  "as": "user"
}
```
```

# 验证请求拦截（发送真实请求，查看日志输出）

./my-lark-cli api GET /open-apis/calendar/v4/calendars --as user
```
输出：
```
🚀 请求开始: GET https://open.larksuite.com/open-apis/authen/v1/user_info
✅ 请求完成: GET .../user_info 状态码=200 (耗时 644ms)
🚀 请求开始: GET https://open.larksuite.com/open-apis/calendar/v4/calendars
✅ 请求完成: GET .../calendars 状态码=200 (耗时 1070ms)
{ "code": 0, "data": { "calendar_list": [...] } }
```
两个扩展同时生效：Credential 提供了凭证和 Token，Transport 拦截并记录了每个请求。

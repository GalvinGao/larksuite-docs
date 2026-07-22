---
document_id: '7633446179720777143'
directory_id: '7634017946344050100'
title: 一键创建应用（NodeJS）
full_path: /mcp_open_tools/integrating-agents-with-feishu/scan-to-create-an-app-in-one-click-nodejs
breadcrumb:
- Lark CLI
- One-click create agent app
- Create an app in one click (NodeJS)
document_type: GuideDocumentType
updated_at: 2026-06-05T03:29:57Z
source_url: https://open.larksuite.com/document/mcp_open_tools/integrating-agents-with-feishu/scan-to-create-an-app-in-one-click-nodejs
---

# 一键创建应用（NodeJS）

Node SDK 提供了 `registerApp` 方法，基于 OAuth 2.0 Device Authorization Grant（RFC 8628）协议实现一键创建应用。

调用该方法会返回一个验证链接，用户在Lark或 Lark 中打开该链接（或扫码）完成授权后，即可自动注册应用并获取凭据（App ID 和 App Secret），无需手动前往开发者后台创建。

查看源码：[node-sdk](https://github.com/larksuite/node-sdk/blob/main/README.zh.md#%E4%B8%80%E9%94%AE%E5%88%9B%E5%BB%BA%E5%BA%94%E7%94%A8)

## 前提条件

- 安装 `@larksuiteoapi/node-sdk`，且 SDK 版本为 1.61.1 及以上。

  ```shell
  npm install @larksuiteoapi/node-sdk
  ```
  
## 快速开始示例  

```javascript 
import * as lark from '@larksuiteoapi/node-sdk';

try {
    const result = await lark.registerApp({
        onQRCodeReady(info) {
            console.log(`请扫码: ${info.url}`);
            console.log(`链接将在 ${info.expireIn} 秒后过期`);
        },
        onStatusChange(info) {
            // 处理状态变化：'polling' | 'slow_down' | 'domain_switched'
        },
    });

    console.log('App ID:', result.client_id);
    console.log('App Secret:', result.client_secret);

    // 用获取到的凭据初始化 Client
    const client = new lark.Client({
        appId: result.client_id,
        appSecret: result.client_secret,
    });
} catch (e) {
    // e.code: 'access_denied' | 'expired_token' | 'abort' | ...
    // e.description: 错误描述
    console.error(e.code, e.description);
}

``` 

## registerApp 参数

| 参数 | 描述 | 类型 | 必填 | 默认 |
| --- | --- | --- | --- | --- |
| domain | 自定义认证域名（仅 host 部分） | string | 否 | <code>accounts.feishu.cn</code> |
| larkDomain | 自定义 Lark 认证域名（仅 host 部分），检测到 Lark 租户时自动切换 | string | 否 | <code>accounts.larksuite.com</code> |
| source | 来源标识，拼入二维码 URL 的 <code>from</code> 参数，格式为 <code>node-sdk/{source}</code> | string | 否 | - |
| signal | 用于取消轮询的 <code>AbortSignal</code> | AbortSignal | 否 | - |
| onQRCodeReady | 验证链接就绪时的回调，参数为 <code>{ url, expireIn }</code>。可将 URL 渲染为二维码供用户扫码，或直接作为链接展示 | function | 是 | - |
| onStatusChange | 轮询状态变化时的回调，参数为 <code>{ status, interval? }</code>。status 取值：<code>polling</code>、<code>slow_down</code>、<code>domain_switched</code> | function | 否 | - |
| appPreset | 预设应用信息（头像、名称、描述）。所有字段都是可选的，用户扫码后仍可在页面手动修改 | AppPreset | 否 | - |
| appPreset.avatar | 应用头像 URL，支持 1-6 个；传多个时默认选中第一个。支持 png / jpg / jpeg / webp / gif（gif 自动取一帧，不保留动图） | string &#124; string[] | 否 | - |
| appPreset.name | 应用名称，支持 <code>{user}</code> 占位符（替换为扫码用户名称） | string | 否 | - |
| appPreset.desc | 应用描述，支持 <code>{user}</code> 占位符 | string | 否 | - |


## 返回值

| 字段 | 类型 | 描述 |
| --- | --- | --- |
| client_id | string | 应用的 App ID |
| client_secret | string | 应用的 App Secret |
| user_info | object（可选） | 扫码授权的用户信息 |
| user_info.open_id | string（可选） | 扫码用户的 open_id |
| user_info.tenant_brand | string（可选） | 租户品牌，取值为 `"Lark"` 或 `"lark"` |

## 错误处理

抛出的错误对象包含 `code` 和 `description` 字段：

| code | 描述 |
| --- | --- |
| `access_denied` | 用户拒绝了授权 |
| `expired_token` | 二维码过期或轮询超时 |
| `abort` | 通过 AbortSignal 取消操作|

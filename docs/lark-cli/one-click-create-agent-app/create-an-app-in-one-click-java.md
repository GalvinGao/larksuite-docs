---
document_id: '7633446179720711607'
directory_id: '7634017946344050100'
title: 一键创建应用（Java）
full_path: /mcp_open_tools/integrating-agents-with-feishu/create-an-app-in-one-click-java
breadcrumb:
- Lark CLI
- One-click create agent app
- Create an app in one click (Java)
document_type: GuideDocumentType
updated_at: 2026-06-05T03:29:57Z
source_url: https://open.larksuite.com/document/mcp_open_tools/integrating-agents-with-feishu/create-an-app-in-one-click-java
---

# 一键创建应用（Java）

Java SDK 提供了 `RegisterApp.register` 方法，基于 OAuth 2.0 Device Authorization Grant（RFC 8628）协议实现一键创建应用。

调用该方法会返回一个验证链接，用户在Lark或 Lark 中打开该链接（或扫码）完成授权后，即可自动注册应用并获取凭据（App ID 和 App Secret），无需手动前往开发者后台创建。

查看源码：[oapi-sdk-java](https://github.com/larksuite/oapi-sdk-java/blob/v2_main/README.zh.md#%E4%B8%80%E9%94%AE%E5%88%9B%E5%BB%BA%E5%BA%94%E7%94%A8)

## 前提条件

- 安装 `oapi-sdk`，且 SDK 版本为 2.6.1 及以上。

  **Maven**：

  ```xml
  <dependency>
      <groupId>com.larksuite.oapi</groupId>
      <artifactId>oapi-sdk</artifactId>
      <version>2.6.1</version>
  </dependency>
  ```


## 快速开始示例

```java
import com.lark.oapi.Client;
import com.lark.oapi.scene.registration.RegisterApp;
import com.lark.oapi.scene.registration.RegisterAppOptions;
import com.lark.oapi.scene.registration.RegisterAppResult;
import com.lark.oapi.scene.registration.exception.RegisterAppException;

public class Main {
    public static void main(String[] args) {
        try {
            RegisterAppResult result = RegisterApp.register(
                RegisterAppOptions.builder()
                    .onQRCode(info -> {
                        System.out.println("请扫码: " + info.getUrl());
                        System.out.println("链接将在 " + info.getExpireIn() + " 秒后过期");
                    })
                    .onStatusChange(info -> {
                        // 处理状态变化："polling" | "slow_down" | "domain_switched"
                    })
                    .build()
            );

            System.out.println("App ID: " + result.getClientId());
            System.out.println("App Secret: " + result.getClientSecret());

            // 用获取到的凭据初始化 Client
            Client client = Client.newBuilder(result.getClientId(), result.getClientSecret()).build();

        } catch (RegisterAppException e) {
            // e.getCode(): "access_denied" | "expired_token" | ...
            // e.getDescription(): 错误描述
            System.err.println(e.getCode() + ": " + e.getDescription());
        }
    }
}
```

## RegisterApp.register 参数

`RegisterApp.register` 的方法签名为：

```java
public static RegisterAppResult register(RegisterAppOptions opts) throws RegisterAppException
```

`RegisterAppOptions` 通过 Builder 模式构建，支持以下字段：

| 参数 | 描述 | 类型 | 必填 | 默认 |
| --- | --- | --- | --- | --- |
| source | 来源标识，拼入二维码 URL 的 <code>source</code> 参数，格式为 <code>java-sdk/{source}</code> | String | 否 | - |
| domain | 自定义Lark认证基地址 | String | 否 | <code>https://accounts.larksuite.com</code> |
| larkDomain | 自定义 Lark 认证基地址，检测到 Lark 租户时自动切换 | String | 否 | <code>https://accounts.larksuite.com</code> |
| appPreset | 预设应用信息，用于初始化应用创建页。所有字段都是可选的，用户扫码后仍可在页面手动修改。SDK 会自动对原始值做 URL Encode。 | AppPreset | 否 | - |
| appPreset.avatar | 应用头像候选 URL，支持 1-6 个；传多个时默认选中第一个。图片格式、GIF 截帧、裁切和展示由页面/服务端处理。 | String / String[] / List&lt;String&gt; | 否 | - |
| appPreset.name | 应用名称，支持 <code>{user}</code> 占位符，由应用创建页替换为扫码用户名称。 | String | 否 | - |
| appPreset.desc | 应用描述，支持 <code>{user}</code> 占位符。 | String | 否 | - |
| onQRCode | 验证链接就绪时的回调，参数为 <code>QRCodeInfo</code>，包含 <code>url</code> 和 <code>expireIn</code> | Consumer&lt;QRCodeInfo&gt; | 是 | - |
| onStatusChange | 轮询状态变化时的回调，参数为 <code>StatusChangeInfo</code> | Consumer&lt;StatusChangeInfo&gt; | 否 | - |


## 回调参数类型

**QRCodeInfo**：

| 字段 | 类型 | 描述 |
| --- | --- | --- |
| url | String | 二维码 URL，用户扫码或在浏览器中打开此链接完成授权 |
| expireIn | int | 链接过期时间（秒），默认 600 |

**StatusChangeInfo**：

| 字段 | 类型 | 描述 |
| --- | --- | --- |
| status | String | 状态值，取值见下方说明 |
| interval | int | 当 status 为 `slow_down` 时，返回新的轮询间隔（秒） |

**状态常量**（定义在 `StatusChangeInfo` 类中）：

| 常量 | 值 | 描述 |
| --- | --- | --- |
| POLLING | `polling` | 授权等待中，继续轮询 |
| SLOW_DOWN | `slow_down` | 服务端要求降低轮询频率，间隔增加 5 秒 |
| DOMAIN_SWITCHED | `domain_switched` | 检测到 Lark 租户，域名已自动切换 |

## 返回值

`RegisterAppResult` 类的字段如下：

| 字段 | 类型 | 描述 |
| --- | --- | --- |
| clientId | String | 应用的 App ID |
| clientSecret | String | 应用的 App Secret |
| userInfo | UserInfo（可选） | 扫码授权的用户信息 |
| userInfo.openId | String（可选） | 扫码用户的 open_id |
| userInfo.tenantBrand | String（可选） | 租户品牌，取值为 `"Lark"` 或 `"lark"` |

## 错误处理

`RegisterApp.register` 方法在遇到错误时会抛出 `RegisterAppException` 或其子类异常。可以通过 `instanceof` 或 catch 特定子类来区分不同的错误场景：

| 异常类型 | code | 描述 |
| --- | --- | --- |
| `AccessDeniedException` | `access_denied` | 用户拒绝了授权 |
| `ExpiredException` | `expired_token` | 二维码过期或轮询超时 |
| `RegisterAppException` | `invalid_response` | 服务端响应异常 |
| `RegisterAppException` | `network_error` | 网络请求失败 |
| `RegisterAppException` | `invalid_argument` | 参数校验不通过（如 onQRCode 为 null） |
| `RegisterAppException` | 其他 | 服务端返回的其他错误 |

所有异常类型均包含 `getCode()` 和 `getDescription()` 方法。

**错误处理示例**：

```java
try {
    RegisterAppResult result = RegisterApp.register(opts);
} catch (AccessDeniedException e) {
    System.err.println("用户拒绝授权: " + e.getDescription());
} catch (ExpiredException e) {
    System.err.println("二维码已过期: " + e.getDescription());
} catch (RegisterAppException e) {
    System.err.println("注册失败: " + e.getCode() + " - " + e.getDescription());
}
```

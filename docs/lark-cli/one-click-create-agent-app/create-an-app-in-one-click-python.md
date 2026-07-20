---
document_id: '7633446179720793527'
directory_id: '7634017946344050100'
title: 一键创建应用（Python）
full_path: /mcp_open_tools/scan-to-create-an-app-in-one-click
breadcrumb:
- Lark CLI
- One-click create agent app
- Create an app in one click (Python)
document_type: GuideDocumentType
updated_at: 2026-06-05T03:29:57Z
source_url: https://open.larksuite.com/document/mcp_open_tools/scan-to-create-an-app-in-one-click
---

# 一键创建应用（Python）

Python SDK 提供了 `register_app`（同步）和 `aregister_app`（异步）方法，基于 OAuth 2.0 Device Authorization Grant（RFC 8628）协议实现一键创建应用。

调用该方法会返回一个验证链接，用户在Lark或 Lark 中打开该链接（或扫码）完成授权后，即可自动注册应用并获取凭据（App ID 和 App Secret），无需手动前往开发者后台创建。

查看源码：[oapi-sdk-python](https://github.com/larksuite/oapi-sdk-python)

## 前提条件
- Python 版本 `>= 3.7`

- 安装或升级 `lark-oapi` 至 `1.5.5` 及以上版本：
  ```
  pip install lark-oapi
  ```


## 快速开始示例

### 同步版

```python
import lark_oapi as lark
def handle_qr(info):
    print(f"请扫码: {info['url']}")
    print(f"有效期 {info['expire_in']} 秒")
def handle_status(info):
    print(f"状态: {info['status']}", info.get("interval", ""))
try:
    result = lark.register_app(
        on_qr_code=handle_qr,
        on_status_change=handle_status,
    )
    print(f"App ID: {result['client_id']}")
    print(f"App Secret: {result['client_secret']}")
    print(f"User Info: {result.get('user_info')}")
except Exception as e:
    print(f"失败: {e}")
```

### 异步版

```python
import asyncio
import lark_oapi as lark
async def main():
    result = await lark.aregister_app(
        on_qr_code=lambda info: print(f"请扫码: {info['url']}"),
    )
    print(f"App ID: {result['client_id']}")
    print(f"App Secret: {result['client_secret']}")
asyncio.run(main())
```


## API 描述

### 函数签名

| **函数**          | **签名**                                                                                                     | **说明**           |
| --------------- | ---------------------------------------------------------------------------------------------------------- | ---------------- |
| `register_app`  | `(on_qr_code, on_status_change=None, source=None, cancel_event=None, domain=..., lark_domain=...) -> dict` | 同步版，阻塞当前线程直到完成   |
| `aregister_app` | `async (on_qr_code, on_status_change=None, source=None, domain=..., lark_domain=...) -> dict`              | 异步版，返回 coroutine |

###  register_app 参数

| 参数 | 描述 | 类型 | 必填 | 默认值 |
| ---- | ---- | ---- | ---- | ---- |
| `on_qr_code` | 验证链接就绪时的回调，参数为 `{"url": str, "expire_in": int}` | function | 是 | - |
| `on_status_change` | 轮询状态变化回调，状态包括 `polling`、`slow_down`、`domain_switched` | function | 否 | - |
| `source` | 来源标识，会拼入二维码 URL 的 `source` 参数，格式为 `python-sdk/{source}` | string | 否 | `python-sdk` |
| `cancel_event` | 用于取消同步轮询的 `threading.Event` | threading.Event | 否 | - |
| `domain` | 自定义Lark账号域名 base URL | string | 否 | `https://accounts.larksuite.com` |
| `lark_domain` | 自定义 Lark 账号域名 base URL，检测到 Lark 租户时使用 | string | 否 | `https://accounts.larksuite.com` |
| `app_preset` | 创建页预填信息。所有字段都是选填，用户扫码后仍可在页面手动修改。调用方传原始值，SDK 自动 URL Encode | dict | 否 | - |
| `app_preset.avatar` | 应用头像 URL，支持 1-6 个；传多个时默认选中第一个。图片格式由 Web 页面处理：png / jpg / jpeg / webp / gif | string 或 list[string] | 否 | - |
| `app_preset.name` | 应用名称，支持 `{user}` 占位符，由 Web 页面替换为扫码用户名称 | string | 否 | - |
| `app_preset.desc` | 应用描述，支持 `{user}` 占位符 | string | 否 | - |

### on_qr_code 回调参数

| **字段**      | **类型** | **说明**                                  |
| ----------- | ------ | --------------------------------------- |
| `url`       | `str`  | 完整的二维码 URL（已拼好 `from`、`tp`、`source` 参数） |
| `expire_in` | `int`  | 二维码有效期（秒）                               |

### on_status_change 回调参数

| **字段**     | **类型**    | **说明**                                         |
| ---------- | --------- | ---------------------------------------------- |
| `status`   | `str`     | 状态：`polling` / `slow_down` / `domain_switched` |
| `interval` | `int`（可选） | 当前轮询间隔（秒），`slow_down` 时有值                      |

### 返回值

返回一个 `dict`，包含：
| **字段**          | **类型**     | **说明**                                               |
| --------------- | ---------- | ---------------------------------------------------- |
| `client_id`     | `str`      | App ID                                               |
| `client_secret` | `str`      | App Secret                                           |
| `user_info`     | `dict`（可选） | 扫码用户信息，含 `open_id`、`tenant_brand`（`feishu` 或 `lark`） |

### 异常类型

| **异常**                 | **触发条件**                         |
| ---------------------- | -------------------------------- |
| `AppAccessDeniedError` | 用户主动拒绝授权                         |
| `AppExpiredError`      | 二维码过期或轮询超时                       |
| `RegisterAppError`     | 其他错误（携带 `code`、`description` 属性） |
* * *

## 进阶用法

### 同步版：取消轮询

```python
import threading
import lark_oapi as lark
cancel_event = threading.Event()

# 另一个线程里可以随时取消

# cancel_event.set()

result = lark.register_app(
    on_qr_code=handle_qr,
    cancel_event=cancel_event,
)
```

### 异步版：取消任务

```python
import asyncio
import lark_oapi as lark
async def main():
    task = asyncio.create_task(lark.aregister_app(on_qr_code=handle_qr))
    # 某个时刻取消

    task.cancel()
    try:
        result = await task
    except asyncio.CancelledError:
        print("已取消")
asyncio.run(main())
```

### 拿到凭据后初始化 Client

```python
result = lark.register_app(on_qr_code=handle_qr)
client = lark.Client.builder() \
    .app_id(result['client_id']) \
    .app_secret(result['client_secret']) \
    .build()
```
  
* * *

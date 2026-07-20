---
document_id: '7073823165957537798'
directory_id: '7073460768595378181'
title: 打开小程序
full_path: /uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-a-gadget
breadcrumb:
- Developer Guides
- AppLink Protocol
- Supported protocol
- Open a Gadget
document_type: GuideDocumentType
updated_at: 2022-03-11T16:42:50Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-a-gadget
---

# 打开小程序
::: note 
从Lark 3.8.0 版本开始支持。
:::
## 使用场景
打开一个小程序或者小程序中的一个页面

## 协议
`https://applink.larksuite.com/client/mini_program/open`

## 参数
::: note 
每一个参数都应当编码。
:::
| 字段         | 必填           | 说明        | 
| --------- | --------------- | -------   | 
|**appId** |    是      | 小程序 appId(可从「开发者后台-凭证与基础信息」获取) | 
|**mode** | PC 必填 | PC小程序启动模式，枚举值包括：<br>`sidebar-semi`：聊天的侧边栏打开<br>`appCenter`：工作台中打开<br>`window`：独立大窗口打开<br>`window-semi`：独立小窗口打开，3.33版本开始支持此模式 | 
|**path** | 否 | 需要跳转的页面路径，路径后可以带参数。也可以使用 path_android、path_ios、path_pc 参数对不同的客户端指定不同的path | 
|**path_android** | 否 | 同 path 参数，Android 端会优先使用该参数，如果该参数不存在，则会使用 path 参数 | 
|**path_ios** | 否 | 同 path 参数，iOS 端会优先使用该参数，如果该参数不存在，则会使用 path 参数 | 
|**path_pc** | 否 | 同 path 参数，PC 端会优先使用该参数，如果该参数不存在，则会使用 path 参数 |  | 
|**min_lk_ver** | 否         | 指定 AppLink 协议能够兼容的最小Lark版本，使用三位版本号 x.y.z。如果当前Lark版本号小于min_lk_ver，打开该 AppLink 会显示为兼容页面 | 

::: note 
也可使用 min_lk_ver_android、min_lk_ver_ios、min_lk_ver_pc 参数对不同的客户端指定不同的版本。
::: 

`path` 参数值可以携带 Query 参数。比如：
```js
path=pages%2Findex%3Ffoo%3Dbar
// 上述值decode以后是 pages/index?foo=bar
```
通过 [App.onLaunch](/document/uYjL24iN/uMDNuMDNuMDN) 或者 [tt.getLaunchOptionsSync](/document/uYjL24iN/uAzM1YjLwMTN24CMzUjN) 即可取得：
```json
{
	"path": "pages/index",
    "query": {
    	"foo": "bar" // 启动小程序时传入的path参数值中携带的query参数
    },
    "scene": "1000", // 场景值
    "subScene": "",
    "group_id": ""
}
```

## 使用示例
#### 1. 打开小程序                                            
`https://applink.larksuite.com/client/mini_program/open?appId=1234567890&mode=window`
#### 2. 打开小程序的一个页面 pages/home                                     
`https://applink.larksuite.com/client/mini_program/open?appId=1234567890&mode=window&path=pages%2fhome`

#### 3. 打开小程序的一个页面带参数 pages/home?xid=123
`https://applink.larksuite.com/client/mini_program/open?appId=1234567890&mode=window&path=pages%2fhome%3fxid%3d123`

#### 4. 在 PC 端打开页面 pages/pc_home?pid=123，在其他端打开页面 pages/home?xid=123
`https://applink.larksuite.com/client/mini_program/open?appId=1234567890&mode=window&path=pages%2fhome%3fxid%3d123&path_pc=pages%2fpc_home%3fpid%3d123`

#### 5. 在 PC 4.2.0 及以上版本支持打开小程序，PC 4.2.0 以下版本提示不支持
`https://applink.larksuite.com/client/mini_program/open?appId=1234567890&mode=window&min_lk_ver_pc=4.2.0`

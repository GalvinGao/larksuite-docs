---
document_id: '7073823165957685254'
directory_id: '7073460768595378181'
title: 打开网页应用
full_path: /uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-an-h5-app
breadcrumb:
- Developer Guides
- AppLink Protocol
- Supported protocol
- Open an H5 app
document_type: GuideDocumentType
updated_at: 2022-07-06T07:38:46Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-an-h5-app
---

# 打开网页应用
::: note 
从Lark 3.19.0 版本开始支持。
:::
## 使用场景
打开一个已安装的H5应用

## 协议
`https://applink.larksuite.com/client/web_app/open`

####  参数

| 字段         | 必填           | 说明        | 
| --------- | --------------- | -------   | 
|**appId** |    是      | H5应用的 appId(可从「开发者后台-凭证与基础信息」获取) | 
|**mode** | 否 | 打开H5应用的容器模式，枚举值包括<br> `appCenter`：在工作台打开，3.20版本开始支持（缺省值） <br> `window`：在独立窗口打开，3.20版本开始支持 <br> `sidebar`：在侧边栏打开，3.40版本开始支持 | 
|**path** | 否 | 访问H5应用的具体某个页面，path参数将替换H5应用URL的path部分（注意：path中不应该出现#和?字符，否则会导致最终的H5页面URL结构异常） <br>也可以使用 path_android、path_ios、path_pc 参数对不同的客户端指定不同的path <br>3.20版本开始支持 | 
|**path_android** | 否 | 同 path 参数，Android 端会优先使用该参数，如果该参数不存在，则会使用 path 参数。 <br>3.20版本开始支持 | 
|**path_ios** | 否 | 同 path 参数，iOS 端会优先使用该参数，如果该参数不存在，则会使用 path 参数 <br>3.20版本开始支持 | 
|**path_pc** | 否 | 同 path 参数，PC 端会优先使用该参数，如果该参数不存在，则会使用 path 参数 <br>3.20版本开始支持 | 
|**lk_target_url** | 否 | 访问H5应用的具体某个页面，针对网页path中包含#或?字符时，可使用该参数，而不使用`path`参数。需要填写H5应用某个具体页面的完整URL（协议名`scheme`和域名`domain`应当与开发者后台配置的应用首页相匹配），并进行[URL encode](https://meyerweb.com/eric/tools/dencoder/)后使用。具体参考示例3 <br>**LarkV5.12版本开始支持。Lark低版本中无法解析此参数，将打开应用首页** |
如果需要携带参数，将预期的H5应用URL的query作为applink的query即可，具体参考示例。

## 使用示例
#### 1. 使用 appId 打开H5应用
`https://applink.larksuite.com/client/web_app/open?appId=1234567890&path=bytedance/d/home.htmld&mode=window`
#### 2. 使用appid打开H5应用并携带参数，path是/a/b，参数是xxd=123
`https://applink.larksuite.com/client/web_app/open?appId=1234567890&path=/a/b&xxd=123`

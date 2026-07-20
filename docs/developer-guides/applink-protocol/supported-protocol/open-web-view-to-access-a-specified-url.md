---
document_id: '7073823165957636102'
directory_id: '7073460768595378181'
title: 端内web-view访问指定URL
full_path: /uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-the-web-view-in-feishu-to-access-the-specified-url
breadcrumb:
- Developer Guides
- AppLink Protocol
- Supported protocol
- Open web-view to access a specified URL
document_type: GuideDocumentType
updated_at: 2023-12-07T08:11:42Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-the-web-view-in-feishu-to-access-the-specified-url
---

# 打开端内web-view访问指定URL
::: note 
从Lark 3.41.0 版本开始支持。
:::
## 使用场景
用户在端内点击这类applink，不必跳转外部浏览器，可以直接在聊天的侧边栏、或Lark的独立窗口中打开指定的网页。
可以配置这类applink在消息卡片的“查看详情”跳转上，使用户连贯地消费消息中的详情内容。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d37e55f55fd888cf7486b03b27de157f_cXrYDqR775.png?height=962&lazyload=true&maxWidth=600&width=1649)
## 协议
`https://applink.larksuite.com/client/web_url/open`
##  参数

| 字段         | 必填           | 说明        | 
| --------- | --------------- | -------   | 
|**url** |    是      | 客户端内打开的目标页面链接，需要encodeURIComponent处理，若链接包含特殊字符，则先将特殊字符进行百分号编码，再进行Encode处理。V4.2.0+版本支持lark协议 | 
|**mode** | 是 | PC端打开的容器模式，枚举值包括：<br>`sidebar-semi` 在侧边栏打开；<br>`window` 在独立窗口打开； <br> `appCenter` 在Lark导航栏的标签页打开。V7.5+版本支持，低于此版本无法打开网页 |
|**height** | 否 | PC端自定义独立窗口高度（仅当`mode`为`window`时生效），Lark5.12版本开始支持<br>**最小值**：480<br>**最大值**：屏幕的高度<br>**默认值**：Lark窗口的高度 | 
|**width** | 否 | PC端自定义独立窗口宽度（仅当`mode`为`window`时生效），Lark5.12版本开始支持<br>**最小值**：640<br>**最大值**：屏幕的宽度<br>**默认值**：Lark窗口的宽度 | 



## 使用示例
#### 1. 在PC端侧边栏打开网页：

[https://applink.larksuite.com/client/web_url/open?mode=sidebar-semi&url=https%3a%2f%2fwww.larksuite.com%2f](https://applink.larksuite.com/client/web_url/open?mode=sidebar-semi&url=https%3a%2f%2fwww.larksuite.com%2f)

#### 2. 在PC端独立窗口中打开网页：

[https://applink.larksuite.com/client/web_url/open?mode=window&url=https%3a%2f%2fwww.larksuite.com%2f](https://applink.larksuite.com/client/web_url/open?mode=window&url=https%3a%2f%2fwww.larksuite.com%2f)

#### 3. 在PC端独立窗口中打开网页，并指定窗口尺寸大小（最小值640*480，最大值为屏幕宽高）：

[https://applink.larksuite.com/client/web_url/open?url=https%3a%2f%2fwww.larksuite.com%2f&mode=window&height=700&width=1200](https://applink.larksuite.com/client/web_url/open?url=https%3a%2f%2fwww.larksuite.com%2f&mode=window&height=700&width=1200)

#### 4. 在PC端侧边栏打开网页，网页链接中携带Query和Fragment：

目标URL为： [https://open.larksuite.com?key1=value1&key2=中文#position2345](https://open.larksuite.com?key1=value1&key2=中文#position2345)

##### 4.1 对URL 里的参数部分进行编码 

`encodeURIComponent("中文") '%E4%B8%AD%E6%96%87'`

第一步得到的URL为 

[https://open.larksuite.com?key1=value1&key2=%E4%B8%AD%E6%96%87#position2345]( https://open.larksuite.com?key1=value1&key2=%E4%B8%AD%E6%96%87#position2345)

##### 4.2 URL 作为参数放置到 Applink 的 query 里，URL 进行编码

`encodeURIComponent("https://open.larksuite.com?key1=value1&key2=%E4%B8%AD%E6%96%87#position2345")`

编码结果为：
`https%3A%2F%2Fopen.larksuite.com%3Fkey1%3Dvalue1%26key2%3D%25E4%25B8%25AD%25E6%2596%2587%23position2345`

##### 4.3 将编码后的URL 作为参数组装完整的Applink URL
[https://applink.larksuite.com/client/web_url/open?url=https%3A%2F%2Fopen.larksuite.com%3Fkey1%3Dvalue1%26key2%3D%25E4%25B8%25AD%25E6%2596%2587%23position2345&mode=sidebar-semi](https://applink.larksuite.com/client/web_url/open?url=https%3A%2F%2Fopen.larksuite.com%3Fkey1%3Dvalue1%26key2%3D%25E4%25B8%25AD%25E6%2596%2587%23position2345&mode=sidebar-semi)

---
document_id: '6967240092650323974'
directory_id: '7130175417072861190'
title: 步骤二（可选）：鉴权调用 JSAPI
full_path: /uYjL24iN/uEzM4YjLxMDO24SMzgjN
breadcrumb:
- Developer Guides
- Develop Web Apps
- Development Guide
- 'Step 2 (Optional): Authenticate and call JSAPI'
document_type: GuideDocumentType
updated_at: 2024-10-21T03:46:45Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEzM4YjLxMDO24SMzgjN
---

# 步骤二（可选）：鉴权调用 JSAPI

在开放平台创建好自建应用后，即可在本地开发相应的 Web 项目。如果你的 Web 项目需要调用 [H5 JSAPI ](/document/uYjL24iN/uMTMuMTMuMTM/)，则需要先通过鉴权。JSAPI 鉴权是指Lark验证网页应用有权限访问的 JSAPI 范围。本文主要介绍 JSAPI 鉴权的工作流程，以及各Lark版本的鉴权策略。

:::note
**示例代码**
<br>
关于如何在 Web 项目中实现 JSAPI 鉴权，开放平台提供了相应的网页应用开发教程。开发教程包含完整的网页应用示例代码，你可以查看代码介绍、体验开发流程，从而帮助你了解和使用 JSAPI 鉴权。详情参见[快速开发网页应用](/document/home/integrating-web-apps-in-5-minutes/debug-and-release)。
:::


## 鉴权流程

网页应用的 JSAPI 鉴权是从应用页面发起的，如果前端页面调用需要鉴权的 JSAPI 方法，则在页面加载时需要向服务端发起请求，获取鉴权参数（appId、timestamp、nonceStr、signature）。通过这些参数，前端页面调用 [config](/document/uYjL24iN/uQjMuQjMuQjM/authentication/h5sdkconfig) 接口进行鉴权，鉴权成功后即可调用 JSAPI。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/629c13ad366cd72e0e3463c036db79c2_kNP4uVTHLJ.png?height=794&lazyload=true&maxWidth=750&width=914)


## 注意事项

- 获取 **access_token**、**jsapi_ticket** 等鉴权流程的详细介绍，可参见[示例代码介绍](/document/home/integrating-web-apps-in-5-minutes/debug-and-release)。

- 网页应用 JSAPI 列表可参见 [H5 JSAPI 总览](/document/uYjL24iN/uMTMuMTMuMTM/)。 除了 [requestAuthCode](/document/uYjL24iN/uUzMuUzMuUzM/20220308)、[closeWindow](/document/uYjL24iN/uYTOuYTOuYTO/closewindow)、[requestAccess](/document/uYjL24iN/uUzMuUzMuUzM/requestaccess) API，其它所有 JSAPI 在页面被调用时，均需要先完成鉴权。

- 接入方前端调用 JSAPI 前的鉴权过程，需要接入方服务端配合完成。建议你合理安排前端和服务端开发人员共同参与业务代码的编写。

- 接入方前端只能调用 JSAPI，如果你需要调用服务端 API，请在接入方服务端调用，详情参见 [如何调用服务端API](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)。

## 鉴权策略

为了兼顾安全性和便捷性，网页应用的鉴权策略经过了多个版本的迭代。

| 版本 | 说明 |
| --- | --- |
| 容器级别的鉴权策略（Lark版本 <  V4.0）</md-td> | 网页容器简单来说是网页所在的环境，从客户端视角来看：<br>- Android 端：一个 Activity 对应一个容器。<br>- iOS 端：一个 ViewController 对应一个容器。鉴权信息作用域为网页应用容器，即容器内只需要鉴权一次，便可在应用容器的生命周期内有效。 |
| URL 级别的鉴权策略（V4.0 <= Lark版本 < V5.1）</md-td> | 鉴权信息作用域为 URL，URL 改变后需要重新鉴权。 |
| 染色级别的鉴权策略（Lark版本 >= V5.1 ） | 在 URL 的基础上，增加了染色机制，即只要在当前页面的回退页面栈中出现过当前页面的父路径，且该父路径鉴权通过包含应用鉴权信息，则该页面就可以被染上同样的应用鉴权信息。 |


### 示例一

假设打开 pathA 后获得了应用鉴权信息 a1，接着打开 pathB 的鉴权情况如下表所示。
- pathA 指 http://www.example.com/home
- pathB 指 http://www.example.com/detail
  

| 鉴权策略 | 打开 pathA | 打开 pathB |
| --- | --- | --- |
| 容器级别 | 获得鉴权信息 a1 | 仍可获得鉴权信息 a1。<br>原因：在同一容器生命周期内，因此仍然可以获得鉴权信息。 |
| URL 级别 | 获得鉴权信息 a1 | 不能获得鉴权，需要重新对页面鉴权。<br>原因：页面 URL 发生了变化，因此需要重新鉴权。 |
| 染色级别 | 获得鉴权信息 a1 | 不能获得鉴权，需要重新对页面鉴权。<br>原因：pathB 的回退页面栈中不包含 pathB 的父路径，不满足染色条件，因此需要重新鉴权。 |



### 示例二

假设打开 pathA 后获得了应用鉴权信息 a1，接着打开 pathC 的鉴权情况如下：
- pathA 指 http://www.example.com/home
- pathC 指 http://www.example.com/home/route

| 鉴权策略 | 打开 pathA | 打开 pathC |
| --- | --- | --- |
| 容器级别 | 获得鉴权信息 a1 | 仍可获得鉴权信息 a1。<br>原因：在同一个容器生命周期内，因此仍然可以获得鉴权信息。 |
| URL 级别 | 获得鉴权信息 a1 | 不能获得鉴权，需要重新对页面鉴权。<br>原因：页面 URL 发生了变化，因此需要重新鉴权。 |
| 染色级别 | 获得鉴权信息 a1 | 仍可获得鉴权信息 a1。<br>原因：pathC 的回退页面栈中包含 pathC 的父路径 pathA，因此仍然可以获得 pathA 的鉴权信息。 |


:::note
所有鉴权策略均需确认容器是否唯一，即在网页应用内是否又开启了新容器。若开启了新容器，则需重新调用 config 接口进行鉴权。
:::
  
开启新的网页容器是指在客户端上新打开一个网页容器承载网页展现，即网页所在的环境与之前不一致。常见的开启新容器的场景如下所示。
  
- 使用 [openSchema](/document/uAjLw4CM/uYjL24iN/block/api/navigator/openschema) 方式打开新链接 ：openSchema 是一个 JSAPI，用于在新窗口打开网页、文档、小程序等。
- 使用 [AppLink](/document/uYjL24iN/ucjN1UjL3YTN14yN2UTN) 打开新链接：AppLink 是一个 URL 协议，用于打开Lark或者其中的一个功能。
- 前端通过 `window.open(xxx , '_self')` 打开新页面时，iOS 端按照浏览器标准实现新容器的开启。Android 端在旧的网页容器中渲染页面，不会开启新容器。
    
  其中，`window.open` 是 JavaScript 函数，用于打开一个新窗口或改变原窗口，`_self`表示在原窗口打开。


## Config 接口错误码

本章节提供了 Config 接口可能产生的错误码。无论你使用什么方式（例如，前端 console、端侧日志等），都需要先得到 Config 接口的 Response，然后根据 Response 中的错误码，采取相应的处理措施。

### [Errno错误码](/document/uYjL24iN/uAjMuAjMuAjM/errno)

| errno | errString | 描述 |
| --- | --- | --- |
| 102 | Internal error | 内部错误，通常是应用容器发生了某些异常。 |
| 104 | Invalid parameter | 参数错误，参考下文 errorCode 错误码表中的 1012 错误码。 |
| 305 | Network failure | 网络失败。 |
| 2601001 | Server-side data exception | 服务端数据异常，常见于服务端宕机等场景。 |
| 2601002 | Authentication failed. %s (error code: %s) | 鉴权失败，按照下文 errorCode 错误码表，进一步排查。 |


### errorCode 错误码

| 错误码 | 描述 | 排查建议 |
| --- | --- | --- |
| 1012 | 参数类型错误 | 根据下面的参数校验规则，包括类型校验与值校验，进行自检。<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6fee21a7237ef2c0a7ee5531736d4a14_P81ZwOh2sW.png?height=488&lazyload=true&width=1172) |
| 10001 | 网络请求失败 | 稍后重试。 |
| 333441 | 签名错误 | 签名错误是指客户端将用户构造的 signature 字段传递至服务端验证时，与服务端构造的 signature 不一致，所以服务端会认为该签名不合法。<br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fe6c7b09105c16b704c444ce9ff634bf_eA2pblbx9M.png?height=393&lazyload=true&width=1113)<br>可能原因：<br>- 构建的参数不一致。<br>- 生成 verifyStr 规则错误。<br>- sha1 算法出错。<br>排查建议：<br>- 确认签名算法为 sha1。<br>- 确认 config 接口内的 nonceStr 参数名为驼峰写法。<br>- 确认 config 接口内的 nonceStr、timestamp 参数与服务端生成签名时的 noncestr、timestamp 参数一致。<br>- 确认 config 接口内的 appid 参数与获取 access_token 时的 appid 参数一致。<br>- 确认 URL 是页面完整的 URL（请在当前页面`alert(location.href.split('#')[0])`确认），包括`http(s)://`部分，以及`?`后面的 GET 参数部分，但不包括`'#'(hash)`后面的部分。<br>- 确保缓存了 access_token 和 jsapi_ticket。<br>- 如果是单页面应用，且使用 react-router 或 vue-router 等（类似 pushState、replaceState）进行页面跳转时（即 vue-router 的 history 模式或者 react-router 的 browserHistory），可以尝试将最后一次不用 vue-router 或者 react-router 跳转的页面 URL 传到当前页面进行鉴权，该方式需要根据实际情况进行处理。 |
| 333448 | 页面不在安全域名内 | 应用开发者需要在 **开发者后台 > 应用详情页 > 安全设置 > H5可信域名** 中检查需要调用 JSAPI 接口的页面所在域名。 |
| 333449 | 应用不可见 | 应用对该用户没有可见性，应用开发者需要在 **开发者后台 > 应用详情页 > 版本管理与发布 > 创建版本 > 可用范围** 中配置可见性。 |
| 1014 | 网络异常错误 | 检查设备网络。 |
| \- | Android 无法调用相关API，提示 find no handler 等 | Lark V4.0 版本之前（不包含 V4.0）仅支持在工作台打开网页应用，如果在非工作台打开网页应用需在 URL 后增加`app_id=xxx`参数信息。 |
| 10002 | 网络请求返回数据格式错误 | \- |
| 333430 | userId 或者 appId 不合法 | \- |
| 333440 | app 不存在 | 前往 **开发者后台 > 应用详情页 > 凭证与基础信息** 检查 appId 是否正确。 |
| 333442 | app 没找到有效的 jsapi_ticket | 检查 config 中的 appid 与用来获取 jsapi_ticket 的 appid 一致。 |
| 333443 | 签名重复 | 10 分钟内再次验签，请 10 分钟后重试。 |
| 333444 | 签名过期 | 签名有效期为 10 分钟，每次调用 config 前重新计算签名，不建议缓存签名。 |
| 333445 | JSAPI 未授权 | 调用 Config 接口时，在参数 jsApiList 里添加相应 API。 |
| 333446 | JSAPI 不存在 | 前往 [H5 JSAPI 总览](/document/uYjL24iN/uMTMuMTMuMTM/) 查阅 JSAPI，确认是否写错。 |
| 333447 | 安全域名未设置 | 应用开发者需要在 **开发者后台 > 应用详情页 > 安全设置 > H5可信域名** 中设置需要调用 JSAPI 接口的页面所在域名。 |
| 9999169x | invalid session 用户登录态校验失败，x=[1-4] | \- |


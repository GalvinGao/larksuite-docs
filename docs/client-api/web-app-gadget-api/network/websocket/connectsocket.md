---
document_id: '6965379541104590853'
directory_id: '6907567266540650498'
title: connectSocket
full_path: /uYjL24iN/ugDMx4COwEjL4ATM
breadcrumb:
- Client API
- Web app/Gadget API
- Network
- WebSocket
- connectSocket
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:00Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDMx4COwEjL4ATM
---

# connectSocket(Object object)

创建一个 WebSocket 连接实例，并通过返回的 `socketTask` 操作该连接。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/web-socket/web-socket" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入


继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| url | string | 是 |  | Socket 连接地址<br>**示例值**：wss://echo.websocket.org |
| header | object | 否 |  | HTTP Header<br>**示例值**：{'content-type': 'application/json'} |
| protocols | string[] | 否 |  | 子协议数组<br>**示例值**：['protocol1'] |



## 输出
返回值：`SocketTask`，该对象的方法列表参见下表：

:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::

| 方法 | 介绍 |
| --- | --- |
| [send(Object msg)](/document/uYjL24iN/ugDOugDOugDO/sockettask/send) | 发送数据 |
| [close(Object option)](/document/uYjL24iN/ugDOugDOugDO/sockettask/close) | 关闭 Socket 连接 |
| [onOpen(function callback)](/document/uYjL24iN/ugDOugDOugDO/sockettask/onopen) | 监听连接成功的事件回调 |
| [onClose(function callback)](/document/uYjL24iN/ugDOugDOugDO/sockettask/onclose) | 监听连接关闭的事件回调 |
| [onMessage(function callback)](/document/uYjL24iN/ugDOugDOugDO/sockettask/onmessage) | 监听接收到服务器的消息事件回调 |




## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/web-socket/web-socket" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>

  </div>
</div> 
:::

```js
let socketTask = tt.connectSocket({
    url: 'wss://echo.websocket.org',
    success() {
        console.log('Build WebSocketTask success');
    },
    fail(err) {
        console.error('Build WebSocketTask failed', err);
    }
});

```




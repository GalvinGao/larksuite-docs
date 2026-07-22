---
document_id: '7073692582769606662'
directory_id: '7073451436034064389'
title: SocketTask.onOpen
full_path: /uYjL24iN/ugDOugDOugDO/sockettask/onopen
breadcrumb:
- Client API
- Web app/Gadget API
- Network
- WebSocket
- SocketTask
- SocketTask.onOpen
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:06Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDOugDOugDO/sockettask/onopen
---

# SocketTask.onOpen(function callback)

监听 WebSocket 连接打开事件



## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/web-socket/web-socket" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |




## 输入
无

## 输出
回调函数返回对象的属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| header | object | 返回 HTTP Header |




## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/web-socket/web-socket" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
const socketTask = tt.connectSocket({"url":"wss://echo.websocket.org"});
socketTask.onOpen(function(res) {
    console.log(JSON.stringify(res));
});

```




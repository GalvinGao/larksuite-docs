---
document_id: '7073693024735379461'
directory_id: '7073451436034064389'
title: SocketTask.send
full_path: /uYjL24iN/ugDOugDOugDO/sockettask/send
breadcrumb:
- Client API
- Web app/Gadget API
- Network
- WebSocket
- SocketTask
- SocketTask.send
document_type: GuideDocumentType
updated_at: 2022-06-23T06:13:28Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDOugDOugDO/sockettask/send
---

# SocketTask.send(Object object)

通过 WebSocket 连接发送数据



## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/web-socket/web-socket" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| data | string｜arraybuffer | 是 |  | 需要发送的内容<br>**示例值**：send text<br><md-alert type="tip" icon="none"><br>iOS 端：Lark[V5.2.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持arraybuffer<br></md-alert> |



## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性



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
    socketTask.send({
      data: "send text",
      success(res) {
        console.log(JSON.stringify(res));
      },
      fail(res) {
        console.log(`send fail: ${JSON.stringify(res)}`);
      }
  });
});




```



---
document_id: '7073693024736100357'
directory_id: '7073451436034064389'
title: SocketTask.close
full_path: /uYjL24iN/ugDOugDOugDO/sockettask/close
breadcrumb:
- Client API
- Web app/Gadget API
- Network
- WebSocket
- SocketTask
- SocketTask.close
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:06Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDOugDOugDO/sockettask/close
---

# SocketTask.close(Object object)

关闭 WebSocket 连接



## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/web-socket/web-socket" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| code | number | 否 | 1000 | 关闭连接状态码<br>**示例值**：1000 |
| reason | string | 否 |  | 关闭连接消息<br>**示例值**：close socket |



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
socketTask.close({
    code: 1000,
    reason: "close socket",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`close fail: ${JSON.stringify(res)}`);
    }
});

```



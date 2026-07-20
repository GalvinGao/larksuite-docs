---
document_id: '7073691561008021509'
directory_id: '7073451436034064389'
title: SocketTask.onMessage
full_path: /uYjL24iN/ugDOugDOugDO/sockettask/onmessage
breadcrumb:
- Client API
- Web app/Gadget API
- Network
- WebSocket
- SocketTask
- SocketTask.onMessage
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:09Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDOugDOugDO/sockettask/onmessage
---

# SocketTask.onMessage(function callback)

监听 WebSocket 接受到服务器的消息事件



## 支持说明
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">应用能力</md-th>
      <md-th style="width: 20%;">Android</md-th>
       <md-th style="width: 20%;">iOS</md-th>
      <md-th style="width: 20%;">PC</md-th>
      <md-th style="width: 20%;">预览效果</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>小程序</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/web-socket/web-socket" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>/</md-td>
</md-tr>



</md-tbody>
</md-table>
:::



## 输入
无

## 输出
回调函数返回对象的属性：
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 30%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                data
            </md-td>
            <md-td>
                string｜arraybuffer
            </md-td>
            <md-td>
                服务器返回的消息
<md-alert type="tip" icon="none">
iOS 端：Lark[V5.2.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持arraybuffer 
</md-alert>  
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::



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
socketTask.onMessage(function(res) {
    console.log(JSON.stringify(res));
});

```




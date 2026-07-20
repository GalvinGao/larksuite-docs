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
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/web-socket/web-socket" fontSize="14">预览</md-preview-app>
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


继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 20%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th style="width: 10%;">
                必填
            </md-th>
            <md-th style="width: 10%;">
                默认值
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                url
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                Socket 连接地址

**示例值**：wss://echo.websocket.org
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                header
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                HTTP Header

**示例值**：{'content-type': 'application/json'}
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                protocols
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                子协议数组

**示例值**：['protocol1']
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 输出
返回值：`SocketTask`，该对象的方法列表参见下表：

:::html
<md-alert type="tip">
点击下表中的方法名，查看对应API的支持说明、调用方法
</md-alert>
:::
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 50%;">方法</md-th>
      <md-th style="width: 50%;">介绍</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>[send(Object msg)](/document/uYjL24iN/ugDOugDOugDO/sockettask/send)</md-td>
      <md-td>发送数据</md-td>
    </md-tr>
        <md-tr>
      <md-td>[close(Object option)](/document/uYjL24iN/ugDOugDOugDO/sockettask/close)</md-td>
      <md-td>关闭 Socket 连接</md-td>
    </md-tr>

    <md-tr>
      <md-td>[onOpen(function callback)](/document/uYjL24iN/ugDOugDOugDO/sockettask/onopen)</md-td>
      <md-td>监听连接成功的事件回调</md-td>
    </md-tr>
    
    <md-tr>
      <md-td>[onClose(function callback)](/document/uYjL24iN/ugDOugDOugDO/sockettask/onclose)</md-td>
      <md-td>监听连接关闭的事件回调</md-td>
    </md-tr>
   
    
        <md-tr>
      <md-td>[onMessage(function callback)](/document/uYjL24iN/ugDOugDOugDO/sockettask/onmessage)</md-td>
      <md-td>监听接收到服务器的消息事件回调</md-td>
    </md-tr>
    
</md-tbody>
</md-table>
:::



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




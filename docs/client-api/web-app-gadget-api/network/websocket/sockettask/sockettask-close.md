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
                code
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                1000
            </md-td>
            <md-td>
                关闭连接状态码

**示例值**：1000
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                reason
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                关闭连接消息

**示例值**：close socket
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


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



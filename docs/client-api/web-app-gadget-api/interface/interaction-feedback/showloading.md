---
document_id: '6965379543684300806'
directory_id: '6907567266541240322'
title: showLoading
full_path: /uYjL24iN/uMDNy4yM0IjLzQjM
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Interaction Feedback
- showLoading
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:11Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMDNy4yM0IjLzQjM
---

# showLoading(Object object)

显示灰色背景的 loading 提示框。


:::html 
<md-alert type="tip">
注意事项：
- 该提示框**不会**主动隐藏。
- loading 的实现基于 toast，等同于`icon`为`loading`，`duration`为`24`小时的 toast。
多次弹出 toast/loading 时，后一个会**立刻**覆盖前一个。
</md-alert>
:::


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
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/toast/toast" fontSize="14">预览</md-preview-app>
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
                title
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                消息内容。最多显示7个汉字长度的文本

**示例值**：加载中
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                mask
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>false</md-td>
            <md-td>
                是否显示透明蒙层，防止触摸穿透

**示例值**：false
<md-alert type="tip" icon="none">
PC 端：暂不支持
</md-alert> 
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/toast/toast" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.showLoading({
    "title": "加载中",
    "mask": false,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`showLoading fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：

```json
{
    "errMsg": "showLoading:ok"
}
``` 


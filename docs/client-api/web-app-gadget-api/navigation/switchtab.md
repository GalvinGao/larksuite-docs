---
document_id: '6965379543684268038'
directory_id: '6907567266537734145'
title: switchTab
full_path: /uYjL24iN/ukTOz4SO5MjL5kzM
breadcrumb:
- Client API
- Web app/Gadget API
- Navigation
- switchTab
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:48Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukTOz4SO5MjL5kzM
---

# switchTab(Object object)

跳转到指定 TabBar 页面，并关闭其他所有非 TabBar 页面。



:::html
<md-alert type="tip">

将要跳转到的 TabBar 页面需要在app.json中定义。

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
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/navigator/navigator" fontSize="14">预览</md-preview-app>
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
                需要跳转的 TabBar 页面的路径。**路径后不能携带参数**

**示例值**：/page/API/index
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/navigator/navigator" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          
  </div>
</div> 
:::

```js
tt.switchTab({
    "url": "/pages/api/index",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`switchTab fail: ${JSON.stringify(res)}`);
    }
});
```

返回值示例：
```json
{"errMsg":"switchTab:ok"}
``` 




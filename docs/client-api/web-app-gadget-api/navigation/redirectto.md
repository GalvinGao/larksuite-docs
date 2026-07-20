---
document_id: '6965379543684104198'
directory_id: '6907567266537734145'
title: redirectTo
full_path: /uYjL24iN/ucTOz4yN5MjL3kzM
breadcrumb:
- Client API
- Web app/Gadget API
- Navigation
- redirectTo
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:45Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucTOz4yN5MjL3kzM
---

# redirectTo(Object object)


关闭当前页面，跳转到指定页面

:::html
<md-alert type="tip">
**不能**跳转到 TabBar 页面。
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
      <md-td>/
      </md-td>
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
                需要跳转的应用内非 tabBar 的页面的路径, 路径后可以带参数。可以以这种形式带上参数 "path?key1=value1&key2=value2"。参数在指定页面的 onLoad 参数以对象形式传递。

**示例值**：path?key1=value1&key2=value2
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
tt.redirectTo({
    "url": "/pages/navigateTo/index",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`redirectTo fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "redirectTo:ok"
}
```

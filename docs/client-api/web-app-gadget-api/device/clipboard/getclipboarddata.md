---
document_id: '6965379543684169734'
directory_id: '6907567266536734721'
title: getClipboardData
full_path: /uYjL24iN/uczNx4yN3EjL3cTM
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Clipboard
- getClipboardData
document_type: GuideDocumentType
updated_at: 2022-03-11T04:16:46Z
source_url: https://open.larksuite.com/document/uYjL24iN/uczNx4yN3EjL3cTM
---

# getClipboardData(Object object)

获取系统粘贴板数据。

:::html
<md-alert type="tip">
从**3.36版本**开始后, 调用前需要用户授权 `scope.clipboard`。了解如何授权，可查看[API 权限](/document/uYjL24iN/uITMuITMuITM)。
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
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-clipboard-data/get-clipboard-data" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app></md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性


## 输出

`success`返回对象的扩展属性：
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
                string
            </md-td>
            <md-td>
                粘贴板数据
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-clipboard-data/get-clipboard-data" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.getClipboardData({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getClipboardData fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "getClipboardData:ok",
    "data": "hello world"
}
```


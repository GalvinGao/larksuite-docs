---
document_id: '6965379543683776518'
directory_id: '6907567266536308737'
title: getStorageInfo
full_path: /uYjL24iN/ucTOx4yN5EjL3kTM
breadcrumb:
- Client API
- Web app/Gadget API
- Cache
- getStorageInfo
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:42Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucTOx4yN5EjL3kTM
---

# getStorageInfo(Object object)

获取本地缓存数据的相关信息。


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
      <md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path=" /page/API/pages/storage/storage" fontSize="14">预览</md-preview-app></md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**x**</md-td>
      <md-td>**x**</md-td>
      <md-td>**x**</md-td>
      <md-td>/</md-td>
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
                keys
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                本地数据缓存中的所有键名列表，如果没有本地数据则返回空数组
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                currentSize
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                占用空间大小，以 `KB` 为单位
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                limitSize
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                存储空间上限，以 `KB` 为单位，一般来说会返回 `10240`
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path=" /page/API/pages/storage/storage" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.getStorageInfo({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getStorageInfo fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "limitSize": 10240,
    "currentSize": 0.0419921875,
    "keys": [
        "name"
    ],
    "errMsg": "getStorageInfo:ok"
}
```

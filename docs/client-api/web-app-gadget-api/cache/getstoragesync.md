---
document_id: '6965379541104263173'
directory_id: '6907567266536308737'
title: getStorageSync
full_path: /uYjL24iN/uATOx4CM5EjLwkTM
breadcrumb:
- Client API
- Web app/Gadget API
- Cache
- getStorageSync
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:33Z
source_url: https://open.larksuite.com/document/uYjL24iN/uATOx4CM5EjLwkTM
---

# getStorageSync(string key)

获取本地缓存数据。


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
      <md-td><md-preview-app type="vh" disable="true" fontSize="14">预览</md-preview-app></md-td>
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
                key
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td></md-td>
            <md-td>
                键名

**示例值**：stateKey

**最小长度**：`1`  字符
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 输出
返回值：
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
                string|object|number|boolean|object[]|string[]|number[]|boolean[]|undefined|null
            </md-td>
            <md-td>
                键名对应的数据

            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 示例代码

```js
try {
    let result = tt.getStorageSync("name");
    console.log(`getStorageSync success: ${JSON.stringify(result)}`);
} catch (error) {
    console.log(`getStorageSync fail: ${JSON.stringify(error)}`);
}
```

返回值示例：
```json
"小王"
```

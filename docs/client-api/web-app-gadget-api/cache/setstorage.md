---
document_id: '6965379543684661254'
directory_id: '6907567266536308737'
title: setStorage
full_path: /uYjL24iN/uETOx4SM5EjLxkTM
breadcrumb:
- Client API
- Web app/Gadget API
- Cache
- setStorage
document_type: GuideDocumentType
updated_at: 2022-03-11T04:20:24Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOx4SM5EjLxkTM
---

# setStorage(Object object)

以「键值对」的形式设置本地缓存数据。

:::html
<md-alert type="tip">
注意事项：
- 单个 key 允许存储的最大数据长度为 **1MB**，所有数据存储上限为 **10MB**，同时也受到用户设备存储空间、缓存清理等机制的限制，可能会导致信息丢失，因此请不要将重要数据存放在本地数据缓存。
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

**示例值**：name

**最小长度**：`1`  字符
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                data
            </md-td>
            <md-td>
                string|object|number|boolean|object[]|string[]|number[]|boolean[]|undefined|null
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                undefined
            </md-td>
            <md-td>
                键名对应的数据

**示例值**：小王
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码

```js
tt.setStorage({
    key: "name",
    data: "小王",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`setStorage fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "setStorage:ok"
}
```

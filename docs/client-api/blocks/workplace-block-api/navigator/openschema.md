---
document_id: '7180269945547177989'
directory_id: '7180165099251056645'
title: openSchema
full_path: /uAjLw4CM/uYjL24iN/block/api/navigator/openschema
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Navigator
- openSchema
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/navigator/openschema
---

# openSchema

在新窗口打开网页、文档、小程序等。


::: note
需要先[配置白名单](/document/uAjLw4CM/uYjL24iN/block/guide/open-ability/openschema)。如果出现无法跳转的现象，请先检查 schema 是否在白名单里。
:::

## 输入

param 继承自[标准对象输入](/document/uAjLw4CM/uYjL24iN/block/api/standard-object-input)，扩展属性描述：

| **名称** | **数据类型** | **是否必填** | **默认值** | **描述**                                                                                                        |
| ------ | ------ | ------- | ------ | ------------------------------------------------------------------------------------------------------------- | ---------- |
| schema | string | 是      | -      | 指定应用的 schema，schema 需要满足 URI 协议，可以使用 [AppLink 协议](/document/uYjL24iN/ucjN1UjL3YTN14yN2UTN)|
| external | boolean | 否      | true      | 是否在外部浏览器打开页面，如果为false，则页面将会在Lark独立窗口内打开，**仅PC支持**|
## 输出

各 callback 返回对象参数均无额外扩展属性。

## 示例代码

### 调用示例
:::html
<md-block-api>
{
  "sourceData":{
          "tab": "api",
          "item": "openSchema"
  },
    "openDetail": 1, 
    "title": "openSchema", 
    "list_page_url": "https://applink.larksuite.com/client/block/workplace/open?appId=cli_a00834ec56f8d01b%26blockTypeId=blk_610a40455f800004c32b6bb6%26sourceData=%7B%22tab%22%3A%22api%22%2C%22item%22%3A%22login%22%7D", 
    "min_lk_ver": {
        "pc": "5.10.0", 
        "mobile": "5.11.0"
    },
    "blockEntity": {
        "sourceData": {
            "type": "api",
            "item": "openSchema",
            "isNew": true
        },
                  "blockID": "mock-block"
    }
}
</md-block-api>
:::
```js
tt.openSchema({
  schema: 'https://larksuite.com/',
  success (res) {
    console.log('openSchema 调用成功', res.errMsg);
  },
  fail (res) {
    console.log('openSchema 调用失败', res.errMsg);
  },
  complete (res) {
    console.log('openSchema 调用结束', res.errMsg);
  }
});
```

### 返回示例

```json
{
  "errMsg": "openSchema:ok"
}
```

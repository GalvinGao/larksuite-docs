---
document_id: '7180270043522842630'
directory_id: '7180165099251007493'
title: removeStorage
full_path: /uAjLw4CM/uYjL24iN/block/api/data-cache/removestorage
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Data Cache
- removeStorage
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:25Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/data-cache/removestorage
---

# removeStorage

删除本地缓存数据。

## 输入

param 继承自[标准对象输入](/document/uAjLw4CM/uYjL24iN/block/api/standard-object-input)，扩展属性描述：

| **名称** | **数据类型** | **是否必填** | **默认值** | **描述** |
| ------ | ------ |-------- | ------ | ------ | ---------- |
| key    | string | 是      | -      | 键名     | PC 端 1.0.4 |

## 输出

各 callback 的参数均无额外属性。

## 示例代码

### 调用示例
:::html
<md-block-api>
{
  "sourceData":{
          "tab": "api",
          "item": "removeStorage"
  },
    "openDetail": 1, 
    "title": "removeStorage", 
    "list_page_url": "https://applink.larksuite.com/client/block/workplace/open?appId=cli_a00834ec56f8d01b%26blockTypeId=blk_610a40455f800004c32b6bb6%26sourceData=%7B%22tab%22%3A%22api%22%2C%22item%22%3A%22login%22%7D", 
    "min_lk_ver": {
        "pc": "5.10.0", 
        "mobile": "5.11.0"
    },
    "blockEntity": {
        "sourceData": {
            "type": "api",
            "item": "removeStorage",
            "isNew": true
        },
                  "blockID": "mock-block"
    }
}
</md-block-api>
:::
```js
tt.removeStorage({
  key: '测试',
  success (res) {
    console.log('removeStorage 调用成功', res.errMsg);
  },
  fail (res) {
    console.log('removeStorage 调用失败', res.errMsg);
  },
  complete (res) {
    console.log('removeStorage 调用结束', res.errMsg);
  }
});
```

### 返回示例

```json
{
  "errMsg": "removeStorage:ok"
}
```

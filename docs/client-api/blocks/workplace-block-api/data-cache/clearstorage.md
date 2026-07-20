---
document_id: '7180270043522531334'
directory_id: '7180165099251007493'
title: clearStorage
full_path: /uAjLw4CM/uYjL24iN/block/api/data-cache/clearstorage
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Data Cache
- clearStorage
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:25Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/data-cache/clearstorage
---

# clearStorage

清理**全部**本地缓存数据。


## 输入

param 继承自[标准对象输入](/document/uAjLw4CM/uYjL24iN/block/api/standard-object-input)，无额外扩展属性。

## 输出

各 callback 的参数均无额外属性。

## 示例代码

### 调用示例
:::html
<md-block-api>
{
  "sourceData":{
          "tab": "api",
          "item": "clearStorage"
  },
    "openDetail": 1, 
    "title": "clearStorage", 
    "list_page_url": "https://applink.larksuite.com/client/block/workplace/open?appId=cli_a00834ec56f8d01b%26blockTypeId=blk_610a40455f800004c32b6bb6%26sourceData=%7B%22tab%22%3A%22api%22%2C%22item%22%3A%22login%22%7D", 
    "min_lk_ver": {
        "pc": "5.10.0", 
        "mobile": "5.11.0"
    },
    "blockEntity": {
        "sourceData": {
            "type": "api",
            "item": "clearStorage",
            "isNew": true
        },
                  "blockID": "mock-block"
    }
}
</md-block-api>
:::
```js
tt.clearStorage({
  success (res) {
    console.log('clearStorage 调用成功', res.errMsg);
  },
  fail (res) {
    console.log('clearStorage 调用失败', res.errMsg);
  },
  complete (res) {
    console.log('clearStorage 调用结束', res.errMsg);
  }
});
```

### 返回示例

```json
{
  "errMsg": "clearStorage:ok"
}
```

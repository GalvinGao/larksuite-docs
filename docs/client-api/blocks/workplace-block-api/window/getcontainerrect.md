---
document_id: '7180269945547767813'
directory_id: '7180165099250909189'
title: getContainerRect
full_path: /uAjLw4CM/uYjL24iN/block/api/window/getcontainerrect
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Window
- getContainerRect
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:25Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/window/getcontainerrect
---

# getContainerRect

获取容器尺寸。

## 输入
param 继承自[标准对象输入](/document/uAjLw4CM/uYjL24iN/block/api/standard-object-input)，无额外扩展属性。

## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| width | number | 宽度，单位 px |
| height | number | 高度，单位 px |



## 示例代码
### 调用示例
:::html
<md-block-api>
{
  "sourceData":{
          "tab": "api",
          "item": "getContainerRect"
  },
    "openDetail": 1, 
    "title": "getContainerRect", 
    "list_page_url": "https://applink.larksuite.com/client/block/workplace/open?appId=cli_a00834ec56f8d01b%26blockTypeId=blk_610a40455f800004c32b6bb6%26sourceData=%7B%22tab%22%3A%22api%22%2C%22item%22%3A%22login%22%7D", 
    "min_lk_ver": {
        "pc": "5.10.0", 
        "mobile": "5.11.0"
    },
    "blockEntity": {
        "sourceData": {
            "type": "api",
            "item": "getContainerRect",
            "isNew": true
        },
                  "blockID": "mock-block"
    }
}
</md-block-api>
:::
```js
tt.getContainerRect({
  success (res) {
    console.log('getContainerRect 调用成功', res);
  },
  fail (res) {
    console.log('getContainerRect 调用失败', res.errMsg);
  },
  complete (res) {
    console.log('getContainerRect 调用结束', res.errMsg);
  }
});
```

### 返回示例
```json
{
  "errMsg": "getContainerRect:ok",
  "width": 0,
  "height": 0
}
```

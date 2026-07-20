---
document_id: '7180269945548718085'
directory_id: '7180165099250909189'
title: onContainerResize
full_path: /uAjLw4CM/uYjL24iN/block/api/window/oncontainerresize
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Window
- onContainerResize
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:25Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/window/oncontainerresize
---

# onContainerResize

监听容器尺寸变化。

## 输入

| **属性**   | **类型** |**必填**| **说明**              |
| -------- | ------ | ------------------- | -------- |
| callback    | function | 是| 当容器尺寸变化时的回调函数  |          

## 输出
回调函数返回对象的属性：
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
                width
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                宽度，单位 px
            </md-td>
        </md-tr>
      <md-tr>
            <md-td>
                height
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                高度，单位 px
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 示例代码
### 调用示例
:::html
<md-block-api>
{
  "sourceData":{
          "tab": "api",
          "item": "onContainerResize"
  },
    "openDetail": 1, 
    "title": "onContainerResize", 
    "list_page_url": "https://applink.larksuite.com/client/block/workplace/open?appId=cli_a00834ec56f8d01b%26blockTypeId=blk_608288f1f7c000146b4eabc1%26sourceData=%7B%22tab%22%3A%22api%22%2C%22item%22%3A%22login%22%7D", 
    "min_lk_ver": {
        "pc": "5.10.0", 
        "mobile": "5.11.0"
    },
    "blockEntity": {
        "sourceData": {
            "type": "api",
            "item": "onContainerResize",
            "isNew": true
        },
                  "blockID": "mock-block"
    }
}
</md-block-api>
:::
```js
tt.onContainerResize(({ width, height}) => {
  console.log('容器尺寸变化 width = ', width, ' height = ', height);
});
```

### 回调函数返回对象示例
```json
{
  "width": 545,
  "height": 50
}
```



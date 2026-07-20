---
document_id: '7180269945547571205'
directory_id: '7180165099250909189'
title: offContainerResize
full_path: /uAjLw4CM/uYjL24iN/block/api/window/offcontainerresize
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Window
- offContainerResize
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:25Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/window/offcontainerresize
---

# offContainerResize

取消监听容器尺寸变化。
:::html
<md-alert type="tip">

为防止多次注册事件监听导致一次事件多次回调，建议每次调用 onContainerResize 方法监听事件之前，先调用 offContainerResize 方法，关闭之前的事件监听。
</md-alert>
:::


## 输入
| **名称**   | **数据类型** |**是否必填**|**默认值**| **描述**              |
| -------- | ------ | ------------------- | -------- |----|
| callback    | function | 是|-- |当容器尺寸变化监听取消时的回调函数  |          



## 示例代码
```js
const fn = (res) => {
  console.log('取消监听');
};

tt.offContainerResize(fn);
```

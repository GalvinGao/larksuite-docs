---
document_id: '7270779605451177990'
directory_id: '7270719284443594757'
title: Bridge.getContainerRect
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Bridge.getContainerRect
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Bridge
- Bridge.getContainerRect
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Bridge.getContainerRect
---

# Bridge.getContainerRect
获取宿主容器的 Rect 信息，该方法为异步调用。
  
## 可用性说明

| 权限要求 | 视图可用说明 | 平台可用 | 场景 |
| --- | --- | --- | --- |
| 可读 | 所有视图 | - PC<br>- 移动端 | 演示模式 |



## 输入

无需传入参数。
  

## 输出

异步返回宿主容器的 Rect 信息
| **名称**        | **数据类型**      | **是否必填** | **描述**            |
| ------------- | ------------- | -------- | ----------------- |
| containerRect | ContainerRect | 是        | 容器的矩形信息（相对于文档左上角） |
|  ∟x           | number        | 是        | 矩形的 x 坐标          |
|  ∟y           | number        | 是        | 矩形的 y 坐标          |
|  ∟width       | number        | 是        | 矩形的宽度             |
|  ∟height      | number        | 是        | 矩形的高度             |
  

## 示例代码

### 调用示例

```js
const rect = await DocMiniApp.Bridge.getContainerRect();
console.log('debug', rect);
```

### 返回示例

```json
{
    "x": 159.5,
    "y": 191.25,
    "width": 816,
    "height": 100
}
```

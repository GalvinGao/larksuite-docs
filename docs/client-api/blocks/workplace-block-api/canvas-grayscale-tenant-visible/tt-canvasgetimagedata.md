---
document_id: '7180269945544556550'
directory_id: '7180165099251040261'
title: canvasGetImageData
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/tt-canvasgetimagedata
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- tt.canvasGetImageData
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/tt-canvasgetimagedata
---

# canvasGetImageData

## tt.canvasGetImageData(Object param)

获取 Canvas 画布上指定矩形区域的像素数据。

## 输入

param 继承自[标准对象输入](/document/uAjLw4CM/uYjL24iN/block/api/standard-object-input)，扩展属性描述：

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
| canvasId | string  | -      | 是       | 画布标识，传入 canvas 组件的 canvas-id 属性|1.6.0
| x | number  | -      | 是       | 矩形区域左上角的横坐标|1.6.0
| y | number  | -      | 是       | 矩形区域左上角的纵坐标|1.6.0
| width | number  | -      | 是       | 矩形区域的宽度|1.6.0
| height | number  | -      | 是       | 矩形区域的高度|1.6.0

## 输出

success 函数返回对象参数：

| **属性**      | **类型** | **说明**                                                     |
| ------------- | -------- | ------------------------------------------------------------ |
| width      | number   | 图像数据矩形的宽度 |
| height      | number   | 图像数据矩形的高度 |
| data      | Uint8ClampedArray   | 图像像素点数据，一维数组，每四项表示一个像素点的 rgba |

## 示例

### 代码示例

```js
tt.canvasGetImageData({
  canvasId,
  x: 0,
  y: 0,
  width: 1,
  height: 1,
  success(res) {
    console.log('CanvasGetImageData Response Width:', res.width); // 1
    console.log('CanvasGetImageData Response Height:', res.height); // 1
    console.log('CanvasGetImageData Response data test:', res.data instanceof Uint8ClampedArray); // true
    console.log('CanvasGetImageData Response data length:', res.data.length); // 1 * 1 * 4
},
  fail(err) {
    console.log('Error', err);
  },
  complete() {
    console.log('GetImageData Complete'); 
  } 
});
```

### **success** **函数返回对象示例**

```json
{
  "width": 1,
  "height": 1,
  "data": [0, 0, 0, 0]
}
```

## Tip

暂无。

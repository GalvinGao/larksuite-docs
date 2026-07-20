---
document_id: '7180269945548603397'
directory_id: '7180165099251040261'
title: canvasPutImageData
full_path: /uAjLw4CM/uYjL24iN/block/api/canvas/tt-canvasputimagedata
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- Canvas（Grayscale Tenant Visible）
- tt.canvasPutImageData
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/api/canvas/tt-canvasputimagedata
---

# canvasPutImageData

## tt.canvasPutImageData(Object param)

将像素数据绘制到画布上。此方法不受画布转换矩阵的影响。

## 输入

param 继承自[标准对象输入](/document/uAjLw4CM/uYjL24iN/block/api/standard-object-input)，扩展属性描述：

|**属性**|**类型**|**默认值**|**必填**|**说明**|**最低版本**
|:-|:-|:-|:-|:-|:-|
| canvasId | string  | -      | 是       | 画布标识，传入 canvas 组件的 canvas-id 属性|1.6.0
| data | Uint8ClampedArray  | -      | 是       | 图像像素点数据，一维数组，每四项表示一个像素点的 rgba|1.6.0
| x | number  | -      | 是       | 源图像数据在目标画布中的位置偏移量（x 轴方向的偏移量）|1.6.0
| y | number  | -      | 是       | 源图像数据在目标画布中的位置偏移量（y 轴方向的偏移量）|1.6.0
| width | number  | -      | 是       | 源图像数据矩形区域的宽度|1.6.0
| height | number  | -      | 是       | 源图像数据矩形区域的宽度|1.6.0

## 输出

无额外扩展对象属性。

## 示例

### 代码示例

```js
const data = new Uint8ClampedArray(40 * 40 * 4);
  
for (let i = 0; i < data.length; i += 4) {
  data[i] = 0;
  data[i + 1] = 0;
  data[i + 2] = 0;
  data[i + 3] = 255;
}

tt.canvasPutImageData({
  canvasId: 'canvas',
  x: 80,
  y: 80,
  width: 40,
  height: 40,
  data,
  success (res) {
    console.log('success:', res);
  }
});
```

### **success** **函数返回对象示例**

```json
{
  "errMsg": "putImageData:ok"
}
```

### 示例结果


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/60495e9f9a72f2db7b98452763cafa25_WDETUkXKgS.png)

## Tip

暂无。

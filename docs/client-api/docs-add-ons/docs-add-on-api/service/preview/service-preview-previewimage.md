---
document_id: '7270779605450391558'
directory_id: '7270719284443414533'
title: Service.Preview.previewImage
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Preview.previewImage
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Service
- Preview
- Service.Preview.previewImage
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Preview.previewImage
---

# Service.Preview.previewImage
传入图片信息，唤起图片查看器，该方法为异步调用。
  
## 可用性说明
:::html
<md-table>
<md-thead>
<md-tr>
<md-th>权限要求</md-th>
<md-th>视图可用说明</md-th>
<md-th>平台可用</md-th>
<md-th>场景</md-th></md-tr>
</md-thead>
<md-tbody>
<md-tr>
<md-td>可读</md-td>
<md-td>所有视图</md-td>
<md-td>- PC
- 移动端（6.0后可用）</md-td>
<md-td>演示模式</md-td>
</md-tr></md-tbody>
</md-table>
:::


## 输入

| **名称**      | **数据类型**            | **是否必填** | **描述**      |
| ----------- | ------------------- | -------- | ----------- |
| imageInfo   | PreivewImageInfo    | 是        | /           |
|  ∟imageList | PreviewImageItem[] | 是        | 图片数组        |
|   ∟src      | string              | 是        | 图片的地址       |
|   ∟width    | number              | 是        | 图片宽度        |
|   ∟height   | number              | 是        | 图片高度        |
|   ∟scale    | number              | 否        | 图片缩放值       |
|  ∟index     | index               | 否        | 图片数组的预览起始索引 |
  

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
DocMiniApp.Service.Preview.previewImage({
    imageList: [
        {
            src: '',
            width: 100,
            height: 100
        }
    ]
});
```

### 返回示例

无

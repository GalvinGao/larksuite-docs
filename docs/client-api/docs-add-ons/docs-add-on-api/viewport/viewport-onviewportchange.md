---
document_id: '7270779605450178566'
directory_id: '7270719284443627525'
title: Viewport.onViewportChange
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Viewport.onViewportChange
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Viewport
- Viewport.onViewportChange
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Viewport.onViewportChange
---

# Viewport.onViewportChange
监听文档视口变化，该方法为异步调用。
  
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
- 移动端</md-td>
<md-td>演示模式</md-td>
</md-tr></md-tbody>
</md-table>
:::


## 输入

| **名称**  | **数据类型**              | **是否必填** | **描述**  |
| ------- | --------------------- | -------- | ------- |
| handler | ViewportChangeHandler | 是        | 视口变化监听器 |

### ViewportChangeHandler

```js
type ViewportChangeHandler = (event: ViewportChangeEvent) => void;
```

### ViewportChangeEvent

| **名称**           | **数据类型**                                                                   | **是否必填** | **描述**         |
| ---------------- | -------------------------------------------------------------------------- | -------- | -------------- |
| viewportBlockIds | [BlockId](/document/uAjLw4CM/uYjL24iN/docs-add-on/06-data-structure/blockid)[] | 是        | 当前视口中 Block Id |

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const viewportChangeHandler = (event: ViewportChangeEvent) => {
    console.log('debug', event);
};
useEffect(() => {
    DocMiniApp.Viewport.onViewportChange(viewportChangeHandler);
    return () => {
        DocMiniApp.Viewport.offViewportChange(viewportChangeHandler);
    };
});
```

### 返回示例

无

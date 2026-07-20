---
document_id: '7270779605451390982'
directory_id: '7270719284443316229'
title: Service.Hyperlink.open
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Hyperlink.open
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Service
- Hyperlink
- Service.Hyperlink.open
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Hyperlink.open
---

# Service.Hyperlink.open
传入url，打开对应链接，该方法为异步调用。
  
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

| **名称** | **数据类型** | **是否必填** | **描述** |
| ------ | -------- | -------- | ------ |
| url    | string   | 是        | 链接     |
  

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
DocMiniApp.Service.Hyperlink.open('baidu.com');
```

### 返回示例

无

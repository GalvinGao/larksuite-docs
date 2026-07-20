---
document_id: '7270779605447196678'
directory_id: '7270719284443578373'
title: Service.History.getMinorVersions
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.History.getMinorversions
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Service
- History
- Service.History.getMinorversions
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.History.getMinorversions
---

# Service.History.getMinorVersions
获取历史记录的小版本列表，该方法为异步调用。
  
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
<md-td>可写</md-td>
<md-td>所有视图</md-td>
<md-td>- PC
- 移动端</md-td>
<md-td>演示模式</md-td>
</md-tr></md-tbody>
</md-table>
:::


## 输入

无需传入参数。
  

## 输出

Promise<any[]>
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const minorVersions = await DocMiniApp.Service.History.getMinorVersions();
console.log('debug', minorVersions);
```

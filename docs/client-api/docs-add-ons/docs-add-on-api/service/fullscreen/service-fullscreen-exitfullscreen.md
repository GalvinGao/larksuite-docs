---
document_id: '7270779700748959749'
directory_id: '7270719284443529221'
title: Service.Fullscreen.exitFullscreen
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Fullscreen.exitFullscreen
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Service
- Fullscreen
- Service.Fullscreen.exitFullscreen
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.Fullscreen.exitFullscreen
---

# Service.Fullscreen.exitFullscreen
使小应用退出全屏状态，该方法为异步调用。
  
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
<md-td>全屏视图</md-td>
<md-td>PC</md-td>
<md-td>\-</md-td>  
</md-tr></md-tbody>
</md-table>
:::


## 输入

无需传入参数。
  

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
DocMiniApp.Service.Fullscreen.exitFullscreen();
```

### 返回示例

无

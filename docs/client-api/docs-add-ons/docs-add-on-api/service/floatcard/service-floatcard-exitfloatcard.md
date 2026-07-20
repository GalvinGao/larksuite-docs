---
document_id: '7270779605450358790'
directory_id: '7270719284443512837'
title: Service.FloatCard.exitFloatCard
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.FloatCard.exitFloatCard
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Service
- FloatCard
- Service.FloatCard.exitFloatCard
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.FloatCard.exitFloatCard
---

# Service.FloatCard.exitFloatCard
使小应用退出悬浮窗口状态

:::html
<md-alert type="warn">注意这个接口还处于内部测试阶段，最终是否开放还未确定，请谨慎使用</md-alert>
:::
  
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
<md-td>悬浮卡片视图</md-td>
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
DocMiniApp.Service.FloatCard.exitFloatCard();
```

### 返回示例

无

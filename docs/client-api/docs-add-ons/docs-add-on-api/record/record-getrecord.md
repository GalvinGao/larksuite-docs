---
document_id: '7270779605450342406'
directory_id: '7270719284443463685'
title: Record.getRecord
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Record.getRecord
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Record
- Record.getRecord
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Record.getRecord
---

# Record.getRecord
获取当前小应用的 Record 数据，该方法为异步调用。
  
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
<md-td>- 正文小组件
- 全屏视图
- 模态框视图
- 弹窗视图</md-td>
<md-td>- PC
- 移动端</md-td>
<md-td>演示模式</md-td>
</md-tr></md-tbody>
</md-table>
:::


## 输入

无需传入参数。
  

## 输出

当前云文档小组件的 Record 数据
  

## 示例代码

### 调用示例

```js
 DocMiniApp.Record.getRecord()
     .then((recordData: any) => {
         //对数据做处理
         console.log('debug',recordData);
     })
```

### 返回示例

```json
{ recordData:'xxxxx' }
```

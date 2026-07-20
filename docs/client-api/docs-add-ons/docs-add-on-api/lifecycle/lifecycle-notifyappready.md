---
document_id: '7270779605451456518'
directory_id: '7270719284443332613'
title: LifeCycle.notifyAppReady
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/LifeCycle.notifyAppReady
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- LifeCycle
- LifeCycle.notifyAppReady
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/LifeCycle.notifyAppReady
---

# LifeCycle.notifyAppReady
通知文档应用加载完毕，配合配 `useHostLoading`使用，该方法为异步调用。
```json
// app.json
{
    "contributes": {
    "addPanel": {
        "useHostLoading": true, // true 的时候notifyAppReady才生效
        ...
    }
  }
}
```
  
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

无需传入参数。
  

## 输出

无
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
useEffect(() => {
    DocMiniApp.LifeCycle.notifyAppReady();
});
```

### 返回示例

无

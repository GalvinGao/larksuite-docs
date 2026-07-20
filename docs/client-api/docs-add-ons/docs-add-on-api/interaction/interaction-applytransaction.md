---
document_id: '7270779605446950918'
directory_id: '7270719284443299845'
title: Interaction.applyTransaction
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Interaction.applyTransaction
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Interaction
- Interaction.applyTransaction
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Interaction.applyTransaction
---

# Interaction.applyTransaction
获取当前小应用的 Interaction 数据, 作为一个事务批量operations，该方法为异步调用。
  
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

获取当前小应用的 Interaction 数据, 作为一个事务批量operations
| **参数**     | **类型**                                                                                  | **必选** | **释义**           |
| ---------- | --------------------------------------------------------------------------------------- | ------ | ---------------- |
| operations | [InteractionOperation](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/InteractionOperation)[] | 是      | Interaction 变更操作 |
  

## 输出

异步返回一个InteractionData 对象
  

## 示例代码

### 调用示例

```js
const [data, updateData] = useState<any>();
useEffect(() => {
  const handler = (data = {}) => updateData(data);
  DocMiniApp.Interaction.getData().then(handler);
  DocMiniApp.Interaction.onDataChange(handler);
  return () => {
    DocMiniApp.Interaction.offDataChange(handler);
  };
}, []);
  
const handleAddItem = useCallback(async () => {
  const result = await DocMiniApp.Interaction.applyTransaction([{
    p: ['test'],
    action: {
      oi: 2
    }
  }]);
  console.log('debug', result);
}, []);
return (
  <div>
    <Button onClick={handleAddItem}>AddItem</Button>
  </div>
)
```

### 返回示例

```json
{"test":2}
```

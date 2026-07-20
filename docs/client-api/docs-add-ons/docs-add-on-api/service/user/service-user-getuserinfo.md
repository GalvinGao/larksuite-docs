---
document_id: '7270779605450735622'
directory_id: '7270719284443398149'
title: Service.User.getUserInfo
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.User.getUserinfo
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Service
- User
- Service.User.getUserinfo
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:15Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.User.getUserinfo
---

# Service.User.getUserInfo
获取当前用户的信息，该方法为异步调用。
  
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

异步返回一个 [UserInfo](/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/UserInfo)
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
DocMiniApp.Service.User.getUserInfo()
  .then((userInfo) => {
    console.log('debug', userInfo);
  })
```

### 返回示例

```json
{
    "nickName": "用户昵称",
    "avatarUrl": "用户头像",
    "gender": "male",
    "country": "CN",
    "city": "深圳",
    "language": "zh_CN",
    "tenantId": "1"
}
```

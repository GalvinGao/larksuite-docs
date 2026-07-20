---
document_id: '7270779605451554822'
directory_id: '7270719284443398149'
title: Service.User.login
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.User.login
breadcrumb:
- Client API
- Docs Add-ons
- Docs Add-on API
- Service
- User
- Service.User.login
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:19Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/05-api-doc/basic-data-reference---base/Service.User.login
---

# Service.User.login
登录并获取当前用户的 code，使用该 code， 业务后台可以向Lark后台请求获取该用户的身份，从而实现用户登录流程。
参考流程如下：
1. 在文档小组件内调用 DocMiniApp.Service.User.login，获取 code，并传递给业务后台
```js
const code = await DocMiniApp.Service.User.login();
await axios.post(`https://your_backend/login?code=${code}`)
```
2. 参考 [code2session - 客户端文档 - 开发文档 - Lark开放平台](/document/uYjL24iN/ukjM04SOyQjL5IDN)， 业务后台使用 code 获取用户信息
2. 根据用户信息，返回给云文档小组件该用户对应的 session
2. 文档小组件将 session 进行持久化，后续请求使用该 session，便可以正确识别用户身份。
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4b44a5d5a04f0fd9ed476a45f3e244c1_ZQok76130m.png?lazyload=true&width=1157&height=1280)
  

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

异步返回当前用户的 code
  

## 示例代码

### 调用示例

```js
const DocMiniApp = new BlockitClient().initAPI();
const code = await DocMiniApp.Service.User.login();
console.log('debug', code);
```

### 返回示例

```
'user code'
```
  

## 开放平台集成代码示例

```js
// 需要保证文档小组件的appid和后端服务的appid是一致的。
1. 获取应用access_token
curl -i -X POST 'https://open.larksuite.com/open-apis/auth/v3/app_access_token/internal' \
-H 'Content-Type: application/json' \
-d '{
        "app_id": "xxxxxxxxxxxxxxxx",
        "app_secret": "xxxxxxxxxxxxxxxxxxxxxx"
}'
{
    "app_access_token": "t-gsssdsdsdsdsdsdsdsWOAA5XFW44CD3BMONERSZ76",
    "code": 0,
    "expire": 6119,
    "msg": "ok",
    "tenant_access_token": "t-gsdsdsdsdgsdgfdsgsdXFW44CD3BMONERSZ76"
}
2. 前端传入code，获取 user_access_token
curl -i -X GET 'https://open.larksuite.com/open-apis/mina/v2/tokenLoginValidate' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer t-gsssdsdsdsdsdsdsdsWOAA5XFW44CD3BMONERSZ76' \
-d '{
        "code": "65755f6e63-3bfb257c19311ae9"
}'
{
    "code": 0,
    "data": {
        "access_token": "u-3yDIsssssssssssssssssssssssssssssssss",
        "employee_id": "5b3d15e1",
        "expires_in": 1679407800,
        "open_id": "ssssssssssssssssssssssssss",
        "refresh_token": "ur-2Y127IT693QUbEVaz6p3gV1k6_ok501zU200lgsw2xHF",
        "session_key": "cf1994859671244789de90f52977a25d",
        "tenant_key": "736588c9260f175d",
        "union_id": "ou_3ssssssssssssssssss,
        "msg": "success"
}
3.根据user_access_token获取登录用户的信息
curl -i -X GET 'https://open.larksuite.com/open-apis/authen/v1/user_info' \
-H 'Authorization: Bearer u-3yDIsssssssssssssssssssssssssssssssss'
{
    "code": 0,
    "data": {
        "avatar_big": "",
        "avatar_middle": "",
        "avatar_thumb": "",
        "avatar_url": "",
        "en_name": "xxxxxxxxxxxx",
        "name": "xxxxxxxxx",
        "open_id": "xxxxxxxxxxxxxxxx",
        "tenant_key": "xxxxxxxxxxxxxxxx",
        "union_id": "xxxxxxxxxxxxxxxx",
        "user_id": "xxxxxxxxxxxxxxxx"2
    },
    "msg": "success"
}
```

---
document_id: '7320454711965351941'
directory_id: '7258197168736534534'
title: 常见问题
full_path: /uAjLw4CM/uYjL24iN/base-extensions/faq
breadcrumb:
- Developer Guides
- Develop Base Extensions
- FAQ
document_type: GuideDocumentType
updated_at: 2024-01-16T13:15:10Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/faq
---

# 常见问题

### Q：如何完成用户登录授权流程（以及获取用户信息、调用 OpenAPI ）

对于 [app_access_token](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/app_access_token) 和 [tenant_access_token](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token) 的获取，由于是通过服务端处理，其获取方式与常规流程并无差异。
该方式仅对视图插件有效。


#### 使用`requestAccess`方法
使用`requestAccess`方法需要将 `@lark-opdev/block-open-api` 更新至 0.2.0 及以上版本
`user_access_token` 的获取可参考[登录流程](/document/uYjL24iN/uETO5QjLxkTO04SM5kDN)。
我们在插件页面中：
1. 引入 `@lark-opdev/block-open-api` 包：


`npm install @lark-opdev/block-open-api`；

2. 导入并使用 `requestAccess` 方法获得 `code` ：
```js
import { requestAccess } from "@lark-opdev/block-open-api";
const {code} = await requestAccess({
	scopes: ['bitable:app','bitable:app:readonly'],
	state: undefined,
});
```
它将唤起一个授权窗口提示用户是否授权，授权成功后将返回code。
    
`requestAccess`参数说明
:::html
<table>
  	<thead>
       <tr>
         <th>字段名</th>
         <th>类型</th>
         <th>描述</th>
         <th>是否必填</th>
       </tr>
  	</thead>
  	<tbody>
      <tr>
        <td>scopes</td>
        <td>String[]</td>
        <td>需要授权的 Scope List，详见下图scopes</td>
        <td>是</td>
      </tr>
      <tr>
        <td>state</td>
        <td>String</td>
        <td>用来维护请求和回调状态的附加字符串， 在授权完成回调时会附加此参数，应用可以根据此字符串来判断上下文关系。</td>
        <td>否</td>
      </tr>
  	</tbody>
<table>
:::

- scopes参数说明

必填，值为开放平台权限（并且该应用已开通的权限）名称字符串数组

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/790ac93694fd73b2b20285d274d6a27a_2p46ip1V8z.png?height=983&lazyload=true&width=2417)

`requestAccess`返回值说明
:::html
<table>
    <thead>
      <tr>
        <th>字段名</th>
        <th>类型</th>
        <th>描述</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <td>code</td>
        <td>String</td>
        <td>临时登录凭证，有效期 3 分钟，只能使用一次</td>
      </tr>
      <tr>
        <td>state</td>
        <td>String</td>
        <td>用来维护请求和回调状态的附加字符串， 在授权完成回调时会附加此参数，应用可以根据此字符串来判断上下文关系。</td>
      </tr>
    </tbody>
<table>
:::

3. 获得`临时授权码`（该`code`三分钟有效，并且只可使用一次）之后，通过服务端调用[获取 user_access_token](/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/user_info/get)换取 `user_access_token`。
4. 接下来你可以用刚换得的 `user_access_token` 调用[获取用户信息](/document/common-capabilities/sso/api/get-user-info)接口。

#### 使用`login`方法
使用 login 方法需将 `@lark-opdev/block-open-api` 更新至 `0.1.5` 及以上版本；

  `user_access_token` 的获取可参考[登录流程](/document/uYjL24iN/uETO5QjLxkTO04SM5kDN)。
  我们在插件页面中：
  1. 引入 `@lark-opdev/block-open-api` 包：

       ` npm install @lark-opdev/block-open-api`
  2. 导入并使用 `login` 方法获得 `code` ：
  ```
  import { login } from "@lark-opdev/block-open-api";
  const loginRes = await login();
  const code = loginRes.code;
  ```
  3. 获得`临时授权码`（该`code`三分钟有效）之后，通过服务端调用 [code2session 接口](/document/uYjL24iN/ukjM04SOyQjL5IDN)换取 `user_access_token`。
  4. 接下来你可以用刚换得的 `user_access_token` 调用[获取用户信息](/document/common-capabilities/sso/api/get-user-info)接口。




### Q：如何添加前端 router

A: 不推荐使用基于 path 的 router，暂时多维表格的静态文件服务不提供 rewrite 能力，
推荐使用 hash router [React HashRouter](https://reactrouter.com/en/main/router-components/hash-router) 。
  

### Q：更新依赖后 VSCode 的类型提示不正常

A: 尝试重启 VSCode，或者删除 `node_modules` 目录后重新安装依赖。
  

### Q：如何修改启动调试时自动打开的文档

A: 修改插件目录下的 `block.json` 中的 `url` 属性。
  

### Q：如何修改本地调试时的小应用名称

A: 注意，线上小应用名称需要在开放平台开发者后台设置。
修改插件目录下的 `block.json` 中的 projectName 属性。
  

### Q：视图插件的页面托管域名是？

A: 在Lark品牌下页面托管域名如下：
:::html
<table cellspacing="0" cellpadding="14px" >
  <thead>
    <th>品牌</th>
    <th>插件域名</th>
    <th>插件宿主域名</th>
  </thead>
  <tr style="border-bottom: 1px solid #dee0e3;">
    <td>Lark</td>
    <td>https://*.larkpkg.com</td>
    <td>https://*.lark.com</td>
  </tr>
</table>
:::

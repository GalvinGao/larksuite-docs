---
document_id: '7407265523660570629'
directory_id: '6907567266537635841'
title: applyTenantAppScope
full_path: /uYjL24iN/uczMx4yNzEjL3MTM/applytenantappscope
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Authorize
- applyTenantAppScope
document_type: GuideDocumentType
updated_at: 2024-08-26T02:56:22Z
source_url: https://open.larksuite.com/document/uYjL24iN/uczMx4yNzEjL3MTM/applytenantappscope
---

# applyTenantAppScope(Object object)

弹窗咨询用户是否向租户管理员申请所有未授予权限(不包括租户敏感权限)。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.43.0+</md-version> | <md-version>V3.43.0+</md-version> | <md-version>V3.43.0+</md-version> | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性

## 输出
`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| data | object | 权限申请结果 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>status<br></md-text> | number | 状态码<br>**可选值**：<br>- `1`：用户已申请权限<br>- `2`：未申请权限(包括申请被拒绝)<br>- `3`：权限申请中<br>- `4`：无可申请列表<br>- `5`：相同授权超过数量限制<br>- `6`：仅租户敏感权限未授权 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>msg<br></md-text> | string | 结果信息<br>**可选值**：<br>- `user agrees to apply`：对应status1<br>- `user cancels application`：对应status2<br>- `administrator is processing`：对应status3<br>- `no application list to apply`：对应status4<br>- `the number of applications exceeds the limit`：对应status5<br>- `permission is not within the scope of application`：对应status6 |

![20210907-203347.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d05c82d3d4308826a3908849e250a0d1_HQh2fgV7PV.png?lazyload=true&width=1640&height=1339)

## 示例代码

```js
tt.applyTenantAppScope({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`openSetting fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
  "data": {
    "status": 4,
    "msg": "no application list to apply"
  }
}
```



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
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">应用能力</md-th>
      <md-th style="width: 20%;">Android</md-th>
       <md-th style="width: 20%;">iOS</md-th>
      <md-th style="width: 20%;">PC</md-th>
      <md-th style="width: 20%;">预览效果</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>小程序</md-td>
      <md-td><md-version>V3.43.0+</md-version></md-td>
      <md-td><md-version>V3.43.0+</md-version></md-td>
      <md-td><md-version>V3.43.0+</md-version></md-td>
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
     <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>/</md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性

## 输出
`success`返回对象的扩展属性：
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 30%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                data
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                权限申请结果
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    status
                </md-text>
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                状态码

**可选值**：
- `1`：用户已申请权限
- `2`：未申请权限(包括申请被拒绝)
- `3`：权限申请中
- `4`：无可申请列表
- `5`：相同授权超过数量限制
- `6`：仅租户敏感权限未授权
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    msg
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                结果信息

**可选值**：
- `user agrees to apply`：对应status1
- `user cancels application`：对应status2
- `administrator is processing`：对应status3
- `no application list to apply`：对应status4
- `the number of applications exceeds the limit`：对应status5
- `permission is not within the scope of application`：对应status6
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::
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



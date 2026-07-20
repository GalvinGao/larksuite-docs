---
document_id: '6965379543683907590'
directory_id: '6907567266541699074'
title: enterProfile
full_path: /uYjL24iN/ucDM04yNwQjL3ADN
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- UserInfo
- enterProfile
document_type: GuideDocumentType
updated_at: 2024-03-20T11:46:04Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucDM04yNwQjL3ADN
---

# enterProfile(Object object)

enterProfile(Object object) 用于进入个人信息主页。

## 注意事项

- 小程序调用该接口前，需要确保已经调用 [requestAccess](/document/uYjL24iN/uUzMuUzMuUzM/requestaccess) ( 如需兼容 LarkV6.9.0 以下版本，可使用 [login](/document/uYjL24iN/uYzMuYzMuYzM) )。
- 网页应用需要在[鉴权](/document/uYjL24iN/uEzM4YjLxMDO24SMzgjN)后调用该接口。
- Lark在 [V3.8.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 版本开始支持该接口的回调。


## 支持说明

该接口支持小程序和网页应用调用，对应的客户端版本支持情况如下所示。

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
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
     <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td> 
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>4.0.0</md-version></md-td>
      <md-td><md-version>4.0.0</md-version></md-td>
      <md-td><md-version>4.0.0</md-version></md-td>
     <md-td><md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app></md-td> 
</md-tr>  
</md-tbody>
</md-table>
:::

## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性说明如下。

:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 15%;">
                名称
            </md-th>
            <md-th style="width: 15%;">
                数据类型
            </md-th>
            <md-th style="width: 15%;">
                是否必填
            </md-th>
            <md-th style="width: 15%;">
                默认值
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                openid
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                是
            </md-td>
            <md-td>\-</md-td>
            <md-td>
                用户的 open_id。关于获取用户 ID 的操作说明，可参见 [如何获取不同的用户 ID](/document/home/user-identity-introduction/open-id)。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                left
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                0
            </md-td>
            <md-td>
                用户卡片原点（左上角）横坐标。单位：px
<md-alert type="tip" icon="none">
**注意**：Android、iOS 端暂不支持该字段。
</md-alert>  
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                top
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                0
            </md-td>
            <md-td>
                用户卡片原点（左上角）纵坐标。单位：px
<md-alert type="tip" icon="none">
**注意**：Android、iOS 端暂不支持该字段。
</md-alert>  
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性。

## 示例代码

调用示例：

```js
tt.chooseContact({
  success: (res) => {
    tt.enterProfile({
      openid: res.data[0].openId,
      left: 100,
      top: 200,
      success(res) {
        console.log(JSON.stringify(res));
      },
      fail(res) {
        console.log(`enterProfile fail: ${JSON.stringify(res)}`);
      }
    })
  }
})
```

`success`返回对象示例：
```json
{
    "errMsg": "enterProfile:ok"
}
```

## 错误码

`fail` 返回对象中可能包含 errno 属性，表示错误码。关于 errno 错误码的详细说明以及通用错误码列表，可参见[Errno 错误码](/document/uYjL24iN/uAjMuAjMuAjM/errno)。

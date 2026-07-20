---
document_id: '6965379543683416070'
directory_id: '6907567269107466242'
title: chooseContact
full_path: /uYjL24iN/uMTM04yMxQjLzEDN
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Contact
- chooseContact
document_type: GuideDocumentType
updated_at: 2024-03-20T11:46:38Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTM04yMxQjLzEDN
---

# chooseContact(Object object)

chooseContact(Object object) 用于打开用户的联系人选择列表，可以选择用户或部门，并返回选定用户或部门的信息。

## 注意事项

- 小程序调用该接口前，需要确保已经调用 [requestAccess](/document/uYjL24iN/uUzMuUzMuUzM/requestaccess) ( 如需兼容 LarkV6.9.0 以下版本，可使用 [login](/document/uYjL24iN/uYzMuYzMuYzM) )。
- 网页应用需要在[鉴权](/document/uYjL24iN/uEzM4YjLxMDO24SMzgjN)后调用该接口。



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
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/choose-contact/choose-contact" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V5.7.0+</md-version></md-td>
      <md-td><md-version>V5.7.0+</md-version></md-td>
      <md-td><md-version>V5.7.0+</md-version></md-td>
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览				 </md-preview-app>
	  </md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入

该接口继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性如下所示。

:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 20%;">
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
                multi
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                true
            </md-td>
            <md-td>
                是否支持多选联系人。取值：
- true：支持
- false：不支持
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                ignore
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>false</md-td>
            <md-td>
                选择列表中是否排除自己（当前登录用户）。取值：
- true：排除
- false：不排除
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                maxNum
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>\-</md-td>
            <md-td>
                多选联系人时，最大选人数量。
<md-alert type="tip" icon="none">
**注意**：
- Lark [V3.15.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该能力。
- 需要 `multi` 字段取值为 `true`，`maxNum` 可设置的数量没有限制，不传值时默认支持任意多选。
</md-alert>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                limitTips
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>\-</md-td>
            <md-td>
                达到选人上限时的提示文案。示例值：已达数量上限。
<md-alert type="tip" icon="none">
**注意**：Lark [V3.15.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该能力。
</md-alert>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                externalContact
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                true
            </md-td>
            <md-td>
                选择联系人列表中，是否包含外部联系人。取值：
- true：包含
- false：不包含
<md-alert type="tip" icon="none">
**注意**：Lark [V3.30.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该能力。
</md-alert>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                enableChooseDepartment
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>false</md-td>
            <md-td>
                是否支持选择部门。取值：
- true：支持
- false：不支持
<md-alert type="tip" icon="none">
**注意**：Lark [V4.1.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该能力。
</md-alert>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                chosenIds
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                指定要选取的联系人 open_id 数组。
- 可设置多个，无数量限制，格式为`["ou_xxx", "ou_yyy"]`。
- 用户 Open ID 的获取方式，可参见[如何获取不同的用户 ID](/document/home/user-identity-introduction/open-id)。
- 当 `multi` 取值 `false` 时，该字段不生效。

<md-alert type="tip" icon="none">
**注意**：Lark [V3.13.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该能力。
</md-alert>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                disableChosenIds
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                通过该字段指定联系人 open_id 数组后，相对应的联系人在列表内将处于置灰、不可选择的状态。
- 可设置多个，无数量限制，格式为`["ou_xxx", "ou_yyy"]`。
- 用户 Open ID 的获取方式，可参见[如何获取不同的用户 ID](/document/home/user-identity-introduction/open-id)。
- 当 `multi` 取值 `false` 时，该字段不生效。
   

<md-alert type="tip" icon="none">
**注意**：Lark [V3.15.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该能力。
</md-alert>
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 输出

该接口继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，`success` 返回对象的扩展属性如下所示。

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
                user[]
            </md-td>
            <md-td>
                选择用户后，返回选中用户的信息。
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
                    openId
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用户的 open_id。关于用户 ID 的说明，可参见[如何选择使用哪种 ID](/document/home/user-identity-introduction/user-id)。
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
                    unionId
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用户的 union_id。关于用户 ID 的说明，可参见[如何选择使用哪种 ID](/document/home/user-identity-introduction/user-id)。
<md-alert type="tip" icon="none">
**注意**：Lark [V5.4.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该输出字段。
</md-alert>
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
                    name
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                用户姓名。
              
**字段权限要求**：
<md-perm name="contact:user.base:readonly" desc="获取用户基本信息" support_app_types="custom,isv" tags="">获取用户基本信息</md-perm>
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
                    i18nNames
                </md-text>
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                国际化姓名。
              
**字段权限要求**：
<md-perm name="contact:user.base:readonly" desc="获取用户基本信息" support_app_types="custom,isv" tags="">获取用户基本信息</md-perm>
<md-alert type="tip" icon="none">
**注意**：Lark [V3.13.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该输出字段。
</md-alert>

            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    zh_cn
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                中文名，可能为空。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    en_us
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                英文名，可能为空。
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                &emsp;&emsp;
                <span style="color: #8F959E">
                    ∟
                </span>
                &nbsp;
                <md-text type="field-name">
                    ja_jp
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                日文名，可能为空。
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
                    avatarUrls
                </md-text>
            </md-td>
            <md-td>
                string[]
            </md-td>
            <md-td>
                联系人的头像 URL 数组，包含多种图片分辨率。
              
**字段权限要求**：
<md-perm name="contact:user.base:readonly" desc="获取用户基本信息" support_app_types="custom,isv" tags="">获取用户基本信息</md-perm>
<md-alert type="tip" icon="none">
**注意**：Lark [V3.13.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该输出字段。
</md-alert>

            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                department_data
            </md-td>
            <md-td>
                department[]
            </md-td>
            <md-td>
                选择部门后，返回选中部门的信息。
<md-alert type="tip" icon="none">
**注意**：Lark [V4.1.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该输出字段。
</md-alert>

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
                    departmentId
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                部门 departmentId，只作为唯一标识，不能用来请求 Open API。关于部门 ID 的说明，可参见[部门资源介绍](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview)。
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
                    openDepartmentId
                </md-text>
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                部门 openDepartmentId，可以用来请求 Open API。关于部门 ID 的说明，可参见[部门资源介绍](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview)。
              
<md-alert type="tip" icon="none">
**注意**：Lark [V5.2.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该输出字段。
</md-alert>
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 示例代码

调用示例：

:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/choose-contact/choose-contact" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.chooseContact({
    multi: true,
    ignore: false,
    maxNum: 10,
    limitTips: "选择人数达到上限了",
    externalContact: true,
    enableChooseDepartment: true,
    disableChosenIds: [
        "ou_7dab8a3d3cdcc9da365777c7ad53xxxx"
    ],
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`chooseContact fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：

```json
{
    "data": [
        {
            "avatarUrls": [
                "https://s3-imfile.feishucdn.com/static-resource/v1/v2_92c4fbb9-6706-439c-9d20-3b474af0xxxx~?image_size=72x72&cut_type=&quality=&format=png&sticker_format=.webp",
                "https://s3-imfile.feishucdn.com/static-resource/v1/v2_92c4fbb9-6706-439c-9d20-3b474af0xxxx~?image_size=240x240&cut_type=&quality=&format=png&sticker_format=.webp",
                "https://s3-imfile.feishucdn.com/static-resource/v1/v2_92c4fbb9-6706-439c-9d20-3b474af0xxxx~?image_size=noop&cut_type=&quality=&format=png&sticker_format=.webp",
                "https://s1-imfile.feishucdn.com/static-resource/v1/v2_92c4fbb9-6706-439c-9d20-3b474af0xxxx~?image_size=640x640&cut_type=&quality=&format=png&sticker_format=.webp"
            ],
            "name": "汤姆",
            "openId": "ou_de627cfb342ed0aaa380fbf0785cxxxx",
            "unionId": "on_cad4860e7af114fb4ff6c5d496d1xxxx",
            "i18nNames": {
                "en_us": "Tom",
                "ja_jp": "",
                "zh_cn": "汤姆"
            }
        }
    ],
    "department_data": [],
    "errMsg": "chooseContact:ok"
}
```

## 错误码

`fail` 返回对象中可能包含 errno 属性，表示错误码。关于 errno 错误码的详细说明以及通用错误码列表，可参见[Errno 错误码](/document/uYjL24iN/uAjMuAjMuAjM/errno)。

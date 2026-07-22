---
document_id: '6965379543683399686'
directory_id: '6907567269107597314'
title: chooseChat
full_path: /uYjL24iN/uMTN3QjLzUzN04yM1cDN
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Chat
- chooseChat
document_type: GuideDocumentType
updated_at: 2024-03-20T11:46:16Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTN3QjLzUzN04yM1cDN
---

# chooseChat(Object object)

chooseChat(Object object) 用于打开用户会话列表的选择会话。


## 注意事项

- 小程序调用该接口前，需要确保已经调用 [requestAccess](/document/uYjL24iN/uUzMuUzMuUzM/requestaccess) ( 如需兼容 LarkV6.9.0 以下版本，可使用 [login](/document/uYjL24iN/uYzMuYzMuYzM) )。
- 网页应用需要在[鉴权](/document/uYjL24iN/uEzM4YjLxMDO24SMzgjN)后调用该接口。



## 支持说明

该接口支持小程序和网页应用调用，对应的客户端版本支持情况如下所示。

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V3.1.0+</md-version> | <md-version>V3.1.0+</md-version> | <md-version>V3.1.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/choose-chat/choose-chat" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



## 输入

该接口继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性如下所示。

| 名称 | 数据类型 | 是否必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| allowCreateGroup | boolean | 否 | true | 是否允许在选择会话列表内，创建群组并转发。取值：<br>- true：允许<br>- false：不允许<br><md-alert type="tip" icon="none"><br>**注意**：Lark PC 端从 V5.28 版本开始支持该能力。<br></md-alert> |
| multiSelect | boolean | 否 | false | 是否允许多选。取值为 true 时，支持在会话列表选择多个会话（最多可选择 10 个）。取值：<br>- true：允许<br>- false：不允许 |
| ignoreSelf | boolean | 否 | false | 是否在会话列表内过滤掉自己。取值：<br>- true：过滤<br>- false：不过滤 |
| ignoreBot | boolean | 否 | false | 是否在会话列表内过滤掉机器人。取值：<br>- true：过滤<br>- false：不过滤<br><md-alert type="tip" icon="none"><br>**注意**：<br>- Android、iOS 端：Lark [V3.9.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该能力。<br>- PC 端：暂不支持使用。<br></md-alert> |
| selectType | number | 否 | 0 | 选择模式。可选值：<br>- `0`：选择单聊或群聊<br>- `1`：只选择群聊<br>- `2`：只选择单聊（该模式下，allowCreateGroup 设置为 true 将不会生效） |
| confirmTitle | string | 否 | Chosen | 选中会话后，确认弹框的标题名称。示例值：已选择。 |
| confirmText | string | 否 | 发送 | 选中会话后，确认弹框的确认按钮文案。<br><md-alert type="tip" icon="none"><br>**注意**：Lark [V5.31.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该能力。<br></md-alert> |
| confirmDesc | string | 否 |  | 选中会话后，确认弹框的描述。该字段为空时，界面将不展示描述框。 |
| externalChat | boolean | 否 | true | 选择的会话是否包含外部会话。取值：<br>- true：包含<br>- false：不包含<br><md-alert type="tip" icon="none"><br>**注意**：Lark [V3.30.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该能力。<br></md-alert> |
| showMessageInput | boolean | 否 | false | 选中会话后，确认弹框内是否显示留言输入框。取值：<br>- true：显示<br>- false：不显示<br><md-alert type="tip" icon="none"><br>**注意**：Lark [V4.10.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该能力。<br></md-alert> |


## 输出

该接口继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，`success` 返回对象的扩展属性如下所示。

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| data | object[] | 选择会话列表信息。 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>id<br></md-text> | string | 会话 ID（chat_id），详情可参见 [群 ID 说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)。 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>chatType<br></md-text> | number | 会话类型。可能值：<br>- `0`：单聊<br>- `1`：群聊 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>userType<br></md-text> | number | 单聊类型，仅当 `chatType` 为 `0` 时，会返回此参数。可能值：<br>- `0`：普通用户<br>- `1`：机器人 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>avatarUrls<br></md-text> | string[] | 会话的头像 URL数组，包含多种图片的分辨率。<br>**字段权限要求**：<br><md-perm name="im:chat.group_info:readonly" desc="读取群信息" support_app_types="custom,isv" tags="">读取群信息</md-perm><br><md-alert type="tip" icon="none"><br>**注意**：<br>- Android、iOS 端：Lark [V3.9.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该输出字段。<br>- PC 端：Lark [V3.17.2](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该输出字段。<br></md-alert> |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>name<br></md-text> | string | 会话名称。<br>**字段权限要求**：<br><md-perm name="im:chat.group_info:readonly" desc="读取群信息" support_app_types="custom,isv" tags="">读取群信息</md-perm><br><md-alert type="tip" icon="none"><br>**注意**：<br>- Android、iOS 端：Lark [V3.9.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持。<br>- PC 端：Lark [V3.17.2](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持。<br></md-alert> |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>i18nNames<br></md-text> | object | 国际化会话名。<br>**字段权限要求**：<br><md-perm name="im:chat.group_info:readonly" desc="读取群信息" support_app_types="custom,isv" tags="">读取群信息</md-perm><br><md-alert type="tip" icon="none"><br>**注意**：<br>- Android、iOS 端：Lark [V3.9.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该输出字段。<br>- PC 端：Lark [V3.17.2](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该输出字段。<br></md-alert> |
| &emsp;&emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>zh_cn<br></md-text> | string | 中文名，可能为空。 |
| &emsp;&emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>en_us<br></md-text> | string | 英文名，可能为空。 |
| &emsp;&emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>ja_jp<br></md-text> | string | 日文名，可能为空。 |
| message | string | 用户输入的留言，当输入字段 `showMessageInput` 为 `true` 时才会输出该字段。<br><md-alert type="tip" icon="none"><br>注意：Lark [V4.10.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持该输出字段。<br></md-alert> |


## 示例代码

调用示例：

:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/choose-chat/choose-chat" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" disable="true" fontSize="16" style="margin-right: 24px">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.chooseChat({
    allowCreateGroup: true,
    multiSelect: true,
    ignoreSelf: false,
    ignoreBot: false,
    selectType: 0,
    confirmTitle: "确认",
    confirmDesc: "确认发送？",
    externalChat: true,
    showMessageInput: true,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`chooseChat fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "chooseChat:ok",
    "message": "",
    "data": [
        {
            "name": "Lark团队",
            "i18nNames": {
                "en_us": "Lark Team",
                "zh_cn": "Lark团队",
                "ja_jp": "Lark チーム"
            },
            "id": "oc_a88923604e75701a65980f8d1a1dxxxx",
            "avatarUrls": [
                "https://s3-imfile.feishucdn.com/static-resource/v1/v2_d6e145c6-f7bb-492b-9743-5a0d1d6843eg~?image_size=72x72&cut_type=&quality=&format=png&sticker_format=.webp",
                "https://s1-imfile.feishucdn.com/static-resource/v1/v2_d6e145c6-f7bb-492b-9743-5a0d1d6843eg~?image_size=240x240&cut_type=&quality=&format=png&sticker_format=.webp",
                "https://s3-imfile.feishucdn.com/static-resource/v1/v2_d6e145c6-f7bb-492b-9743-5a0d1d6843eg~?image_size=noop&cut_type=&quality=&format=png&sticker_format=.webp",
                "https://s1-imfile.feishucdn.com/static-resource/v1/v2_d6e145c6-f7bb-492b-9743-5a0d1d6843eg~?image_size=640x640&cut_type=&quality=&format=png&sticker_format=.webp"
            ],
            "chatType": 0,
            "userType": 1
        }
    ]
}
```

## 错误码

`fail` 返回对象中可能包含 errno 属性，表示错误码。关于 errno 错误码的详细说明以及通用错误码列表，可参见[Errno 错误码](/document/uYjL24iN/uAjMuAjMuAjM/errno)。

---
document_id: '7072608739626893317'
directory_id: '7021712771402711046'
title: 申请发布应用
full_path: /uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/events/publish_apply
breadcrumb:
- Server API
- App Information
- Event
- Apply To Publish an Application
document_type: ReferenceDocumentType
updated_at: 2022-03-08T06:14:05Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/events/publish_apply
---

# 申请发布应用

通过订阅该事件，可接收应用提交发布申请事件{使用示例}(url=/api/tools/api_explore/api_explore_config?project=application&version=v6&resource=application.app_version&event=publish_apply)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">

</md-alert>
:::

:::html
<md-alert type="tip">

</md-alert>
:::



## 事件

| 基本 |  |
| --- | --- |
| 事件类型 | application.application.app_version.publish_apply_v6 |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">订阅该事件所需的权限，开启其中任意一项权限即可订阅</md-tooltip> | <md-perm name="application:application.app_version:readonly" desc="获取应用版本信息" support_app_types="custom" tags="">获取应用版本信息</md-perm> |
| 字段权限要求 | <md-alert type="tip" icon="none"><br>该接口返回体中存在下列敏感字段，仅当开启对应的权限后才会返回；如果无需获取这些字段，则不建议申请<br></md-alert><br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm> |
| 推送方式 | <md-tag mode="inline" type="push-webhook" href="/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM" >Webhook</md-tag> |




### 事件体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >schema</md-text> | <md-text type="field-type" >string</md-text> | 事件模式 |
| <md-text type="field-name" >header</md-text> | <md-text type="field-type" >event_header</md-text> | 事件头 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >event_id</md-text> | <md-text type="field-type" >string</md-text> | 事件 ID |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >event_type</md-text> | <md-text type="field-type" >string</md-text> | 事件类型 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >create_time</md-text> | <md-text type="field-type" >string</md-text> | 事件创建时间戳（单位：毫秒） |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >token</md-text> | <md-text type="field-type" >string</md-text> | 事件 Token |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >app_id</md-text> | <md-text type="field-type" >string</md-text> | 应用 ID |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >tenant_key</md-text> | <md-text type="field-type" >string</md-text> | 租户 Key |
| <md-text type="field-name" >event</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >operator_id</md-text> | <md-text type="field-type" >user_id</md-text> | 用户 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >union_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 union id |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 user id<br>**字段权限要求**：<br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm> |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >open_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 open id |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >online_version</md-text> | <md-text type="field-type" >application.app_version_event</md-text> | 当前线上版本信息 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >app_id</md-text> | <md-text type="field-type" >string</md-text> | 应用 id |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >version</md-text> | <md-text type="field-type" >string</md-text> | 开发者填入的应用版本 ID<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >version_id</md-text> | <md-text type="field-type" >string</md-text> | 唯一标识应用版本的 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >app_name</md-text> | <md-text type="field-type" >string</md-text> | 应用默认名称<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >avatar_url</md-text> | <md-text type="field-type" >string</md-text> | 应用头像 url |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >description</md-text> | <md-text type="field-type" >string</md-text> | 应用默认描述<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >scopes</md-text> | <md-text type="field-type" >app_scope\[\]</md-text> | 应用权限列表 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >scope</md-text> | <md-text type="field-type" >string</md-text> | 应用权限 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >description</md-text> | <md-text type="field-type" >string</md-text> | 应用权限的国际化描述 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >level</md-text> | <md-text type="field-type" >string</md-text> | 权限等级描述<br>**可选值有**：<br>- `1`：普通权限<br>- `2`：高级权限<br>- `3`：超敏感权限<br>- `0`：未知等级 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >back_home_url</md-text> | <md-text type="field-type" >string</md-text> | 后台主页地址 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n</md-text> | <md-text type="field-type" >app_i18n_info\[\]</md-text> | 应用的国际化信息列表<br>**数据校验规则**：<br>- 最小长度：`1` |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n_key</md-text> | <md-text type="field-type" >string</md-text> | 国际化语言的 key<br>**可选值有**：<br>- `zh_cn`：中文<br>- `en_us`：英文<br>- `ja_jp`：日文 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 应用国际化名称<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >description</md-text> | <md-text type="field-type" >string</md-text> | 应用国际化描述（副标题）<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >help_use</md-text> | <md-text type="field-type" >string</md-text> | 帮助国际化文档链接 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >common_categories</md-text> | <md-text type="field-type" >string\[\]</md-text> | 应用分类的国际化描述<br>**数据校验规则**：<br>- 最大长度：`3` |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >events</md-text> | <md-text type="field-type" >string\[\]</md-text> | 应用已订阅开放平台事件列表 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >status</md-text> | <md-text type="field-type" >int</md-text> | 版本状态<br>**可选值有**：<br>- `0`：未知状态<br>- `1`：审核通过<br>- `2`：审核拒绝<br>- `3`：审核中<br>- `4`：未提交审核 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >create_time</md-text> | <md-text type="field-type" >string</md-text> | 版本创建时间（单位：s） |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >publish_time</md-text> | <md-text type="field-type" >string</md-text> | 版本发布时间（单位：s） |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >ability</md-text> | <md-text type="field-type" >app_ability</md-text> | 当前版本下应用开启的能力 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >gadget</md-text> | <md-text type="field-type" >gadget</md-text> | 小程序能力 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >enable_pc_mode</md-text> | <md-text type="field-type" >int</md-text> | pc 支持的小程序模式，bit 位表示<br>**可选值有**：<br>- `1`：sidebar 模式<br>- `2`：pc 模式<br>- `4`：主导航模式 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >schema_urls</md-text> | <md-text type="field-type" >string\[\]</md-text> | schema url 列表 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc_use_mobile_pkg</md-text> | <md-text type="field-type" >boolean</md-text> | pc 端是否使用小程序版本 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc_version</md-text> | <md-text type="field-type" >string</md-text> | pc 的小程序版本号<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mobile_version</md-text> | <md-text type="field-type" >string</md-text> | 移动端小程序版本号<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mobile_min_lark_version</md-text> | <md-text type="field-type" >string</md-text> | 移动端兼容的最低Lark版本<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc_min_lark_version</md-text> | <md-text type="field-type" >string</md-text> | pc 端兼容的最低Lark版本<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >web_app</md-text> | <md-text type="field-type" >web_app</md-text> | 网页能力 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc_url</md-text> | <md-text type="field-type" >string</md-text> | pc 端 url |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mobile_url</md-text> | <md-text type="field-type" >string</md-text> | 移动端 url |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >bot</md-text> | <md-text type="field-type" >bot</md-text> | 机器人能力 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >card_request_url</md-text> | <md-text type="field-type" >string</md-text> | 消息卡片回调地址 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >workplace_widgets</md-text> | <md-text type="field-type" >workplace_widget\[\]</md-text> | 小组件能力 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >min_lark_version</md-text> | <md-text type="field-type" >string</md-text> | 最低兼容 lark 版本号 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >navigate</md-text> | <md-text type="field-type" >navigate</md-text> | 主导航小程序 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc</md-text> | <md-text type="field-type" >navigate_meta</md-text> | pc 端主导航信息 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >version</md-text> | <md-text type="field-type" >string</md-text> | 主导航小程序版本号<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >image_url</md-text> | <md-text type="field-type" >string</md-text> | 默认图片 url |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >hover_image_url</md-text> | <md-text type="field-type" >string</md-text> | 选中态图片 url |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mobile</md-text> | <md-text type="field-type" >navigate_meta</md-text> | 移动端主导航信息 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >version</md-text> | <md-text type="field-type" >string</md-text> | 主导航小程序版本号<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >image_url</md-text> | <md-text type="field-type" >string</md-text> | 默认图片 url |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >hover_image_url</md-text> | <md-text type="field-type" >string</md-text> | 选中态图片 url |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >cloud_doc</md-text> | <md-text type="field-type" >cloud_doc</md-text> | 云文档应用 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >space_url</md-text> | <md-text type="field-type" >string</md-text> | 云空间重定向 url |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n</md-text> | <md-text type="field-type" >cloud_doc_i18n_info\[\]</md-text> | 国际化信息<br>**数据校验规则**：<br>- 最小长度：`1` |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n_key</md-text> | <md-text type="field-type" >string</md-text> | 国际化语言的 key<br>**可选值有**：<br>- `zh_cn`：中文<br>- `en_us`：英文<br>- `ja_jp`：日文 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 云文档国际化名称 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >read_description</md-text> | <md-text type="field-type" >string</md-text> | 云文档国际化读权限说明 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >write_description</md-text> | <md-text type="field-type" >string</md-text> | 云文档国际化写权限说明 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >icon_url</md-text> | <md-text type="field-type" >string</md-text> | 图标链接 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mode</md-text> | <md-text type="field-type" >int</md-text> | 云文档支持模式<br>**可选值有**：<br>- `0`：未知<br>- `1`：移动端 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >docs_blocks</md-text> | <md-text type="field-type" >docs_block\[\]</md-text> | 云文档小组件 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >block_type_id</md-text> | <md-text type="field-type" >string</md-text> | BlockTypeID |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n</md-text> | <md-text type="field-type" >block_i18n_info\[\]</md-text> | block 的国际化信息<br>**数据校验规则**：<br>- 最小长度：`1` |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n_key</md-text> | <md-text type="field-type" >string</md-text> | 国际化语言的 key<br>**可选值有**：<br>- `zh_cn`：中文<br>- `en_us`：英文<br>- `ja_jp`：日文 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 名称<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mobile_icon_url</md-text> | <md-text type="field-type" >string</md-text> | 移动端 icon 链接 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc_icon_url</md-text> | <md-text type="field-type" >string</md-text> | pc 端口 icon 链接 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >message_action</md-text> | <md-text type="field-type" >message_action</md-text> | 消息快捷操作 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc_app_link</md-text> | <md-text type="field-type" >string</md-text> | pc 端链接 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mobile_app_link</md-text> | <md-text type="field-type" >string</md-text> | 移动端链接 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n</md-text> | <md-text type="field-type" >message_action_i18n_info\[\]</md-text> | 国际化信息<br>**数据校验规则**：<br>- 最小长度：`1` |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n_key</md-text> | <md-text type="field-type" >string</md-text> | 国际化语言的 key<br>**可选值有**：<br>- `zh_cn`：中文<br>- `en_us`：英文<br>- `ja_jp`：日文 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 国际化名称<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >plus_menu</md-text> | <md-text type="field-type" >plus_menu</md-text> | 加号菜单 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc_app_link</md-text> | <md-text type="field-type" >string</md-text> | pc 端链接 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mobile_app_link</md-text> | <md-text type="field-type" >string</md-text> | 移动端链接 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >remark</md-text> | <md-text type="field-type" >app_version_remark_event</md-text> | 跟随应用版本的信息 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >remark</md-text> | <md-text type="field-type" >string</md-text> | 备注说明 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >update_remark</md-text> | <md-text type="field-type" >string</md-text> | 更新说明 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >visibility</md-text> | <md-text type="field-type" >app_visibility_event</md-text> | 可见性名单 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_all</md-text> | <md-text type="field-type" >boolean</md-text> | 是否全员可见 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >visible_list</md-text> | <md-text type="field-type" >app_visible_list_event</md-text> | 可见名单 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >open_ids</md-text> | <md-text type="field-type" >user_id\[\]</md-text> | 可见性成员 id 列表 |
| &emsp;&emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >union_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 union id |
| &emsp;&emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 user id |
| &emsp;&emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >open_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 open id |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >department_ids</md-text> | <md-text type="field-type" >string\[\]</md-text> | 可见性部门的 id 列表 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >invisible_list</md-text> | <md-text type="field-type" >app_visible_list_event</md-text> | 不可见名单 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >open_ids</md-text> | <md-text type="field-type" >user_id\[\]</md-text> | 可见性成员 id 列表 |
| &emsp;&emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >union_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 union id |
| &emsp;&emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 user id |
| &emsp;&emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >open_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 open id |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >department_ids</md-text> | <md-text type="field-type" >string\[\]</md-text> | 可见性部门的 id 列表 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >under_audit_version</md-text> | <md-text type="field-type" >application.app_version_event</md-text> | 当前线上版本信息 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >app_id</md-text> | <md-text type="field-type" >string</md-text> | 应用 id |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >version</md-text> | <md-text type="field-type" >string</md-text> | 开发者填入的应用版本 ID<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >version_id</md-text> | <md-text type="field-type" >string</md-text> | 唯一标识应用版本的 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >app_name</md-text> | <md-text type="field-type" >string</md-text> | 应用默认名称<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >avatar_url</md-text> | <md-text type="field-type" >string</md-text> | 应用头像 url |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >description</md-text> | <md-text type="field-type" >string</md-text> | 应用默认描述<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >scopes</md-text> | <md-text type="field-type" >app_scope\[\]</md-text> | 应用权限列表 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >scope</md-text> | <md-text type="field-type" >string</md-text> | 应用权限 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >description</md-text> | <md-text type="field-type" >string</md-text> | 应用权限的国际化描述 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >level</md-text> | <md-text type="field-type" >string</md-text> | 权限等级描述<br>**可选值有**：<br>- `1`：普通权限<br>- `2`：高级权限<br>- `3`：超敏感权限<br>- `0`：未知等级 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >back_home_url</md-text> | <md-text type="field-type" >string</md-text> | 后台主页地址 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n</md-text> | <md-text type="field-type" >app_i18n_info\[\]</md-text> | 应用的国际化信息列表<br>**数据校验规则**：<br>- 最小长度：`1` |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n_key</md-text> | <md-text type="field-type" >string</md-text> | 国际化语言的 key<br>**可选值有**：<br>- `zh_cn`：中文<br>- `en_us`：英文<br>- `ja_jp`：日文 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 应用国际化名称<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >description</md-text> | <md-text type="field-type" >string</md-text> | 应用国际化描述（副标题）<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >help_use</md-text> | <md-text type="field-type" >string</md-text> | 帮助国际化文档链接 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >common_categories</md-text> | <md-text type="field-type" >string\[\]</md-text> | 应用分类的国际化描述<br>**数据校验规则**：<br>- 最大长度：`3` |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >events</md-text> | <md-text type="field-type" >string\[\]</md-text> | 应用已订阅开放平台事件列表 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >status</md-text> | <md-text type="field-type" >int</md-text> | 版本状态<br>**可选值有**：<br>- `0`：未知状态<br>- `1`：审核通过<br>- `2`：审核拒绝<br>- `3`：审核中<br>- `4`：未提交审核 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >create_time</md-text> | <md-text type="field-type" >string</md-text> | 版本创建时间（单位：s） |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >publish_time</md-text> | <md-text type="field-type" >string</md-text> | 版本发布时间（单位：s） |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >ability</md-text> | <md-text type="field-type" >app_ability</md-text> | 当前版本下应用开启的能力 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >gadget</md-text> | <md-text type="field-type" >gadget</md-text> | 小程序能力 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >enable_pc_mode</md-text> | <md-text type="field-type" >int</md-text> | pc 支持的小程序模式，bit 位表示<br>**可选值有**：<br>- `1`：sidebar 模式<br>- `2`：pc 模式<br>- `4`：主导航模式 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >schema_urls</md-text> | <md-text type="field-type" >string\[\]</md-text> | schema url 列表 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc_use_mobile_pkg</md-text> | <md-text type="field-type" >boolean</md-text> | pc 端是否使用小程序版本 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc_version</md-text> | <md-text type="field-type" >string</md-text> | pc 的小程序版本号<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mobile_version</md-text> | <md-text type="field-type" >string</md-text> | 移动端小程序版本号<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mobile_min_lark_version</md-text> | <md-text type="field-type" >string</md-text> | 移动端兼容的最低Lark版本<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc_min_lark_version</md-text> | <md-text type="field-type" >string</md-text> | pc 端兼容的最低Lark版本<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >web_app</md-text> | <md-text type="field-type" >web_app</md-text> | 网页能力 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc_url</md-text> | <md-text type="field-type" >string</md-text> | pc 端 url |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mobile_url</md-text> | <md-text type="field-type" >string</md-text> | 移动端 url |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >bot</md-text> | <md-text type="field-type" >bot</md-text> | 机器人能力 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >card_request_url</md-text> | <md-text type="field-type" >string</md-text> | 消息卡片回调地址 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >workplace_widgets</md-text> | <md-text type="field-type" >workplace_widget\[\]</md-text> | 小组件能力 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >min_lark_version</md-text> | <md-text type="field-type" >string</md-text> | 最低兼容 lark 版本号 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >navigate</md-text> | <md-text type="field-type" >navigate</md-text> | 主导航小程序 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc</md-text> | <md-text type="field-type" >navigate_meta</md-text> | pc 端主导航信息 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >version</md-text> | <md-text type="field-type" >string</md-text> | 主导航小程序版本号<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >image_url</md-text> | <md-text type="field-type" >string</md-text> | 默认图片 url |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >hover_image_url</md-text> | <md-text type="field-type" >string</md-text> | 选中态图片 url |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mobile</md-text> | <md-text type="field-type" >navigate_meta</md-text> | 移动端主导航信息 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >version</md-text> | <md-text type="field-type" >string</md-text> | 主导航小程序版本号<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >image_url</md-text> | <md-text type="field-type" >string</md-text> | 默认图片 url |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >hover_image_url</md-text> | <md-text type="field-type" >string</md-text> | 选中态图片 url |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >cloud_doc</md-text> | <md-text type="field-type" >cloud_doc</md-text> | 云文档应用 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >space_url</md-text> | <md-text type="field-type" >string</md-text> | 云空间重定向 url |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n</md-text> | <md-text type="field-type" >cloud_doc_i18n_info\[\]</md-text> | 国际化信息<br>**数据校验规则**：<br>- 最小长度：`1` |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n_key</md-text> | <md-text type="field-type" >string</md-text> | 国际化语言的 key<br>**可选值有**：<br>- `zh_cn`：中文<br>- `en_us`：英文<br>- `ja_jp`：日文 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 云文档国际化名称 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >read_description</md-text> | <md-text type="field-type" >string</md-text> | 云文档国际化读权限说明 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >write_description</md-text> | <md-text type="field-type" >string</md-text> | 云文档国际化写权限说明 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >icon_url</md-text> | <md-text type="field-type" >string</md-text> | 图标链接 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mode</md-text> | <md-text type="field-type" >int</md-text> | 云文档支持模式<br>**可选值有**：<br>- `0`：未知<br>- `1`：移动端 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >docs_blocks</md-text> | <md-text type="field-type" >docs_block\[\]</md-text> | 云文档小组件 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >block_type_id</md-text> | <md-text type="field-type" >string</md-text> | BlockTypeID |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n</md-text> | <md-text type="field-type" >block_i18n_info\[\]</md-text> | block 的国际化信息<br>**数据校验规则**：<br>- 最小长度：`1` |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n_key</md-text> | <md-text type="field-type" >string</md-text> | 国际化语言的 key<br>**可选值有**：<br>- `zh_cn`：中文<br>- `en_us`：英文<br>- `ja_jp`：日文 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 名称<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mobile_icon_url</md-text> | <md-text type="field-type" >string</md-text> | 移动端 icon 链接 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc_icon_url</md-text> | <md-text type="field-type" >string</md-text> | pc 端口 icon 链接 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >message_action</md-text> | <md-text type="field-type" >message_action</md-text> | 消息快捷操作 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc_app_link</md-text> | <md-text type="field-type" >string</md-text> | pc 端链接 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mobile_app_link</md-text> | <md-text type="field-type" >string</md-text> | 移动端链接 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n</md-text> | <md-text type="field-type" >message_action_i18n_info\[\]</md-text> | 国际化信息<br>**数据校验规则**：<br>- 最小长度：`1` |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >i18n_key</md-text> | <md-text type="field-type" >string</md-text> | 国际化语言的 key<br>**可选值有**：<br>- `zh_cn`：中文<br>- `en_us`：英文<br>- `ja_jp`：日文 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 国际化名称<br>**数据校验规则**：<br>- 最小长度：`1` 字符 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >plus_menu</md-text> | <md-text type="field-type" >plus_menu</md-text> | 加号菜单 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >pc_app_link</md-text> | <md-text type="field-type" >string</md-text> | pc 端链接 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >mobile_app_link</md-text> | <md-text type="field-type" >string</md-text> | 移动端链接 |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >remark</md-text> | <md-text type="field-type" >app_version_remark_event</md-text> | 跟随应用版本的信息 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >remark</md-text> | <md-text type="field-type" >string</md-text> | 备注说明 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >update_remark</md-text> | <md-text type="field-type" >string</md-text> | 更新说明 |
| &emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >visibility</md-text> | <md-text type="field-type" >app_visibility_event</md-text> | 可见性名单 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >is_all</md-text> | <md-text type="field-type" >boolean</md-text> | 是否全员可见 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >visible_list</md-text> | <md-text type="field-type" >app_visible_list_event</md-text> | 可见名单 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >open_ids</md-text> | <md-text type="field-type" >user_id\[\]</md-text> | 可见性成员 id 列表 |
| &emsp;&emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >union_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 union id |
| &emsp;&emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 user id |
| &emsp;&emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >open_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 open id |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >department_ids</md-text> | <md-text type="field-type" >string\[\]</md-text> | 可见性部门的 id 列表 |
| &emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >invisible_list</md-text> | <md-text type="field-type" >app_visible_list_event</md-text> | 不可见名单 |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >open_ids</md-text> | <md-text type="field-type" >user_id\[\]</md-text> | 可见性成员 id 列表 |
| &emsp;&emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >union_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 union id |
| &emsp;&emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 user id |
| &emsp;&emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >open_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 open id |
| &emsp;&emsp;&emsp;&emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >department_ids</md-text> | <md-text type="field-type" >string\[\]</md-text> | 可见性部门的 id 列表 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >app_status</md-text> | <md-text type="field-type" >int</md-text> | 应用状态<br>**可选值有**：<br>- `0`：停用状态<br>- `1`：启用状态 |




### 事件体示例

```json
{
    "schema": "2.0",
    "header": {
        "event_id": "5e3702a84e847582be8db7fb73283c02",
        "event_type": "application.application.app_version.publish_apply_v6",
        "create_time": "1608725989000",
        "token": "rvaYgkND1GOiu5MM0E1rncYC6PLtF7JV",
        "app_id": "cli_9f5343c580712544",
        "tenant_key": "2ca1d211f64f6438"
    },
    "event": {
        "operator_id": {
            "union_id": "on_8ed6aa67826108097d9ee143816345",
            "user_id": "e33ggbyz",
            "open_id": "ou_84aad35d084aa403a838cf73ee18467"
        },
        "online_version": {
            "app_id": "cli_9f3ca975326b501b",
            "version": "1.0.0",
            "version_id": "oav_d317f090b7258ad0372aa53963cda70d",
            "app_name": "应用名称",
            "avatar_url": "https://www.example.com",
            "description": "应用描述",
            "scopes": [
                {
                    "scope": "contact:user.base",
                    "description": "获取应用信息",
                    "level": "low_level"
                }
            ],
            "back_home_url": "https://www.example.com",
            "i18n": [
                {
                    "i18n_key": "zh_cn",
                    "name": "应用名称",
                    "description": "应用描述",
                    "help_use": "https://www.example.com"
                }
            ],
            "common_categories": [
                "分析工具"
            ],
            "events": [
                "应用审核事件"
            ],
            "status": 1,
            "create_time": "1610462759",
            "publish_time": "1610462759",
            "ability": {
                "gadget": {
                    "enable_pc_mode": 1,
                    "schema_urls": [
                        "*:*"
                    ],
                    "pc_use_mobile_pkg": false,
                    "pc_version": "1.0.0",
                    "mobile_version": "1.0.0",
                    "mobile_min_lark_version": "2.0",
                    "pc_min_lark_version": "2.0"
                },
                "web_app": {
                    "pc_url": "https://www.example.com",
                    "mobile_url": "https://www.example.com"
                },
                "bot": {
                    "card_request_url": "https://www.example.com"
                },
                "workplace_widgets": [
                    {
                        "min_lark_version": "1.0.0"
                    }
                ],
                "navigate": {
                    "pc": {
                        "version": "1.0.0",
                        "image_url": "https://www.example.com",
                        "hover_image_url": "https://www.example.com"
                    },
                    "mobile": {
                        "version": "1.0.0",
                        "image_url": "https://www.example.com",
                        "hover_image_url": "https://www.example.com"
                    }
                },
                "cloud_doc": {
                    "space_url": "https://www.example.com",
                    "i18n": [
                        {
                            "i18n_key": "zh_cn",
                            "name": "名称",
                            "read_description": "读权限说明",
                            "write_description": "写权限说明"
                        }
                    ],
                    "icon_url": "https://www.example.com",
                    "mode": 1
                },
                "docs_blocks": [
                    {
                        "block_type_id": "blk_4fb61568435880110854c1d0",
                        "i18n": [
                            {
                                "i18n_key": "zh_cn",
                                "name": "名称"
                            }
                        ],
                        "mobile_icon_url": "https://www.example.com",
                        "pc_icon_url": "https://www.example.com"
                    }
                ],
                "message_action": {
                    "pc_app_link": "https://www.example.com",
                    "mobile_app_link": "https://www.example.com",
                    "i18n": [
                        {
                            "i18n_key": "zh_cn",
                            "name": "名称"
                        }
                    ]
                },
                "plus_menu": {
                    "pc_app_link": "https://www.example.com",
                    "mobile_app_link": "https://www.example.com"
                }
            },
            "remark": {
                "remark": "备注说明",
                "update_remark": "更新说明",
                "visibility": {
                    "is_all": false,
                    "visible_list": {
                        "open_ids": [
                            {
                                "union_id": "on_8ed6aa67826108097d9ee143816345",
                                "user_id": "e33ggbyz",
                                "open_id": "ou_84aad35d084aa403a838cf73ee18467"
                            }
                        ],
                        "department_ids": [
                            "od-ddee42c0f8a948a5e650341e2153243b"
                        ]
                    },
                    "invisible_list": {
                        "open_ids": [
                            {
                                "union_id": "on_8ed6aa67826108097d9ee143816345",
                                "user_id": "e33ggbyz",
                                "open_id": "ou_84aad35d084aa403a838cf73ee18467"
                            }
                        ],
                        "department_ids": [
                            "od-ddee42c0f8a948a5e650341e2153243b"
                        ]
                    }
                }
            }
        },
        "under_audit_version": {
            "app_id": "cli_9f3ca975326b501b",
            "version": "1.0.0",
            "version_id": "oav_d317f090b7258ad0372aa53963cda70d",
            "app_name": "应用名称",
            "avatar_url": "https://www.example.com",
            "description": "应用描述",
            "scopes": [
                {
                    "scope": "contact:user.base",
                    "description": "获取应用信息",
                    "level": "low_level"
                }
            ],
            "back_home_url": "https://www.example.com",
            "i18n": [
                {
                    "i18n_key": "zh_cn",
                    "name": "应用名称",
                    "description": "应用描述",
                    "help_use": "https://www.example.com"
                }
            ],
            "common_categories": [
                "分析工具"
            ],
            "events": [
                "应用审核事件"
            ],
            "status": 1,
            "create_time": "1610462759",
            "publish_time": "1610462759",
            "ability": {
                "gadget": {
                    "enable_pc_mode": 1,
                    "schema_urls": [
                        "*:*"
                    ],
                    "pc_use_mobile_pkg": false,
                    "pc_version": "1.0.0",
                    "mobile_version": "1.0.0",
                    "mobile_min_lark_version": "2.0",
                    "pc_min_lark_version": "2.0"
                },
                "web_app": {
                    "pc_url": "https://www.example.com",
                    "mobile_url": "https://www.example.com"
                },
                "bot": {
                    "card_request_url": "https://www.example.com"
                },
                "workplace_widgets": [
                    {
                        "min_lark_version": "1.0.0"
                    }
                ],
                "navigate": {
                    "pc": {
                        "version": "1.0.0",
                        "image_url": "https://www.example.com",
                        "hover_image_url": "https://www.example.com"
                    },
                    "mobile": {
                        "version": "1.0.0",
                        "image_url": "https://www.example.com",
                        "hover_image_url": "https://www.example.com"
                    }
                },
                "cloud_doc": {
                    "space_url": "https://www.example.com",
                    "i18n": [
                        {
                            "i18n_key": "zh_cn",
                            "name": "名称",
                            "read_description": "读权限说明",
                            "write_description": "写权限说明"
                        }
                    ],
                    "icon_url": "https://www.example.com",
                    "mode": 1
                },
                "docs_blocks": [
                    {
                        "block_type_id": "blk_4fb61568435880110854c1d0",
                        "i18n": [
                            {
                                "i18n_key": "zh_cn",
                                "name": "名称"
                            }
                        ],
                        "mobile_icon_url": "https://www.example.com",
                        "pc_icon_url": "https://www.example.com"
                    }
                ],
                "message_action": {
                    "pc_app_link": "https://www.example.com",
                    "mobile_app_link": "https://www.example.com",
                    "i18n": [
                        {
                            "i18n_key": "zh_cn",
                            "name": "名称"
                        }
                    ]
                },
                "plus_menu": {
                    "pc_app_link": "https://www.example.com",
                    "mobile_app_link": "https://www.example.com"
                }
            },
            "remark": {
                "remark": "备注说明",
                "update_remark": "更新说明",
                "visibility": {
                    "is_all": false,
                    "visible_list": {
                        "open_ids": [
                            {
                                "union_id": "on_8ed6aa67826108097d9ee143816345",
                                "user_id": "e33ggbyz",
                                "open_id": "ou_84aad35d084aa403a838cf73ee18467"
                            }
                        ],
                        "department_ids": [
                            "od-ddee42c0f8a948a5e650341e2153243b"
                        ]
                    },
                    "invisible_list": {
                        "open_ids": [
                            {
                                "union_id": "on_8ed6aa67826108097d9ee143816345",
                                "user_id": "e33ggbyz",
                                "open_id": "ou_84aad35d084aa403a838cf73ee18467"
                            }
                        ],
                        "department_ids": [
                            "od-ddee42c0f8a948a5e650341e2153243b"
                        ]
                    }
                }
            }
        },
        "app_status": 1
    }
}
```






### 事件订阅示例代码

事件订阅流程可参考：[事件订阅概述](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)，新手入门可参考：[教程](/document/uAjLw4CM/uMzNwEjLzcDMx4yM3ATM/develop-an-echo-bot/introduction)

:::html
<div style="margin-bottom: 4px;display: flex;column-gap: 4px;align-items: center;">
  <md-text type='field-name'>订阅方式</md-text>
  <md-tooltip>
    <ul class="md_render-table_solid md_render-table">
      <li><b>长连接方式（推荐）：</b>无需发布到公网地址，在本地开发环境中即可接收事件回调，且无需处理加解密逻辑。</li>
      <li><b>发送至开发者服务器：</b>需要提供服务器公网地址。</li>
    </ul>
  </md-tooltip>
</div>
:::

:::html
<md-code-tabs>
  <md-code-tab-group title="使用长连接接收事件">
	
    <md-code-tab-panel sdkType="golang-sdk">
package main

import (
	"context"
	"fmt"

	larkcore "github.com/larksuite/oapi-sdk-go/v3/core"
	larkevent "github.com/larksuite/oapi-sdk-go/v3/event"
	"github.com/larksuite/oapi-sdk-go/v3/event/dispatcher"
	"github.com/larksuite/oapi-sdk-go/v3/service/application/v6"
	larkws "github.com/larksuite/oapi-sdk-go/v3/ws"
)

// SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/golang-sdk-guide/preparations
func main() {
	// 注册事件 Register event
	eventHandler := dispatcher.NewEventDispatcher("", "").
		OnP2ApplicationAppVersionPublishApplyV6(func(ctx context.Context, event *larkapplication.P2ApplicationAppVersionPublishApplyV6) error {
			fmt.Printf("[ OnP2ApplicationAppVersionPublishApplyV6 access ], data: %s\n", larkcore.Prettify(event))
			return nil
		})

	// 构建 client Build client
	cli := larkws.NewClient("YOUR_APP_ID", "YOUR_APP_SECRET",
		larkws.WithEventHandler(eventHandler),
		larkws.WithLogLevel(larkcore.LogLevelDebug),
	)

	// 建立长连接 Establish persistent connection
	err := cli.Start(context.Background())

	if err != nil {
		panic(err)
	}
}

    </md-code-tab-panel>

    <md-code-tab-panel sdkType="python-sdk">
# SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/python--sdk/preparations-before-development
import lark_oapi as lark


def do_p2_application_application_app_version_publish_apply_v6(data: lark.application.v6.P2ApplicationApplicationAppVersionPublishApplyV6) -> None:
    print(f'[ do_p2_application_application_app_version_publish_apply_v6 access ], data: {lark.JSON.marshal(data, indent=4)}')

# 注册事件 Register event
event_handler = lark.EventDispatcherHandler.builder("", "") \
    .register_p2_application_application_app_version_publish_apply_v6(do_p2_application_application_app_version_publish_apply_v6) \
    .build()


def main():
    # 构建 client Build client
    cli = lark.ws.Client("APP_ID", "APP_SECRET",
                        event_handler=event_handler, log_level=lark.LogLevel.DEBUG)
    # 建立长连接 Establish persistent connection
    cli.start()

if __name__ == "__main__":
    main()

    </md-code-tab-panel>

    <md-code-tab-panel sdkType="java-sdk">

package com.example.sample;

import com.lark.oapi.core.utils.Jsons;
import com.lark.oapi.service.application.ApplicationService;
import com.lark.oapi.service.application.v6.model.P2ApplicationAppVersionPublishApplyV6;
import com.lark.oapi.event.EventDispatcher;
import com.lark.oapi.ws.Client;

// SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/java-sdk-guide/preparations
public class Sample {
    // 注册事件 Register event
    private static final EventDispatcher EVENT_HANDLER = EventDispatcher.newBuilder("", "")
            .onP2ApplicationAppVersionPublishApplyV6(new ApplicationService.P2ApplicationAppVersionPublishApplyV6Handler() {
                @Override
                public void handle(P2ApplicationAppVersionPublishApplyV6 event) throws Exception {
                    System.out.printf("[ onP2ApplicationAppVersionPublishApplyV6 access ], data: %s\n", Jsons.DEFAULT.toJson(event.getEvent()));
                }
            })
            .build();

    public static void main(String[] args) {
        // 构建 client Build client
        Client client = new Client.Builder("APP_ID", "APP_SECRET")
                .eventHandler(EVENT_HANDLER)
                .build();
        // 建立长连接 Establish persistent connection
        client.start();
    }
}
    </md-code-tab-panel>

    <md-code-tab-panel sdkType="nodejs-sdk">
// SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/nodejs-sdk/preparation-before-development
import * as Lark from '@larksuiteoapi/node-sdk';
const baseConfig = {
    appId: 'APP_ID',
    appSecret: 'APP_SECRET'
}
// 构建 client Build client
const wsClient = new Lark.WSClient(baseConfig);
// 建立长连接 Establish persistent connection
wsClient.start({
    // 注册事件 Register event
    eventDispatcher: new Lark.EventDispatcher({}).register({
        'application.application.app_version.publish_apply_v6': async (data) => {
            console.log(data);
        }
    })
});
    </md-code-tab-panel>

  </md-code-tab-group>
  <md-code-tab-group title="将事件推送至开发者服务器">
	
    <md-code-tab-panel sdkType="golang-sdk">
package main

import (
	"context"
	"fmt"
	"net/http"

	larkcore "github.com/larksuite/oapi-sdk-go/v3/core"
	"github.com/larksuite/oapi-sdk-go/v3/core/httpserverext"
	larkevent "github.com/larksuite/oapi-sdk-go/v3/event"
	"github.com/larksuite/oapi-sdk-go/v3/event/dispatcher"
	"github.com/larksuite/oapi-sdk-go/v3/service/application/v6"
)

// SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/golang-sdk-guide/preparations
func main() {
	// 注册事件 Register event
	eventHandler := dispatcher.NewEventDispatcher("", "").
		OnP2ApplicationAppVersionPublishApplyV6(func(ctx context.Context, event *larkapplication.P2ApplicationAppVersionPublishApplyV6) error {
			fmt.Printf("[ OnP2ApplicationAppVersionPublishApplyV6 access ], data: %s\n", larkcore.Prettify(event))
			return nil
		})

	// 创建路由处理器 Create route handler
	http.HandleFunc("/webhook/event", httpserverext.NewEventHandlerFunc(handler, larkevent.WithLogLevel(larkcore.LogLevelDebug)))

	err := http.ListenAndServe(":7777", nil)

	if err != nil {
		panic(err)
	}
}

    </md-code-tab-panel>

    <md-code-tab-panel sdkType="python-sdk">
# SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/python--sdk/preparations-before-development
from flask import Flask
from lark_oapi.adapter.flask import *
import lark_oapi as lark

app = Flask(__name__)


def do_p2_application_application_app_version_publish_apply_v6(data: lark.application.v6.P2ApplicationApplicationAppVersionPublishApplyV6) -> None:
    print(f'[ do_p2_application_application_app_version_publish_apply_v6 access ], data: {lark.JSON.marshal(data, indent=4)}')

# 注册事件 Register event
event_handler = lark.EventDispatcherHandler.builder("", "") \
    .register_p2_application_application_app_version_publish_apply_v6(do_p2_application_application_app_version_publish_apply_v6) \
    .build()


# 创建路由处理器 Create route handler
@app.route("/webhook/event", methods=["POST"])
def event():
    resp = event_handler.do(parse_req())
    return parse_resp(resp)

if __name__ == "__main__":
    app.run(port=7777)

    </md-code-tab-panel>

    <md-code-tab-panel sdkType="java-sdk">

package com.lark.oapi.sample.event;

import com.lark.oapi.core.utils.Jsons;
import com.lark.oapi.service.application.ApplicationService;
import com.lark.oapi.service.application.v6.model.P2ApplicationAppVersionPublishApplyV6;
import com.lark.oapi.sdk.servlet.ext.ServletAdapter;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

// SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/java-sdk-guide/preparations
@RestController
public class EventController {

    // 注册事件 Register event
    private static final EventDispatcher EVENT_HANDLER = EventDispatcher.newBuilder("verificationToken", "encryptKey")
            .onP2ApplicationAppVersionPublishApplyV6(new ApplicationService.P2ApplicationAppVersionPublishApplyV6Handler() {
                @Override
                public void handle(P2ApplicationAppVersionPublishApplyV6 event) throws Exception {
                    System.out.printf("[ onP2ApplicationAppVersionPublishApplyV6 access ], data: %s\n", Jsons.DEFAULT.toJson(event.getEvent()));
                }
            })
            .build();

    // 注入 ServletAdapter 实例 Inject ServletAdapter instance
    @Autowired
    private ServletAdapter servletAdapter;

    // 创建路由处理器 Create route handler
    @RequestMapping("/webhook/event")
    public void event(HttpServletRequest request, HttpServletResponse response)
            throws Throwable {
        // 回调扩展包提供的事件回调处理器 Callback handler provided by the extension package
        servletAdapter.handleEvent(request, response, EVENT_DISPATCHER);
    }
}
    </md-code-tab-panel>

    <md-code-tab-panel sdkType="nodejs-sdk">
// SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/nodejs-sdk/preparation-before-development
import http from 'http';
import * as lark from '@larksuiteoapi/node-sdk';

// 注册事件 Register event
const eventDispatcher = new lark.EventDispatcher({
    encryptKey: '',
    verificationToken: '',
}).register({
    'application.application.app_version.publish_apply_v6': async (data) => {
        console.log(data);
        return 'success';
    },
});

const server = http.createServer();
// 创建路由处理器 Create route handler
server.on('request', lark.adaptDefault('/webhook/event', eventDispatcher));
server.listen(3000);
    </md-code-tab-panel>

  </md-code-tab-group>
</md-code-tabs>
:::

---
document_id: '7395083758943436805'
directory_id: '7394378222823063557'
title: 卡片回传交互
full_path: /uAjLw4CM/ukzMukzMukzM/feishu-cards/card-callback-communication
breadcrumb:
- Developer Guides
- Message cards
- Configuring card callbacks
- Card callback communication (callback structure)
document_type: GuideDocumentType
updated_at: 2025-04-21T08:30:47Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-callback-communication
---

# 卡片回传交互

**卡片回传交互**作用于Lark卡片的 **请求回调** 交互组件。当终端用户点击Lark卡片上的回传交互组件后，你在开发者后台应用内注册的回调请求地址将会收到 **卡片回传交互** 回调。该回调包含了用户与卡片之间的交互信息。


你的业务服务器接收到回调请求后，需要在 3 秒内响应回调请求，声明通过弹出 Toast 提示、更新卡片、保持原内容不变等方式响应用户交互。

:::html
<md-alert type="tip">
本文档提供新版本的卡片回调结构和响应示例。开发平台 SDK 暂不支持新版卡片回调。了解旧版回调的 SDK 调用，参考[消息卡片回传交互（旧）](/document/ukTMukTMukTM/uYzM3QjL2MzN04iNzcDN/configuring-card-callbacks/card-callback-structure)。
</md-alert>
:::


:::html
<md-alert type="warn">
在卡片交互的场景下，lark 可能会下发除 `open.larksuite.com` 以外的域名。如果企业内设置了域名白名单，请同时添加 `*.feishu.cn` 域名为白名单。</md-alert>
:::
## 回调

:::html
<md-table>
  <md-thead>
  <tr>
      <md-th>基本信息</md-th>
      <md-th></md-th>
  </tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-th>回调类型</md-th>
      <md-td>card.action.trigger</md-td>
    </md-tr>
    <md-tr>
      <md-th>支持的应用类型</md-th>
      <md-td>
      <md-app-support types="custom,isv"></md-app-support>
      </md-td>
    </md-tr>
    <md-tr>
    <md-th>
            权限要求
            <md-tooltip type="info">订阅该事件所需的权限，开启其中任意一项权限即可订阅</md-tooltip>
            
            <div style="color: rgb(100, 106, 115);font-size: 12px;line-height: 20px;white-space: pre-line;font-weight: 500;padding-top: 4px;">开启任一权限即可</div>
            
    </md-th>
      <md-td>
暂无
      </md-td>
    </md-tr>
      <md-tr>
      <md-th>
          字段权限要求
      </md-th>
      <md-td>
<md-alert type="tip" icon="none">
        事件结构体中存在 `user_id` 敏感字段，仅当应用开启“获取用户 user ID”权限后才会返回。
        </md-alert>
        <md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm>
      </md-td>
    </md-tr>
    </md-tr>
    <md-tr>
      <md-th>推送方式</md-th>
      <md-td>
            <md-tag mode="inline" type="push-webhook" href="/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM" >Webhook</md-tag>
      </md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::

## 回调结构体

:::html
<md-dt-table>
<md-dt-thead>
<md-dt-tr>
<md-dt-th>字段</md-dt-th>
<md-dt-th>数据类型</md-dt-th>
<md-dt-th>描述</md-dt-th>
</md-dt-tr>
</md-dt-thead>
<md-dt-tbody>

<md-dt-tr level="0">
<md-dt-td>schema</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>回调的版本。固定取值为 `2.0`，为最新版本回调。了解旧版本回调，参考[消息卡片回传交互（旧）](/document/ukTMukTMukTM/uYzM3QjL2MzN04iNzcDN/configuring-card-callbacks/card-callback-structure)。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="0">
<md-dt-td>header</md-dt-td>
<md-dt-td>object</md-dt-td>
<md-dt-td>回调基本信息。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="1">
<md-dt-td>event_id</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>回调的唯一标识。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="1">
<md-dt-td>token</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>应用的 Verification Token。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="1">
<md-dt-td>create_time</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>回调发送的时间，接近回调发生的时间。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="1">
<md-dt-td>event_type</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>回调类型。卡片交互场景中，固定为 `"card.action.trigger"`。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="1">
<md-dt-td>tenant_key</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>应用归属的 tenant key，即租户唯一标识。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="1">
<md-dt-td>app_id</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>应用的 App ID。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="0">
<md-dt-td>event</md-dt-td>
<md-dt-td>object</md-dt-td>
<md-dt-td>回调的详细信息。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="1">
<md-dt-td>operator</md-dt-td>
<md-dt-td>object</md-dt-td>
<md-dt-td>回调触发者信息。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="2">
<md-dt-td>tenant_key</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>回调触发者的 tenant key，即租户唯一标识。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="2">
<md-dt-td>user_id</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>回调触发者的 user_id。了解不同的用户 ID，参见[用户身份概述](/document/home/user-identity-introduction/introduction)。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="2">
<md-dt-td>open_id</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>回调触发者的 open_id。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="1">
<md-dt-td>token</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>更新卡片用的凭证，有效期为 30 分钟，最多可更新 2 次。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="1">
<md-dt-td>action</md-dt-td>
<md-dt-td>object</md-dt-td>
<md-dt-td>交互信息。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="2">
<md-dt-td>value</md-dt-td>
<md-dt-td>object/ string</md-dt-td>
<md-dt-td>交互组件绑定的开发者自定义回传数据，对应组件中的 value 属性。类型为 string 或 object，可由开发者指定。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="2">
<md-dt-td>tag</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>交互组件的标签。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="2">
<md-dt-td>timezone</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>用户当前所在地区的时区。当用户操作日期选择器、时间选择器、或日期时间选择器时返回。</md-dt-td>
</md-dt-tr>
  
<md-dt-tr level="2">
<md-dt-td>name</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>组件的自定义唯一标识，用于识别内嵌在表单容器中的某个组件。</md-dt-td>
</md-dt-tr>
  
<md-dt-tr level="2">
<md-dt-td>form_value</md-dt-td>
<md-dt-td>object</md-dt-td>
<md-dt-td>表单容器内用户提交的数据。示例值：
```JSON
{
  "field name 1": [ // 表单容器内某多选组件的 name 和 value
    "selectDemo1",
    "selectDemo2"
  ], 
  "field name 2": "value 2", // 表单容器内某交互组件的 name 和 value
  "field name 3": "value 3", // 表单容器内某交互组件的 name 和 value
}
```
 </md-dt-td>
</md-dt-tr>
<md-dt-tr level="2">
<md-dt-td>input_value</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>当输入框组件未内嵌在表单容器中时，用户在输入框中提交的数据。
 </md-dt-td>
</md-dt-tr>
<md-dt-tr level="2">
<md-dt-td>option</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>当折叠按钮组、下拉选择-单选、人员选择-单选、日期选择器、时间选择器、日期时间选择器组件未内嵌在表单容器中时，用户选择该类组件某个选项时，组件返回的选项回调值。</md-dt-td>
</md-dt-tr>

</md-dt-tr>
<md-dt-tr level="2">
<md-dt-td>options</md-dt-td>
<md-dt-td>string[]</md-dt-td>
<md-dt-td>当下拉选择-多选组件和人员选择-多选组件未内嵌在表单容器中时，用户选择该类组件某个选项时，组件返回的选项回调值。</md-dt-td>
</md-dt-tr>
  
</md-dt-tr>
<md-dt-tr level="2">
<md-dt-td>checked</md-dt-td>
<md-dt-td>bool</md-dt-td>
<md-dt-td>当勾选器组件未内嵌在表单容器中时，勾选器组件的回调数据。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="1">
<md-dt-td>host</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>卡片展示场景。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="1">
<md-dt-td>delivery_type</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>卡片分发类型，固定取值为 `url_preview`，表示链接预览卡片。仅链接预览卡片有此字段。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="1">
<md-dt-td>context</md-dt-td>
<md-dt-td>object</md-dt-td>
<md-dt-td>展示场景上下文。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="2">
<md-dt-td>url</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>链接地址（适用于链接预览场景）。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="2">
<md-dt-td>preview_token</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>链接预览的 token（适用于链接预览场景）。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="2">
<md-dt-td>open_message_id</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>消息 ID。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="2">
<md-dt-td>open_chat_id</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>会话 ID。</md-dt-td>
</md-dt-tr>

</md-dt-tbody>
</md-dt-table>
:::

回调结构体示例

```json
{
    "schema": "2.0", // 回调的版本
    "header": { // 回调基本信息
        "event_id": "f7984f25108f8137722bb63c*****", // 回调的唯一标识
        "token": "066zT6pS4QCbgj5Do145GfDbbag*****", // 应用的 Verification Token
        "create_time": "1603977298000000",  // 回调发送的时间，接近回调发生的时间
        "event_type": "card.action.trigger", // 回调类型卡片交互场景中，固定为 "card.action.trigger"
        "tenant_key": "2df73991750*****", // 应用归属的 tenant key，即租户唯一标识
        "app_id": "cli_a5fb0ae6a4******" // 应用的 App ID
    },
    "event": { // 回调的详细信息
        "operator": {   // 回调触发者信息
            "tenant_key": "2df73991750*****", // 回调触发者的 tenant key，即租户唯一标识
            "user_id": "867*****", // 回调触发者的 user ID。当应用开启“获取用户 user ID”权限后，该参数返回
            "open_id": "ou_3c14f3a59eaf2825dbe25359f15*****" // 回调触发者的 Open ID
        },
        "token": "c-295ee57216a5dc9de90fefd0aadb4b1d7d******", // 更新卡片用的凭证，有效期为 30 分钟，最多可更新 2 次
        "action": { // 用户操作交互组件回传的数据
            "value": { // 交互组件绑定的开发者自定义回传数据，对应组件中的 value 属性。类型为 string 或 object，可由开发者指定。
                "key": "value"
            },
            "tag": "button", // 交互组件的标签
            "timezone": "Asia/Shanghai", // 用户当前所在地区的时区。当用户操作日期选择器、时间选择器、或日期时间选择器时返回
            "form_value": { // 表单容器内用户提交的数据
                "field name1": [ // 表单容器内某多选组件的 name 和 value
                    "selectDemo1",
                    "selectDemo2"
                ],
                "field name2": "value2", // 表单容器内某交互组件的 name 和 value
                "DatePicker_bpqdq5puvn4": "2024-04-01 +0800", // 表单容器内日期选择器组件的 name 和 value
                "DateTimePicker_ihz2d7a74i": "2024-04-29 07:07 +0800", // 表单容器内日期时间选择器组件的 name 和 value
                "Input_lf4fmxwfrd9": "1234", // 表单容器内输入框组件的 name 和 value
                "PersonSelect_2ejys7ype7m": "ou_3c14f3a59eaf2825dbe25359f15*****", // 表单容器内人员选择-单选组件的 name 和 value
                "Select_a2d5b7l3zd": "1", // 表单容器内下拉选择-单选组件的 name 和 value
                "TimePicker_7ecsf6xkqsq": "00:00 +0800" // 表单容器内时间选择器组件的 name 和 value
            },
            "name": "Button_lvkepfu3" // 用户操作交互组件的名称，由开发者自定义
        },
        "host": "im_message", // 卡片展示场景
        "delivery_type": "url_preview", // 卡片分发类型，固定取值为 url_preview，表示链接预览卡片仅链接预览卡片有此字段
        "context": { //  卡片展示场景相关信息
            "url": "xxx", // 链接地址（适用于链接预览场景）
            "preview_token": "xxx", // 链接预览的 token（适用于链接预览场景）
            "open_message_id": "om_574d639e4a44e4dd646eaf628e2*****", // 卡片所在的消息 ID
            "open_chat_id": "oc_e4d2605ca917e695f54f11aaf56*****" // 卡片所在的会话 ID
        }
    }
}
```

## 响应回调的结构体

你的业务服务器接收到回调请求后，需要在 3 秒内响应回调请求，声明通过弹出 Toast 提示、更新卡片、保持原内容不变等方式响应用户交互。
:::html
<md-dt-table>
<md-dt-thead>
<md-dt-tr>
<md-dt-th>字段</md-dt-th>
<md-dt-th>数据类型</md-dt-th>
<md-dt-th>是否必填</md-dt-th>
<md-dt-th>描述</md-dt-th>
</md-dt-tr>
</md-dt-thead>
<md-dt-tbody>

<md-dt-tr level="0">
<md-dt-td>toast</md-dt-td>
<md-dt-td>object</md-dt-td>
<md-dt-td>否</md-dt-td>
<md-dt-td>客户端的 Toast 弹窗提示。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="1">
<md-dt-td>type</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>否</md-dt-td>
<md-dt-td>弹窗提示的类型。可选值有：info、success、error、和 warning。

  不同的值的展示效果如下图所示：
![img_v3_02ao_9fdce3f7-5ba1-4f86-941f-2e5e7f6fd4eg.jpg](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e62145dca9a372b1b51f0ea2e2629160_y1gPzFePcx.jpg?height=844&lazyload=true&width=1280)
</md-dt-td>
</md-dt-tr>

<md-dt-tr level="1">
<md-dt-td>content</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>否</md-dt-td>
<md-dt-td>单语言提示文案。要配置多语言提示文案，请使用 `i18n` 字段。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="1">
<md-dt-td>i18n</md-dt-td>
<md-dt-td>Map</md-dt-td>
<md-dt-td>否</md-dt-td>
<md-dt-td>多语言提示文案。示例配置：
```json
{
  "i18n": {
    "zh_cn": "更新成功！",
    "en_us": "Successful update"
  }
}
```</md-dt-td>
</md-dt-tr>

<md-dt-tr level="2">
<md-dt-td>key</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>否</md-dt-td>
<md-dt-td>语言。可选值：
- `zh_cn`: 简体中文
- `en_us`: 英文
- `zh_hk`: 繁体中文（香港）
- `zh_tw`: 繁体中文（台湾）
- `ja_jp`: 日语
- `id_id`: 印尼语
- `vi_vn`: 越南语
- `th_th`: 泰语
- `pt_br`: 葡萄牙语
- `es_es`: 西班牙语
- `ko_kr`: 韩语
- `de_de`: 德语
- `fr_fr`: 法语
- `it_it`: 意大利语
- `ru_ru`: 俄语
- `ms_my`: 马来语

</md-dt-td>
</md-dt-tr>
<md-dt-tr level="2">
<md-dt-td>value</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>否</md-dt-td>
<md-dt-td>语言对应的文案。</md-dt-td></md-dt-tr>

<md-dt-tr level="0">
<md-dt-td>card</md-dt-td>
<md-dt-td>object</md-dt-td>
<md-dt-td>否</md-dt-td>
<md-dt-td>卡片数据。</md-dt-td>
</md-dt-tr>

<md-dt-tr level="1">
<md-dt-td>type</md-dt-td>
<md-dt-td>string</md-dt-td>
<md-dt-td>是</md-dt-td>
<md-dt-td>卡片类型。可选值：
- `template`：搭建工具构建的卡片模板
- `raw`：由 JSON 构建的卡片
</md-dt-td>
</md-dt-tr>

<md-dt-tr level="1">
<md-dt-td>data</md-dt-td>
<md-dt-td>object</md-dt-td>
<md-dt-td>是</md-dt-td>
<md-dt-td>卡片的数据。不同的卡片类型所需填写的字段不同。详情参考下文。</md-dt-td>
</md-dt-tr>

</md-dt-tbody>
</md-dt-table>
:::


- 当 `card.type` 字段的值为 `raw` 时，`card.data` 中需传入卡片 JSON 的数据。
- 当 `card.type` 字段的值为 `template` 时，`card.data` 中可传入的字段如下所示：
  :::html
  <md-dt-table>
  <md-dt-thead>
  <md-dt-tr>
  <md-dt-th>字段</md-dt-th>
  <md-dt-th>数据类型</md-dt-th>
  <md-dt-th>是否必填</md-dt-th>
  <md-dt-th>描述</md-dt-th>
  </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>
  <md-dt-tr level="0">
  <md-dt-td>template_id</md-dt-td>
  <md-dt-td>string</md-dt-td>
  <md-dt-td>是</md-dt-td>
  <md-dt-td>卡片模板的 ID，即卡片 ID。在卡片搭建工具中获取。</md-dt-td>
  </md-dt-tr>

  <md-dt-tr level="0">
  <md-dt-td>template_variable</md-dt-td>
  <md-dt-td>object</md-dt-td>
  <md-dt-td>否</md-dt-td>
  <md-dt-td>卡片模板的变量。格式为 `{key:value}`。详情参考[配置卡片变量](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/feishu-card-cardkit/configure-card-variables)中的**为卡片变量赋值**一节。</md-dt-td>
  </md-dt-tr>

  <md-dt-tr level="0">
  <md-dt-td>template_version_name</md-dt-td>
  <md-dt-td>string</md-dt-td>
  <md-dt-td>否</md-dt-td>
  <md-dt-td>卡片模版的版本。在卡片搭建工具中获取。</md-dt-td>
  </md-dt-tr>
  </md-dt-tbody>
  </md-dt-table>
  :::




响应体示例：

```json
{
    "toast":{
        "type":"info",
        "content":"卡片交互成功",
        "i18n":{
            "zh_cn":"卡片交互成功",
            "en_us":"card action success"
        }
    },
    "card":{
        "type":"raw",
        "data":{
            "config":{
                "enable_forward":true
            },
            "elements":[
                {
                    "tag":"div",
                    "text":{
                        "content":"This is the plain text",
                        "tag":"plain_text"
                    }
                }
            ],
            "header":{
                "template":"blue",
                "title":{
                    "content":"This is the title",
                    "tag":"plain_text"
                }
            }
        }
    }
}
```




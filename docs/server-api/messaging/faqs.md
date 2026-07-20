---
document_id: '7075603703986077702'
directory_id: '6924949741370621980'
title: 常见问题
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/guide/faq
breadcrumb:
- Server API
- Messaging
- FAQs
document_type: GuideDocumentType
updated_at: 2024-06-05T08:08:04Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/guide/faq
---

# 常见问题

本文列举出用户在使用「消息与群组」API 时可能会遇到的问题，帮助您快速解决问题。

## 开发前须知


**Q：我应该如何使用OpenAPI提供的开放能力？**

Lark的OpenAPI开放能力基于Restful接口对外提供服务，参见：[如何调用服务端API](/document/ukTMukTMukTM/uITNz4iM1MjLyUzM)。
<br>
<br>
<br>
**Q：企业自建应用和应用商店应用ISV的区别是什么？**

-   企业自建应用是指在同一个租户内可以使用的应用，通常情况下，Lark开放平台推荐使用企业自建应用来满足所需功能。
-   应用商店应用是指注册在应用商店中多个租户都可以使用的应用。创建应用商店应用需要单独进行申请，参见：[申请成为LarkISV](https://open.larksuite.com/isv/)。
:::note
在不同的租户中，appID和clientID相同，但是有不同的tenantKey用于获取tenantToken。不同租户中的botID不同，appID 和tenantKey可以确认一个唯一的botID。
:::

关于自建应用和商店应用更加详细的描述可参见： [自建应用与商店应用](/document/home/app-types-introduction/self-built-apps-and-store-apps)。
<br>
<br>
<br>
**Q：在将代码上到生产环境之前，我应该如何方便的进行调试？**

Lark的OpenAPI开放能力基于Restful接口对外提供服务，为了方便开发者快速体验和测试各类接口，我们提供了[API调试台](https://open.larksuite.com/api-explorer)。若接口调试无误，则参数填写正确。
<br>
<br>
<br>

# ID类问题

**Q：如何获取app_id和app_secret？**

1.  打开[Lark开放平台主页](https://open.larksuite.com/?lang=zh-CN)，点击我的后台-开发者后台，选择自己的机器人应用
1.  在左侧导航栏中点击「凭证与基础信息」，获取应用的 App ID 和 App Secret 信息

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/172e4d38b128fa8db7e626ad2c73e45c_UVzXMQt3GP.png?height=500&lazyload=true&width=1640)
<br>
<br>
<br>

**Q：如何获取群组ID（chat_id）?**

您可以选择通过以下 OpenAPI 接口获取 chat_id：

1.  [创建群接口](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/create)
1.  [搜索对用户或机器人可见的群列表接口](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/search)

具体使用场景和步骤可以参照 [群ID 说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)。

:::note
注意：[搜索对用户或机器人可见的群列表接口](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/search)不支持单聊，只支持群聊
:::
<br>
<br>
<br>


# 机器人相关问题

**Q：为什么我的机器人在Lark中搜不到？**

在申请机器人后，需要做以下两步才可以搜到：

1.  开启机器人能力

打开[Lark开放平台主页](https://open.larksuite.com/?lang=zh-CN)，点击右上角[开发者后台](https://open.larksuite.com/app)，选择自己的机器人应用，在左侧导航栏的应用功能-机器人标签页中，点击启用机器人能力

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1fdda889fcc5e364f593487f17a2d0d6_l71RqaJwxG.png?height=945&lazyload=true&width=1640)


2.  将应用发布上线，在应用发布上线成功就可以搜到了。

:::note
注意：请确认你选择了正确的机器人应用，以及正确的环境（BOE/Online/海外）
:::
<br>
<br>
<br>

**Q：提示机器人对某个用户不可见（the bot is invisible to the user），应该如何配置可见性？**

配置机器人对某个用户的可见性，是在应用后台对应用发布版本时进行配置的。

1.  打开[Lark开放平台主页](https://open.larksuite.com/?lang=zh-CN)，点击右上角[开发者后台](https://open.larksuite.com/app)，选择自己的机器人应用

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a7ecbe25bb8b792f231f7d053d5a929f_QzB9D7F1YL.png?height=1660&lazyload=true&width=3682)

2.  在左侧导航栏中，点击版本管理与发布，再点击右上方的创建版本


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3b9fb04ba56d38b3b38b444dd7607745_zcHXol9zlC.png?height=823&lazyload=true&width=1640)

3.  点击可用性状态的编辑，增加指定用户的可用性。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/aebbaaa69d44399f38a0d56446d95cb8_qEOPumy2yZ.png?height=877&lazyload=true&width=1640)

4.  点击保存，发布，等待审核通过上线完毕后可见性即可生效。
<br>
<br>
<br>

**Q：webhook自定义机器人是什么？ 我应该怎么通过webhook机器人发消息？**

群内**自定义机器人**与你开发的**机器人应用**有所不同：

- 自定义机器人只能用于在群聊中自动发送通知，不能响应用户@机器人的消息，不能获得任何的用户、租户信息。

- 自定义机器人可以被添加至外部群使用，机器人应用只能在内部群使用。

-   具体使用自定义机器人的方法请见[自定义机器人指南](/document/ukTMukTMukTM/ucTM5YjL3ETO24yNxkjN)

-   我们还提供了一系列机器人相关使用场景的教程，参见[消息与群组概述](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/introduction) 中的开发教程
<br>
<br>
<br>

**Q：在开发者后台配置了机器人，但是为什么我的机器人没有出现对话框？**

你的应用须配置了事件回调网址，才会出现对话框。
<br>
<br>
<br>

**Q：怎么实现机器人@人（@所有人、@指定人）？**

在机器人发送的普通文本消息（text）、富文本消息（post）、消息卡片（interactive）中，可以使用`at`标签实现@人效果。

具体请求示意如下：<br>

**(1) 在[普通文本消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)中@人、@所有人:**<br>

`at`标签示意

```json 
// at 指定用户
<at user_id="ou_xxx">Name</at> //可以填入open_id，union_id或user_id来at指定人
// at 所有人
<at user_id="all">所有人</at>
``` 

请求体中的`content` 示意：

```json 
{
	"content": " {\"text\":\"<at user_id=\\\"ou_xxxxxxxx\\\">Tom</at> text content\"}"
}
``` 
**(2) 在[富文本消息](/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/create_json#45e0953e)中@人、@所有人:**<br>

`at`标签示意

```json 
// at 指定用户
{
	"tag": "at",
	"user_id": "ou_xxxxxxx", //可以填入open_id，union_id或user_id来at指定人
	"user_name": "tom"
}

// at 所有人
{
	"tag": "at",
	"user_id": "all",//取值使用"all"来at所有人
	"user_name": "所有人"
}
``` 
请求体中的`content` 示意：

```json 
{
  "zh_cn": {
    "title": "我是一个标题",
    "content": [
      [
        {
          "tag": "text",
          "text": "第一行 :"
        },
        {
          "tag": "at",
          "user_id": "ou_xxxxxx",//可以填入open_id，union_id或user_id来at指定人
          "user_name": "tom"
        }
      ],
      [
        {
          "tag": "text",
          "text": "第二行:"
        },
        {
          "tag": "at",
          "user_id": "all",
          "user_name": "所有人"//取值使用"all"来at所有人
        }
      ]
    ]
  }
}
``` 
**(3) 在[消息卡片](/document/ukTMukTMukTM/uczM3QjL3MzN04yNzcDN)中@人、@所有人:**<br>
可以使用消息卡片[Markdown](/document/ukTMukTMukTM/uADOwUjLwgDM14CM4ATN)内容中的at人标签，标签示意：

```json 
// at 指定用户
	<at id=ou_xxx></at> //使用open_id at指定人
	<at id=b6xxxxg8></at> //使用user_id at指定人
	<at email=test@email.com></at> //使用邮箱地址 at指定人
// at 所有人
	<at id=all></at>
``` 

请求体中的`content` 示意：

```json 
{
  "config": {
    "wide_screen_mode": true
  },
  "header": {
    "title": {
      "tag": "plain_text",
      "content": "这是卡片标题内容"
    },
    "template": "blue"
  },
  "elements": [
    {
      "tag": "div",
      "text": {
        "content": "at所有人<at id=all></at> \nat指定人<at id=ou_xxxxxx></at>",
        "tag": "lark_md"
      }
    }
  ]
}
``` 
**注意：**
-   对于自定义机器人，请参考[自定义机器人使用指南](/document/ukTMukTMukTM/ucTM5YjL3ETO24yNxkjN)。
-   如果群主开启了`“仅群主和群管理员可@所有人”`配置，且机器人不是群主或管理员，则无法@所有人。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7679460f4e1b53b2eea22190e26f98f2_RVg0xRTvik.png?height=1029&lazyload=true&width=1640)

<br>
<br>
<br>



**Q：配置使用 webhook 的自定义机器人时，参数text是否有长度要求？**

建议 JSON 的长度不超过 30KB，序列化后的 Protobuf (PB) 长度不超过 100KB，图片须小于 10MB。
<br>
<br>
<br>

**Q：同一个 Custom Bot （自定义机器人）能在不同的群组使用吗？**

不能。Custom Bot （自定义机器人）仅能在单个群中使用。
<br>
<br>
<br>


# 消息相关问题

**Q：发消息的大小有限制吗？**

建议 JSON 的长度不超过 30k，序列化后的 pb 不超过 100k，图片最好小于 10MB。
<br>
<br>
<br>

**Q：已经推送出去的消息，能进行更改吗？**

- 消息卡片类型的消息：参考 [交互模块](/document/ukTMukTMukTM/uYjNwUjL2YDM14iN2ATN) 文档，查看如何更改

- 文本、富文本类型的消息：参考 [编辑消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/update) 文档

其他类型的消息暂不支持更改。
<br>
<br>
<br>

**Q：图片上传接口对图片的限制有哪些？**


- 图片上传格式支持：JPEG、PNG、WEBP、GIF、TIFF、BMP、ICO
- 图片输出格式支持：PNG、JPG、HEIC、WEBP、动态WEBP
- 上传图片的分辨率宽*高不超过200M，字节数不超过150MB
- 最大支持上传头像图片尺寸- 4096x4096
<br>
<br>
<br>

**Q：可以通过接口给外部联系人发送消息吗？**

暂不支持。
<br>
<br>
<br>



**Q：发消息时如何“@所有人”？**

可参见：[发送消息Content](/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/create_json)，按照下图中的提示，填入<at user_id="all"></at>即可。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2ddc898d078d3c6837d06723cb9df88b_7PALM0yxEr.png?height=1029&lazyload=true&width=1640)
<br>
<br>
<br>

**Q：获取消息中的资源文件接口，可以获取别人发送的消息中的资源文件吗?**

可以的，只要这条消息你可以看到即可以拉取对应的资源文件。
<br>
<br>
<br>

# 消息卡片相关问题



**Q：消息卡片能批量发给多人吗？**

支持。API参考[批量发送消息](/document/ukTMukTMukTM/ucDO1EjL3gTNx4yN4UTM)。<br>
注意：<br>
- 请求参数中`msg_type`为`interactive`，`content` 取值为`card:{}`结构体；
- 批量发送的消息卡片不支持更新、不支持回传交互。
<br>
<br>
<br>

**Q：消息卡片是否支持@单个成员？**

支持的，语法参看 [使用markdown标签](/document/ukTMukTMukTM/uADOwUjLwgDM14CM4ATN)。
<br>
<br>
<br>

**Q：消息卡片回调超时时间是多少?**

卡片回调超时时间为 **3s**，不支持自定义。如果业务方在 3s 内无法返回可以先返回{}，然后使用在回调中的 token [异步更新卡片](/document/ukTMukTMukTM/uMDO1YjLzgTN24yM4UjN)。
<br>
<br>
<br>



# 群组相关问题

**Q：搜索用户所在的群列表可以搜到外部群吗?**

[搜索对用户或机器人可见的群列表](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/search) 接口可以搜到外部群，「搜索用户所在的群列表」（历史版本）不会返回外部群。

推荐使用新版本，历史版本将不再维护。
<br>
<br>
<br>

# 事件相关问题

**Q: 为什么收到消息事件的内容为空？**

消息如果被撤回，则事件为空。
<br>
<br>
<br>

**Q: 新老版本事件有什么区别？**

新版本使用了新的协议，是老版本的升级版。

两个版本不兼容，老版本不会下线，但是不再迭代，推荐使用新版本。
<br>
<br>
<br>

# 错误排查

**Q：我在调用某个接口的时候，收到了错误码，错误码对应的错误原因是什么呢？**

1.  每一个具体的API接口文档中会有错误码的介绍和排查建议。
1.  所有接口的错误码罗列在了[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)里，也可以在此文档中搜索。
<br>
<br>
<br>

**Q：为什么我申请了权限，但是还是提示我没权限？**

1. 申请权限之后，需要进行应用版本发布，发布成功后权限才可以生效。
2. ISV应用的权限，除了需要发布应用外，还需要在[租户管理后台](https://admin.larksuite.com/)进行授权，才可以生效。
<br>
<br>
<br>

**Q：我在调用某个接口的时候触发了频率控制策略，具体的频控策略是怎样的呢？**

频控策略是指为了保障后台服务的稳定性，对接口增加一定的调用频率限制的策略。

具体的频控策略可见[频控策略 - 服务端文档](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN)。
<br>
<br>
<br>

**Q: 频控错误码对应的具体触发的频控策略？**

| 错误码      | 频控策略                  | 建议                                                                                        |
| -------- | --------------------- | ----------------------------------------------------------------------------------------- |
| 230020   | 发送消息（V1）接口触发群维度的发消息限流 | 单群发消息的QPS不超过5QPS，参考 [频控策略](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 11232    | 发消息接口（V4）触发消息维度的整体限流   | 等待后重试。                                                                                    |
| 11233    | 发消息接口（V4）触发群维度的发消息限流  | 单群发消息的QPS不超过5QPS，参考 [频控策略](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN) |
| 11247    | 批量发送消息触发每日额度限制        | 单个应用每天通过该接口发送的总消息条数不超过50万，请合理分配批量发送的额度。                                                   |
| 99991400 | 触发接口频控                | 请求过于频繁，请降低请求频次，调用频率不高于[频控策略](/document/ukTMukTMukTM/uUzN04SN3QjL1cDN)                                                                           |
<br>
<br>
<br>

**Q：在调用发送消息的API时，提示请求中的消息内容有问题，我应该如何排查解决问题？**

在文档[发送消息Content](/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/create_json)中介绍了富文本消息json转string的规则，这里明确总结如下：

1.  json整体添加" " 符号括起来；
1.  要使用 "`\`" 进行转义；
1.  消息内容中如果要实现换行功能，应在"\n"的基础上增加转义，参考[发送消息Content](/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/create_json#c9e08671)；

参考示例如下：

```json
{
"receive_id":"14f76322",
"msg_type": "post",
"content":"{\"zh_cn\":{\"title\":\"我是一个标题\",\"content\":[[{\"tag\":\"text\",\"text\":\"第一行:\"},{\"tag\":\"a\",\"href\":\"[http://www.larksuite.com](http://www.larksuite.com/)\",\"text\":\"超链接\"},{\"tag\":\"at\",\"user_id\":\"14f76322\",\"user_name\":\"tom\"}]]}}"
}
 
``` 

4.  在string中不能含有 [unicode](https://zh.wikipedia.org/wiki/Unicode)；
4.  建议使用[JSON压缩转义工具](https://www.sojson.com/yasuo.html)；
4.  如果想发送一些特殊字符，请使用Markdown的方式，使用方法见参见：[Markdown模块 - 通用能力 - 开发文档 - Lark开放平台](/document/ukTMukTMukTM/uADOwUjLwgDM14CM4ATN)。
<br>
<br>
<br>
<br>
# 其他

**Q：如何区分调用海外和国内的OpenAPI?**

调用的是国内还是海外的OpenAPI是通过域名来区分的。

如果想调用海外的OpenAPI，域名为https://open.larksuite.com/。

以[发送消息接口](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)为例：

-   CN -url: https://open.larksuite.com/open-apis/im/v1/messages
-   US -url: https://open.larksuite.com/open-apis/im/v1/messages
<br>
<br>
<br>


**Q：contentType应该设置为什么呢？我可以随意设置吗？**

请注意 contentType 需要设置为"application/json; charset=utf-8"，其他格式不保证支持。

<br>
<br>
<br>
<br>
**其他更多问题可见参见 ： [开发文档-常见问题](/document/ugTN1YjL4UTN24CO1UjN/uEjN1YjLxYTN24SM2UjN)**


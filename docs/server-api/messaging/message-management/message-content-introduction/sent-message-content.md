---
document_id: '7026663896463753221'
directory_id: '7002892512470679558'
title: 发送消息内容
full_path: /uAjLw4CM/ukTMukTMukTM/im-v1/message/create_json
breadcrumb:
- Server API
- Messaging
- Message management
- Message content introduction
- Sent message content
document_type: GuideDocumentType
updated_at: 2024-11-18T06:03:42Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/create_json
---

# 发送消息内容
本文将说明 [发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)、[回复消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/reply)、[编辑消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/update) 接口中**各类型消息的 content 字段应如何构造**。
:::note
- 本文中的示例代码，所有的receive_id, user_id, image_key, file_key等均为示例数据，开发者需要根据实际使用情况进行替换。
- 本文的内容构造示例仅适用于新版本[发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)、[回复消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/reply)、[编辑消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/update) 接口，**不适用于 [批量发送消息](/document/ukTMukTMukTM/ucDO1EjL3gTNx4yN4UTM) 和 历史版本接口**。
- 本文不适用于自定义机器人，自定义机器人请参考[自定义机器人使用指南](/document/ukTMukTMukTM/ucTM5YjL3ETO24yNxkjN)。
:::


## 请求体示例
Content 是 **string** 类型， JSON 结构需要**转义**。可以先构造一个结构体，然后使用 JSON 序列化转成 string 类型，也可以通过已有的网页 JSON 转换工具进行转义。

以文本类型消息为例，请求体示例如下：
```json 
{
    "receive_id": "ou_7d8a6e6df7621556ce0d21922b676706ccs",
    "content": "{\"text\":\" test content\"}",
    "msg_type": "text"
}
``` 


## 各类型消息 JSON 结构

### 文本 text

```json 
{
    "text": "test content"
}
``` 

#### **参数说明**

| 字段 | 类型 | 必须 | 描述 | 默认值 | 示例 |
| --- | --- | --- | --- | --- | --- |
| text | string | 是 | 文本内容 | 无 | test content |

**发送消息请求体示例**
```json 
{
    "receive_id": "ou_7d8a6e6df7621556ce0d21922b676706ccs",
    "content": "{\"text\":\" test content\"}",
    "msg_type": "text"
}
``` 



#### 换行符使用说明
:::note
如果需要文本中进行换行，需要增加转义
:::
#### 换行符使用示例
```json 
{
    "receive_id": "oc_xxx",
    "content": "{\"text\":\"firstline \\n second line  \"}",
    "msg_type": "text"
}
``` 




#### 文本消息@用法说明
:::note
- @单个用户时，`user_id`字段必须填入open_id，union_id 或 user_id来@指定人。**请确保 ID 为有效值**，ID 获取请参考[如何获取 User ID、Open ID 和 Union ID？](/document/home/user-identity-introduction/open-id) 
- @所有人 必须满足所在群开启@所有人功能
- 此处语法与消息卡片 [Markdown 模块](/document/ukTMukTMukTM/uADOwUjLwgDM14CM4ATN#abc9b025) @指定人的语法不同，请注意辨别
:::
```json 
 // @ 单个用户
<at user_id="ou_xxxxxxx">用户名（可不填）</at>
// @ 所有人
<at user_id="all"></at>
``` 
#### 文本消息@使用示例
```json 
{
    "receive_id": "oc_xxx",
    "content": "{\"text\":\"<at user_id=\\\"ou_xxxxxxx\\\">Tom</at> text content\"}",
    "msg_type": "text"
} 
```


***消息发送后的效果***

![va-1.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d2b81159f2dadb958fdde567422fccd1_ZwTWU2DnTL.png?height=257&lazyload=true&width=330)

***


#### 样式标签使用说明
:::note
- 支持加粗、斜体、下划线、删除线四种样式，可嵌套使用：
  - **加粗**: `<b>文本示例</b>`       
  - *斜体*: `<i>文本示例</i>`
  -  _下划线_ : `<u>文本示例</u>`
  - ~~删除线~~: `<s>文本示例</s>`
- 请保证首尾标签对应、嵌套正确，如有首尾标签缺失、嵌套层级错误等情况，会以原始内容发送消息。
- 标签信息会大幅增加消息体大小，请酌情使用。
- 该能力**暂不支持** 自定义机器人 和  [批量发送消息](/document/ukTMukTMukTM/ucDO1EjL3gTNx4yN4UTM)接口。
:::



#### 样式标签使用示例
```json 
{
    "receive_id": "oc_xxx",
    "content": "{\"text\":\"<b>bold content<i>, bold and italic content</i></b>\"}",
    "msg_type": "text"
}
``` 

#### 超链接使用说明
:::note
- 超链接的使用格式为`[文本](链接)`， 如`[Lark Open Platform](https://open.larksuite.com)` 。
- `[文本]`中不支持 [] 多层嵌套使用；此外，若文本中含有其他 '[' 或 ']' 字符，请确保前后符号匹配，否则可能导致超链接识别异常。
- 请确保链接是合法的，否则会以原始内容发送消息。
- 该能力**暂不支持** 自定义机器人 和  [批量发送消息](/document/ukTMukTMukTM/ucDO1EjL3gTNx4yN4UTM)接口。
:::


#### 超链接使用示例
```json 
{
    "receive_id": "oc_xxx",
    "content": "{\"text\":\"[Lark Open Platform](https://open.larksuite.com)\"}",
    "msg_type": "text"
}
``` 



### 富文本 post
:::note
- 富文本可以在一条消息中同时支持文字、At、图片、视频、超链接等元素。
- 一个富文本可分多个段落（由多个[]组成），每个段落可由多个元素组成，每个元素由tag和相应的字段描述组成。
- 图片、视频元素必须是独立的一个段落。
:::

::: warning
- 使用以下示例发送消息时，需压缩成一行，**二次转义**并替换掉其中的 user_id、image_key、file_key 等内容。
- `style` 字段暂不支持 自定义机器人 和 批量发送消息接口。
:::

```json 
{
	"zh_cn": {
		"title": "我是一个标题",
		"content": [
			[
				{
					"tag": "text",
					"text": "第一行:",
					"style": ["bold", "underline"]
                  
				},
				{
					"tag": "a",
					"href": "http://www.larksuite.com",
					"text": "超链接",
					"style": ["bold", "italic"]
				},
				{
					"tag": "at",
					"user_id": "ou_1avnmsbv3k45jnk34j5",
					"style": ["lineThrough"]
				}
			],
          	[{
				"tag": "img",
				"image_key": "img_7ea74629-9191-4176-998c-2e603c9c5e8g"
			}],
			[	
				{
					"tag": "text",
					"text": "第二行:",
					"style": ["bold", "underline"]
				},
				{
					"tag": "text",
					"text": "文本测试"
				}
			],
          	[{
				"tag": "img",
				"image_key": "img_7ea74629-9191-4176-998c-2e603c9c5e8g"
			}],
          	[{
				"tag": "media",
				"file_key": "file_v2_0dcdd7d9-fib0-4432-a519-41d25aca542j",
				"image_key": "img_7ea74629-9191-4176-998c-2e603c9c5e8g"
			}],
          	[{
				"tag": "emotion",
				"emoji_type": "SMILE"
			}],
			[{
					"tag": "hr"
			}],
			[{
					"tag": "code_block",
					"language": "GO",
					"text": "func main() int64 {\n    return 0\n}"
			}],
			[{
					"tag": "md",
					"text": "**mention user:**<at user_id=\"ou_xxxxxx\">Tom</at>\n**href:**[Open Platform](https://open.larksuite.com)\n**code block:**\n```GO\nfunc main() int64 {\n    return 0\n}\n```\n**text styles:** **bold**, *italic*, ***bold and italic***, ~underline~,~~lineThrough~~\n> quote content\n\n1. item1\n    1. item1.1\n    2. item2.2\n2. item2\n --- \n- item1\n    - item1.1\n    - item2.2\n- item2"
			}],
		]
	},
	"en_us": {
		...
	}
}
``` 
#### **富文本参数说明**

| 字段 | 类型 | 必须 | 描述 | 默认值 | 示例 |
| --- | --- | --- | --- | --- | --- |
| zh_cn, en_us | object | 是| `zh_cn`、`en_us`分别为富文本的中、英文配置。若不需要 i18N 消息，仅需要配置一种语言即可，但至少要包含一种语言的配置。| 无 | zh_cn |
| ∟title | string | 否 | 富文本消息的标题。 | 无 | "title" |
| ∟content | [][]node | 是 | 富文本消息内容，由多个段落组成，每个段落为一个 node 列表。支持的 node 标签类型及对应参数，参见下文：**富文本支持的标签和参数说明**。 | [] | [[{"tag": "text","text": "text content"}]] |

#### **富文本支持的标签和参数说明**

#### text

| 字段 | 类型 | 必须 | 描述 | 默认值 | 示例 |
| --- | --- | --- | --- | --- | --- |
| text | string | 是 | 文本内容。 | 无 | test content |
| un\_escape | bool | 否 | 表示是不是 unescape 解码，默认为 false ，不用可以不填。 | false | false |
| style | []string | 否 | 用于配置文本内容加粗、下划线、删除线和斜体样式，可选值分别为`bold`、`underline`、`lineThrough`与`italic`，非可选值将被忽略。 | [] | ["bold", "underline"] |

#### a

| 字段 | 类型 | 必须 | 描述 | 默认值 | 示例 |
| --- | --- | --- | --- | --- | --- |
| text | string | 是 | 文本内容。 | 无 | 超链接 |
| href | string | 是 | 默认的链接地址，请确保链接地址的合法性，否则消息会发送失败。 | 无 | https://open.larksuite.com |
| style | []string | 否 | 用于配置文本内容加粗、下划线、删除线和斜体样式，可选值分别为`bold`、`underline`、`lineThrough`与`italic`，非可选值将被忽略。 | [] | ["bold", "italic"] |

#### at

| 字段 | 类型 | 必须 | 描述 | 默认值 | 示例 |
| --- | --- | --- | --- | --- | --- |
| user\_id | string | 是 | 用户的open_id，union_id 或 user_id，请参考[如何获取 User ID、Open ID 和 Union ID？](/document/home/user-identity-introduction/open-id) <br> **注意**: @单个用户时，`user_id`字段必须是有效值；@所有人填"all"。| 无 | ou\_18eac85d35a26f989317ad4f02e8bbbb |
| style | []string | 否 | 用于配置文本内容加粗、下划线、删除线和斜体样式，可选值分别为`bold`、`underline`、`lineThrough`与`italic`，非可选值将被忽略。 | [] | ["lineThrough"] |

#### img
| 字段 | 类型 | 必须 | 描述 | 默认值 | 示例 |
| --- | --- | --- | --- | --- | --- |
| image\_key | string | 是 | 图片的唯一标识，可通过 [上传图片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create) 接口获取image_key。 | 无 | d640eeea-4d2f-4cb3-88d8-c964fab53987 |


#### media
| 字段 | 类型 | 必须 | 描述 | 默认值 | 示例 |
| --- | --- | --- | --- | --- | --- |
| file\_key | string | 是 | 视频文件的唯一标识，可通过 [上传文件](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/file/create) 接口获取file_key。 | 无 | file_v2_0dcdd7d9-fib0-4432-a519-41d25aca542j |
| image\_key | string | 否 | 视频封面图片的唯一标识，可通过 [上传图片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create) 接口获取image_key。 | 无 | img_7ea74629-9191-4176-998c-2e603c9c5e8g |


#### emotion
| 字段 | 类型 | 必须 | 描述 | 默认值 | 示例 |
| --- | --- | --- | --- | --- | --- |
| emoji_type | string | 是 | 表情类型，部分可选值请参见[表情文案](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-reaction/emojis-introduce)。 | 无 | SMILE |


#### code_block
| 字段 | 类型 | 必须 | 描述 | 默认值 | 示例 |
| --- | --- | --- | --- | --- | --- |
| language | string | 否 | 代码块语言，不填为文本类型，可选值有：PYTHON、C、CPP、GO、JAVA、KOTLIN、SWIFT、PHP、RUBY、RUST、JAVASCRIPT、TYPESCRIPT、BASH、SHELL、SQL、JSON、XML、YAML、HTML、THRIFT等，不区分大小写 | 无 | GO |
| text | string | 是 | 代码块内容 | 无 | func main() int64 {\n    return 0\n} |


#### hr

富文本支持 `tag` 为 `hr`，表示一条分割线。无其它参数。


#### md
:::warning
注意：
1. `md` 标签会独占一个或多个段落，不能与其他标签在一行中。
2. `md` 标签仅支持发送，获取消息内容时将不包含此标签，会根据 `md` 中的内容转换为其他标签。
3. 引用、有序、无序列表在获取消息时会退化成文本标签（text tag）进行输出。
:::

| 字段 | 类型 | 必须 | 描述 | 默认值 | 示例 |
| --- | --- | --- | --- | --- | --- |
| text | string | 是 | markdown 内容，支持的语法见下表 | 无 | 1. item1\n2. item2 |



| 语法 |  示例 | 使用说明 |
| --- | --- | --- | --- |
| @用户 | <at user_id="ou_xxxxx">李华</at> | 使用说明请参考本文“文本消息@用法说明”部分 |
| 超链接 |\[Lark Open Platform](https://open.larksuite.com) | 使用说明请参考本文“超链接使用说明”部分，若链接不合法，则只发送文本部分  |
| 有序列表 | 1. item1\n2. item2  |  1. 每项中，"."后要有一个空格； <br> 2. 每项须独立一行； <br>3. 多层级使用时，每个层级缩进 4 个空格，编号从 1. 开始；<br> 4. 可以与无序列表混合使用；  |
| 无序列表 | - item1\n- item2 | 1. 每项中，"-"后要有一个空格； <br> 2. 每项须独立一行； <br>3. 多层级使用时，每多一个层级须多缩进 4 个空格；<br> 4. 可以与有序列表混合使用，每个层级多缩进 4 个空格后重新以 1. 开始编号；    |
| 代码块 | \```GO\nfunc main(){\n    return\n}\n\``` | 代码块前后使用```包裹，前面的```后紧跟语言类型，支持 PYTHON、C、CPP、GO、JAVA、KOTLIN、SWIFT、PHP、RUBY、RUST、JAVASCRIPT、TYPESCRIPT、BASH、SHELL、SQL、JSON、XML、YAML、HTML、THRIFT等语言（不区分大小写） | 
| 引用 | > 引用 | ">"后要有一个空格
|分割线| \n --- \n| 前后需各有一个换行符
|加粗 | `**加粗文本**`| 1. **与文本间不能有空格； <br> 2. 文本部分不支持再解析其他组件，如文本部分为超链接，将不会被解析 <br> 3. 可以与斜体合用，如`***加粗+斜体***`
|斜体 | `*斜体文本*`| 1. *与文本间不能有空格； <br> 2. 文本部分不支持再解析其他组件，如文本部分为超链接，将不会被解析 <br> 3. 可以与斜体合用，如`***加粗+斜体***`
|下划线 | `~下划线~`| 1. ~与文本间不能有空格； <br> 2. 文本部分不支持再解析其他组件，如文本部分为超链接，将不会被解析 <br> 3. **不支持**与加粗、斜体、删除线合用
|删除线 | `~~删除线~~`| 1. ~~与文本间不能有空格； <br> 2. 文本部分不支持再解析其他组件，如文本部分为超链接，将不会被解析 <br> 3. **不支持**与加粗、斜体、删除线合用，如`~~~测试~~~`,`**~~加粗删除线~~**`

**发消息请求体示例**

```json 
{
	"receive_id": "oc_820faa21d7ed275b53d1727a0feaa917",
	"content": "{\"zh_cn\":{\"title\":\"我是一个标题\",\"content\":[[{\"tag\":\"text\",\"text\":\"第一行 :\"},{\"tag\":\"a\",\"href\":\"http://www.larksuite.com\",\"text\":\"超链接\"},{\"tag\":\"at\",\"user_id\":\"ou_1avnmsbv3k45jnk34j5\",\"user_name\":\"tom\"}],[{\"tag\":\"img\",\"image_key\":\"img_7ea74629-9191-4176-998c-2e603c9c5e8g\"}],[{\"tag\":\"text\",\"text\":\"第二行:\"},{\"tag\":\"text\",\"text\":\"文本测试\"}],[{\"tag\":\"img\",\"image_key\":\"img_7ea74629-9191-4176-998c-2e603c9c5e8g\"}]]}}",
	"msg_type": "post"
}
``` 

***富文本效果***

![va-2.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2498b35aa832b7b251b1411a9bbbed96_4PG2pjVXVR.png?height=461&lazyload=true&width=347)


***

### 图片 image

```json 
{
    "image_key": "img_7ea74629-9191-4176-998c-2e603c9c5e8g"
}
``` 
####  **参数说明** 

| 字段 | 类型 | 必须 | 描述 | 默认值 | 示例 |
| --- | --- | --- | --- | --- | --- |
| image_key | string | 是 | 图片Key，可通过 [上传图片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create) 接口获取image_key。 | 无 |img_7ea74629-9191-4176-998c-2e603c9c5e8g  |


#### **发消息请求体示例**

```json 
{
	"receive_id": "oc_xxx",
	"content": "{\"image_key\": \"img_v2_xxx\"}",
	"msg_type": "image"
} 
``` 

***消息发送后的效果***


![va-3.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4a9aee6c0748608c84b7eeeeb2e8bdad_VIh2iUbVWz.png?height=130&lazyload=true&width=317)

***

### 消息卡片 interactive
Lark 开放平台为消息卡片定义了结构化的组件与样式，你可以通过 JSON 描述定义样式精美、可交互的卡片内容，请参考[消息卡片介绍](/document/ukTMukTMukTM/uczM3QjL3MzN04yNzcDN)。

Lark 开放平台提供了可视化的[Lark 卡片搭建工具](https://open.larksuite.com/tool/cardbuilder)，你可以在该工具内构建、浏览、保存消息卡片，便于你灵活构建卡片模板。
:::note
注意：<br>
- 如果你使用的是历史版本的==发送消息卡片==(`/open-apis/message/v4/send/`)接口，请求体中的`content`参数需要换成`card`；新版[发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)接口中，消息请求体的内容参数已统一为`content`。
- 消息卡片的 `update_multi`（是否为共享卡片）字段在卡片内容的`config`结构体中设置。
:::

**发消息请求体示例**

- 使用卡片JSON发送
```json 
  {
      "receive_id": "oc_820faa21d7ed275b53d1727a0feaa917",
      "content": "{\"config\":{\"wide_screen_mode\":true},\"elements\":[{\"alt\":{\"content\":\"\",\"tag\":\"plain_text\"},\"img_key\":\"img_7ea74629-9191-4176-998c-2e603c9c5e8g\",\"tag\":\"img\"},{\"tag\":\"div\",\"text\":{\"content\":\"你是否曾因为一本书而产生心灵共振，开始感悟人生？\\n你有哪些想极力推荐给他人的珍藏书单？\\n\\n加入 **4·23 Lark 读书节**，分享你的**挚爱书单**及**读书笔记**，**赢取千元读书礼**！\\n\\n📬 填写问卷，晒出你的珍藏好书\\n😍 想知道其他人都推荐了哪些好书？马上[入群围观](https://open.larksuite.com/)\\n📝 用[读书笔记模板](https://open.larksuite.com/)（桌面端打开），记录你的心得体会\\n🙌 更有惊喜特邀嘉宾 4月12日起带你共读\",\"tag\":\"lark_md\"}},{\"actions\":[{\"tag\":\"button\",\"text\":{\"content\":\"立即推荐好书\",\"tag\":\"plain_text\"},\"type\":\"primary\",\"url\":\"https://open.larksuite.com/\"},{\"tag\":\"button\",\"text\":{\"content\":\"查看活动指南\",\"tag\":\"plain_text\"},\"type\":\"default\",\"url\":\"https://open.larksuite.com/\"}],\"tag\":\"action\"}],\"header\":{\"template\":\"turquoise\",\"title\":{\"content\":\"📚晒挚爱好书，赢读书礼金\",\"tag\":\"plain_text\"}}}",
      "msg_type": "interactive"
  } 
``` 
  
- 使用卡片模板`template_id`发送
  
```json
  {
      "receive_id": "ou_7d8a6exxxxccs",
      "msg_type": "interactive",
      "content": "{\"type\": \"template\", \"data\": { \"template_id\": \"ctp_xxxxxxxxxxxx\", \"template_variable\": {\"article_title\": \"这是文章标题内容\"} } }"
  }
```

####  **参数说明** 

| 字段 | 类型 | 必须 | 描述 | 示例值|
| --- | --- | --- | --- | --- |
| type | string | 是 | 固定值：`template`  |template
| data | object | 是 | 卡片模板数据 |  {}   |  
| ∟template_id | string |  是 | 卡片模板 ID，可在[Lark 卡片搭建工具](https://open.larksuite.com/cardkit)，我的卡片中，通过复制卡片 ID 获取 | AAqU6gaMabcef 
| ∟template_variable | object | 否 | 卡片中的变量数据，值为{key:value}形式，其中 key 表示变量名称。value 值表示变量的值 | {"key":"value"}



***消息发送后的效果***


![va-4.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5257cb14b8bd150a91fc3a85f7418868_K2YGyTfWcr.png?height=612&lazyload=true&width=686)


***

### 分享群名片 share_chat

```json 
{
    "chat_id": "oc_0dd200d32fda15216d2c2ef1ddb32f76"
}
``` 

 **参数说明** 

| 字段 | 类型 | 必须 | 描述 | 默认值 | 示例 |
| --- | --- | --- | --- | --- | --- |
| chat_id | string | 是 | 群ID，获取方式请参见[群ID 说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)。 | 无 |oc_0dd200d32fda15216d2c2ef1ddb32f76  |


**发消息请求体示例**

```json 
 {
	"receive_id": "oc_xxx",
	"content": "{\"chat_id\":\"oc_xxx\"}",
	"msg_type": "share_chat"
}
``` 
:::note
机器人必须在群名片所在的群内。
:::

***消息发送后的效果***


![va-5.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b1bb169851f062cea4b4b6fc5776b08d_taLm6HZJ4o.png?height=165&lazyload=true&width=364)


***


### 分享个人名片 share_user
:::note
- `user_id` 只支持[Open ID](/document/home/user-identity-introduction/open-id)，且机器人必须对该用户可见
- 暂不支持分享机器人的名片
:::

```json 
{
    "user_id": "ou_0dd200d32fda15216d2c2ef1ddb32f76"
} 
``` 

 **参数说明** 

| 字段 | 类型 | 必须 | 描述 | 默认值 | 示例 |
| --- | --- | --- | --- | --- | --- |
| user_id | string | 是 | 用户的[Open ID](/document/home/user-identity-introduction/open-id)，获取方式请参见[了解更多：如何获取 Open ID](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)。 | 无 |ou_0dd200d32fda15216d2c2ef1ddb32f76  |


**发消息请求体示例**

```json 
{
	"receive_id": "oc_820faa21d7ed275b53d1727a0feaa917",
	"content": "{\"user_id\":\"ou_xxx\"}",
	"msg_type": "share_user"
} 
``` 


***消息发送后的效果***

![va-6.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e956d315fae21c65d38865ddada9c85c_3hn8B5nN4X.png?height=199&lazyload=true&width=403)



***


### 语音 audio

```json 
{
    "file_key": "75235e0c-4f92-430a-a99b-8446610223cg"     //文件key
}
``` 
 **参数说明** 
| 字段 | 类型 | 必须 | 描述 | 默认值 | 示例 |
| --- | --- | --- | --- | --- | --- |
| file_key | string | 是 | 语音文件Key，可通过[上传文件](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/file/create)接口获取音频文件的 file_key。 | 无 |75235e0c-4f92-430a-a99b-8446610223cg  |
**发消息请求体示例**

```json 
{
	"receive_id": "oc_xxx",
	"content": "{\"file_key\":\"file_v2_xxx\"}",
	"msg_type": "audio"
} 
``` 

***消息发送后的效果***


![va-7.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6c28c9db94ada09b495a77d8fe876996_C44DOvJExR.png?height=162&lazyload=true&width=472)


***


### 视频 media
```json 
{
    "file_key": "75235e0c-4f92-430a-a99b-8446610223cg", //文件key
    "image_key": "img_xxxxxx"  // 视频封面图片key
}

``` 
 **参数说明** 
| 字段 | 类型 | 必须 | 描述 | 默认值 | 示例 |
| --- | --- | --- | --- | --- | --- |
| file_key | string | 是 | 视频文件Key，可通过[上传文件](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/file/create)接口获取视频文件的 file_key。 | 无 |75235e0c-4f92-430a-a99b-8446610223cg  |
| image_key | string | 否 | 视频封面图片Key，可通过[上传图片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create)接口获取图片的image_key。 | 无 |img_xxxxxx  |
**发消息请求体示例**

```json 
{
    "receive_id": "oc_xxx",
    "content": "{\"file_key\":\"file_v2_xxx\",\"image_key\":\"img_v2_xxx\"}",
    "msg_type": "media"
} 
``` 
***消息发送后的效果***

![va-8.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e926fa84cc9fb6ad847824d614a9f1bf_NRBMME9ryG.png?height=440&lazyload=true&width=472)



***


### 文件 file
```json 
{
    "file_key": "75235e0c-4f92-430a-a99b-8446610223cg"
}
``` 
 **参数说明** 
| 字段 | 类型 | 必须 | 描述 | 默认值 | 示例 |
| --- | --- | --- | --- | --- | --- |
| file_key | string | 是 | 文件Key，可通过[上传文件](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/file/create)接口获取文件的 file_key。 | 无 |75235e0c-4f92-430a-a99b-8446610223cg  |

**发消息请求体示例**
```json 
{
	"receive_id": "oc_820faa21d7ed275b53d1727a0feaa917",
	"content": "{\"file_key\":\"file_v2_xxx\"}",
	"msg_type": "file"
} 
``` 

***消息发送后的效果***


![va-9.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f79201c352df6bd57d6c394e8dcbde3b_w2S66SSY2e.png?height=162&lazyload=true&width=472)


***


### 表情包 sticker

```json 
{
    "file_key": "75235e0c-4f92-430a-a99b-8446610223cg"
}
``` 
 **参数说明** 
| 字段 | 类型 | 必须 | 描述 | 默认值 | 示例 |
| --- | --- | --- | --- | --- | --- |
| file_key | string | 是 | 表情包文件Key，目前仅支持发送机器人收到的表情包，可通过[接收消息事件](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/events/receive)的推送获取表情包 file_key。 | 无 |75235e0c-4f92-430a-a99b-8446610223cg  |

**发消息请求体示例**

```json 
{
	"receive_id": "oc_xxx",
	"content": "{\"file_key\":\"file_v2_xxx\"}",
	"msg_type": "sticker"
} 
``` 
***消息发送后的效果***


![va-10.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/96732790aa3de688750f9075c8f47492_KXXQLtdbY1.png?height=338&lazyload=true&width=379)


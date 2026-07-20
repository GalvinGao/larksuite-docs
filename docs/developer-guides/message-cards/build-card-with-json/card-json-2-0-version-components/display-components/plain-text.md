---
document_id: '7520565507028729868'
directory_id: '7520163178001563654'
title: 普通文本
full_path: /uAjLw4CM/ukzMukzMukzM/feishu-cards/card-json-v2-components/content-components/plain-text
breadcrumb:
- Developer Guides
- Message cards
- Build card with JSON
- Card JSON 2.0 version components
- Display components
- Plain text
document_type: GuideDocumentType
updated_at: 2025-06-27T09:52:37Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-json-v2-components/content-components/plain-text
---

# 普通文本组件

卡片的普通文本组件支持添加普通文本和前缀图标，并设置文本大小、颜色、对齐方式等展示样式。

本文档介绍普通文本组件的 JSON 2.0 结构，要查看历史 JSON 1.0 结构，参考[普通文本](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/card-components/content-components/plain-text)。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d44aee0423f960d0aeb0a309769e9cf1_LIYjk3GZZB.png?height=168&lazyload=true&maxWidth=400&width=559)

## JSON 结构

普通文本组件的完整 JSON 2.0 结构如下所示：
```json
{
  "schema": "2.0", // 卡片 JSON 结构的版本。默认为 1.0。要使用 JSON 2.0 结构，必须显示声明 2.0。
  "body": {
    "elements": [
      {
        "tag": "div",
        "element_id": "custom_id", // 操作组件的唯一标识。JSON 2.0 新增属性。用于在调用组件相关接口中指定组件。需开发者自定义。
        "margin": "0px 0px 0px 0px", // 组件的外边距，默认值 "0"。JSON 2.0 新增属性。支持范围 [-99,99]px。
        "width": "fill", // 文本宽度。JSON 2.0 新增属性。支持 "fill"、"auto"、"[16,999]px"。默认值为 fill。
        "text": { // 配置普通文本信息。
          "tag": "plain_text", // 文本类型的标签。可取值：plain_text 和 lark_md。
          "content": "", // 文本内容。当 tag 为 lark_md 时，支持部分 Markdown 语法的文本内容。
          "text_size": "normal", // 文本大小。默认值 normal。支持自定义在移动端和桌面端的不同字号。
          "text_color": "default", // 文本颜色。仅在 tag 为 plain_text 时生效。默认值 default。
          "text_align": "left", // 文本对齐方式。默认值 left。
          "lines": 2 // 内容最大显示行数，超出设置行的内容用 ... 省略。
        },
        "icon": {
          // 前缀图标。
          "tag": "standard_icon", // 图标类型。
          "token": "chat-forbidden_outlined", // 图标的 token。仅在 tag 为 standard_icon 时生效。
          "color": "orange", // 图标颜色。仅在 tag 为 standard_icon 时生效。
          "img_key": "img_v2_38811724" // 图片的 key。仅在 tag 为 custom_icon 时生效。
        }
      }
    ]
  }
}
```

## 字段说明

普通文本组件的字段说明如下表。 
:::html
<md-table>
<md-thead>
<md-tr>
<md-th style="width: 10%;">名称</md-th>
<md-th style="width: 10%;">必填</md-th>
<md-th style="width: 10%;">类型</md-th>
<md-th style="width: 10%;">默认值</md-th>
<md-th style="width: 50%;">说明</md-th>
</md-tr>
</md-thead>
<md-tbody>
<md-tr>
<md-td>
tag
</md-td>
<md-td>是</md-td>
<md-td>String</md-td>
<md-td>/</md-td>
<md-td>
组件的标签。普通文本组件的标签为 `div`。
</md-td>
</md-tr>
  
<md-tr>
      <md-td>element_id</md-td>
      <md-td>否</md-td>
      <md-td>String</md-td>
      <md-td>空</md-td>
      <md-td>操作组件的唯一标识。用于在调用[组件相关接口](/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card-element/create)中指定组件。在同一张卡片内，该字段的值全局唯一。仅允许使用字母、数字和下划线，必须以字母开头，不得超过 20 字符。</md-td></md-tr>
  
  
  <md-tr>
      <md-td>margin</md-td>
      <md-td>否</md-td>
      <md-td>String</md-td>
      <md-td>0</md-td>
      <md-td>组件的外边距。JSON 2.0 新增属性。值的取值范围为 [-99,99]px。可选值：
- 单值，如 "10px"，表示组件的四个外边距都为 10 px。
- 双值，如 "4px 0"，表示组件的上下外边距为 4 px，左右外边距为 0 px。使用空格间隔（边距为 0 时可不加单位）。
- 多值，如 "4px 0 4px 0"，表示组件的上、右、下、左的外边距分别为 4px，12px，4px，12px。使用空格间隔。</md-td>
    </md-tr>
  
  
  
  
    <md-tr>
      <md-td>width</md-td>
      <md-td>否</md-td>
      <md-td>String</md-td>
      <md-td>fill</md-td>
      <md-td>文本的宽度。JSON 2.0 新增属性。可取值：
- fill：文本的宽度将与组件宽度一致，撑满组件。
- auto：文本的宽度自适应文本内容本身的长度。
- [16,999]px：自定义文本宽度。</md-td>
    </md-tr>
  
  
<md-tr>
<md-td>
text
</md-td>
<md-td>否</md-td>
<md-td>Object</md-td>
<md-td>/</md-td>
<md-td>
配置卡片的普通文本信息。
</md-td>
</md-tr>
<md-tr>
<md-td>
└ tag
</md-td>
<md-td>是</md-td>
<md-td>String</md-td>
<md-td>plain_text</md-td>
<md-td>
文本类型的标签。可取值：
- `plain_text`：普通文本内容或[表情](https://www.feishu.cn/docx/doxcnG6utI72jB4eHJF1s5IgVJf)
- `lark_md`：支持部分 Markdown 语法的文本内容。详情参考下文 **lark_md 支持的 Markdown 语法**
  
**注意**：Lark卡片搭建工具中仅支持使用 `plain_text` 类型的普通文本组件。你可使用富文本组件添加 Markdown 格式的文本。
</md-td>
</md-tr>
<md-tr>
<md-td>
└ content
</md-td>
<md-td>是</md-td>
<md-td>String</md-td>
<md-td>/</md-td>
<md-td>
文本内容。当 `tag` 为 `lark_md` 时，支持部分 Markdown 语法的文本内容。详情参考下文 **lark_md 支持的 Markdown 语法**。
</md-td>
</md-tr>
<md-tr>
<md-td>
└ text_size
</md-td>
<md-td>否</md-td>
<md-td>String</md-td>
<md-td>normal</md-td>
<md-td>
文本大小。可取值如下所示。如果你填写了其它值，卡片将展示为 `normal` 字段对应的字号。你也可分别为移动端和桌面端定义不同的字号，详细步骤参考下文 **为移动端和桌面端定义不同的字号**。
- heading-0：特大标题（30px）
- heading-1：一级标题（24px）
- heading-2：二级标题（20 px）
- heading-3：三级标题（18px）
- heading-4：四级标题（16px）
- heading：标题（16px）
- normal：正文（14px）
- notation：辅助信息（12px）
- xxxx-large：30px
- xxx-large：24px
- xx-large：20px
- x-large：18px
- large：16px
- medium：14px
- small：12px
- x-small：10px
</md-td>
</md-tr>
<md-tr>
<md-td>
└ text_color
</md-td>
<md-td>否</md-td>
<md-td>String</md-td>
<md-td>default</md-td>
<md-td>
文本的颜色。仅在 `tag` 为 `plain_text` 时生效。可取值：
- `default`：客户端浅色主题模式下为黑色；客户端深色主题模式下为白色
- 颜色的枚举值。详情参考[颜色枚举值](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/enumerations-for-fields-related-to-color)
</md-td>
</md-tr>
<md-tr>
<md-td>
└ text_align
</md-td>
<md-td>否</md-td>
<md-td>String</md-td>
<md-td>left</md-td>
<md-td>
文本对齐方式。可取值：
- `left`：左对齐
- `center`：居中对齐
- `right`：右对齐
</md-td>
</md-tr>
<md-tr>
<md-td>
└ lines
</md-td>
<md-td>否</md-td>
<md-td>Int</md-td>
<md-td>/</md-td>
<md-td>
内容最大显示行数，超出设置行的内容用 `...` 省略。
</md-td>
</md-tr>
<md-tr>
<md-td>icon</md-td>
<md-td>否</md-td>
<md-td>Object</md-td>
<md-td>/</md-td>
<md-td>添加图标作为文本前缀图标。支持自定义或使用图标库中的图标。</md-td>
</md-tr>
<md-tr>
<md-td>└ tag</md-td>
<md-td>否</md-td>
<md-td>String</md-td>
<md-td>/</md-td>
<md-td>图标类型的标签。可取值：
- `standard_icon`：使用图标库中的图标。
- `custom_icon`：使用用自定义图片作为图标。</md-td>
</md-tr>
<md-tr>
<md-td>└ token</md-td>
<md-td>否</md-td>
<md-td>String</md-td>
<md-td>/</md-td>
<md-td>图标库中图标的 token。当 `tag` 为 `standard_icon` 时生效。枚举值参见[图标库](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/enumerations-for-icons)。</md-td>
</md-tr>
<md-tr>
<md-td>└ color</md-td>
<md-td>否</md-td>
<md-td>String</md-td>
<md-td>/</md-td>
<md-td>图标的颜色。支持设置线性和面性图标（即 token 末尾为 `outlined` 或 `filled` 的图标）的颜色。当 `tag` 为 `standard_icon` 时生效。枚举值参见[颜色枚举值](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/enumerations-for-fields-related-to-color)。</md-td>
</md-tr>
<md-tr>
<md-td>└ img_key</md-td>
<md-td>否</md-td>
<md-td>String</md-td>
<md-td>/</md-td>
<md-td>自定义前缀图标的图片 key。当 `tag` 为 `custom_icon` 时生效。
  
图标 key 的获取方式：调用[上传图片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create)接口，上传用于发送消息的图片，并在返回值中获取图片的 image_key。</md-td>
</md-tr>
:::
  
## 示例代码

### `plain_text` 类型示例

以下 JSON 2.0 结构的示例代码可实现如下图所示的卡片效果：
  
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2ae341e32cc2786194c2e6d261862879_Lm2WFn53rr.png?height=84&lazyload=true&maxWidth=400&width=619)
```json
{
  "schema": "2.0",
  "body": {
    "elements": [
      {
        "tag": "div",
        "text": {
          "tag": "plain_text",
          "content": "这是示例文本。",
          "text_size": "normal",
          "text_align": "center",
          "text_color": "default"
        },
        "icon": {
          "tag": "standard_icon",
          "token": "reply-cn_filled",
          "color": "blue"
        },
        "margin": "0px 0px 0px 0px",
        "element_id": "plaintext01"
      }
    ]
  }
}
```

### `lark_md` 类型示例

以下 JSON 2.0 结构的示例代码可实现如下图所示的卡片效果：
  
![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/caa68445ea80a561def27685da307427_CeCKgmYYAI.png?height=311&lazyload=true&maxWidth=400&width=618)
```json
{
  "schema": "2.0",
  "body": {
    "elements": [
      {
        "tag": "div",
        "text": {
          "tag": "plain_text",
          "content": "text-lark_md",
          "lines": 1
        },
        "fields": [
          {
            "is_short": false,
            "text": {
              "tag": "lark_md",
              "content": "<a>https://open.larksuite.com</a>"
            }
          },
          {
            "is_short": false,
            "text": {
              "tag": "lark_md",
              "content": "ready\nnew line"
            }
          },
          {
            "is_short": false,
            "text": {
              "tag": "lark_md",
              "content": "*Italic*"
            }
          },
          {
            "is_short": false,
            "text": {
              "tag": "lark_md",
              "content": "**Bold**"
            }
          },
          {
            "is_short": false,
            "text": {
              "tag": "lark_md",
              "content": "~~delete line~~"
            }
          },
          {
            "is_short": false,
            "text": {
              "tag": "lark_md",
              "content": "<at id=all></at>"
            }
          }
        ]
      }
    ]
  }
}
```

## `lark_md` 支持的 Markdown 语法

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th>能力</md-th>
      <md-th>语法</md-th>
      <md-th>效果</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>换行</md-td>
      <md-td>第一行\n第二行</md-td>
      <md-td>第一行

第二行</md-td>
    </md-tr>
    <md-tr>
      <md-td>斜体</md-td>
      <md-td>`*斜体*`</md-td>
      <md-td>*斜体*</md-td>
    </md-tr>
    <md-tr>
      <md-td>粗体</md-td>
      <md-td>`**粗体**` 或 `__粗体__`</md-td>
      <md-td>**粗体**</md-td>
    </md-tr>
    <md-tr>
      <md-td>删除线</md-td>
      <md-td>`~~删除线~~`</md-td>
      <md-td>~~删除线~~</md-td>
    </md-tr>
    <md-tr>
      <md-td>文字链接</md-td>
      <md-td>`[文字链接](https://www.larksuite.com)`</md-td>
      <md-td>[文字链接](https://www.larksuite.com)</md-td>
    </md-tr>
    <md-tr>
      <md-td>超链接</md-td>
      <md-td>`&lt;a href='https://open.larksuite.com'&gt;&lt;/a&gt;`</md-td>
      <md-td>[https://open.larksuite.com](https://open.larksuite.com/)</md-td>
    </md-tr>
    <md-tr>
      <md-td>@ 人</md-td>
      <md-td>&lt;at id=all&gt;
&lt;/at&gt;

&lt;at id={{open_id}}&gt;&lt;/at&gt;

&lt;at id={{user_id}}&gt;&lt;/at&gt;

&lt;at email=test@email.com&gt;&lt;/at&gt;
      
      
        
提示：了解如何获取 open_id 或 user_id，参考[如何获取不同的用户 ID](/document/home/user-identity-introduction/open-id)。
        
        
        
      </md-td>
      <md-td>@所有人

@test</md-td>
    </md-tr>
    <md-tr>
      <md-td>彩色文本</md-td>
      <md-td>&lt;font color=red&gt;红色&lt;/font&gt;
        
**提示**：要查看 color 枚举，参考[颜色枚举值](/document/uAjLw4CM/ukzMukzMukzM/feishu-cards/enumerations-for-fields-related-to-color)。</md-td>
      <md-td><span style="color: red;">红色</span></md-td>
    </md-tr>
    <md-tr>
      <md-td>emoji</md-td>
      <md-td>😁😢🌞💼🏆❌✅

**提示**：直接复制表情即可。了解更多 emoji 表情，参考 [Emoji 表情符号大全](https://www.feishu.cn/docx/doxcnG6utI72jB4eHJF1s5IgVJf)。
      </md-td>
      <md-td>😁😢🌞💼🏆❌✅</md-td>
    </md-tr>
    <md-tr>
      <md-td>Lark表情</md-td> <md-td>:OK:

**提示**：要查看表情枚举，参考[表情文案说明](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-reaction/emojis-introduce)。</md-td>
      <md-td>![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/14a7a076d1d02dc352915bf678f3f785_igT4IyBu6v.png?height=44&lazyload=true&width=54)</md-td>
    </md-tr>
    <md-tr>
      <md-td>标签</md-td>
      <md-td>`&lt;text_tag color='neutral'&gt; neutral &lt;/text_tag&gt;`
        
color 的枚举值有：neutral、blue、turquoise、lime、orange、violet、indigo、wathet、green、yellow、red、purple、carmine</md-td>
      <md-td>![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7f37d9bde5afa05511fce58f5fa8cab9_NGDoGSFVdr.png?height=646&lazyload=true&maxWidth=88&width=188)</md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::
  
  
## 为移动端和桌面端定义不同的字号

在普通文本和富文本组件的表头文本中，你可通过配置 `text_size` 为同一段文本定义在移动端和桌面端的不同字号。相关字段描述如下表所示。
:::html
<md-table>
<md-thead>
<md-tr>
<md-th style="width: 20%;">字段</md-th>
<md-th style="width: 10%;">是否必填</md-th>
<md-th style="width: 10%;">类型</md-th>
<md-th style="width: 10%;">默认值</md-th>
<md-th style="width: 50%;">说明</md-th>
</md-tr>
</md-thead>
<md-tbody>
<md-tr>
<md-td>
text_size
</md-td>
<md-td>否</md-td>
<md-td>Object</md-td>
<md-td>/</md-td>
<md-td>
文本大小。你可在此自定义移动端和桌面端的不同字号。
</md-td>
</md-tr>
<md-tr>
<md-td>
└ custom_text_size_name
</md-td>
<md-td>否</md-td>
<md-td>Object</md-td>
<md-td>/</md-td>
<md-td>
自定义的字号。你需自定义该字段的名称，如 `cus-0`、`cus-1` 等。
</md-td>
</md-tr>
<md-tr>
<md-td>
└└ default
</md-td>
<md-td>否</md-td>
<md-td>String</md-td>
<md-td>/</md-td>
<md-td>
在无法差异化配置字号的旧版Lark客户端上，生效的字号属性。建议填写此字段。可取值如下所示。
- heading-0：特大标题（30px）
- heading-1：一级标题（24px）
- heading-2：二级标题（20 px）
- heading-3：三级标题（18px）
- heading-4：四级标题（16px）
- heading：标题（16px）
- normal：正文（14px）
- notation：辅助信息（12px）
- xxxx-large：30px
- xxx-large：24px
- xx-large：20px
- x-large：18px
- large：16px
- medium：14px
- small：12px
- x-small：10px
</md-td>
</md-tr>
  <md-tr>
<md-td>
└└ pc
</md-td>
<md-td>否</md-td>
<md-td>String</md-td>
<md-td>/</md-td>
<md-td>
桌面端的字号。可取值如下所示。
- heading-0：特大标题（30px）
- heading-1：一级标题（24px）
- heading-2：二级标题（20 px）
- heading-3：三级标题（18px）
- heading-4：四级标题（16px）
- heading：标题（16px）
- normal：正文（14px）
- notation：辅助信息（12px）
- xxxx-large：30px
- xxx-large：24px
- xx-large：20px
- x-large：18px
- large：16px
- medium：14px
- small：12px
- x-small：10px
</md-td>
</md-tr>
  <md-tr>
<md-td>
└└ mobile
</md-td>
<md-td>否</md-td>
<md-td>String</md-td>
<md-td>/</md-td>
<md-td>
移动端的文本字号。可取值如下所示。

  **注意**：部分移动端的字号枚举值的具体大小与 PC 端有差异，使用时请注意区分。
- heading-0：特大标题（26px）
- heading-1：一级标题（24px）
- heading-2：二级标题（20 px）
- heading-3：三级标题（17px）
- heading-4：四级标题（16px）
- heading：标题（16px）
- normal：正文（14px）
- notation：辅助信息（12px）
- xxxx-large：26px
- xxx-large：24px
- xx-large：20px
- x-large：18px
- large：17px
- medium：14px
- small：12px
- x-small：10px
</md-td>
</md-tr>
  </md-tbody>
  </md-table>
:::
  
  
  具体步骤如下所示。
1. 在卡片 JSON 代码的全局行为设置中的 `config` 字段中，配置 `style` 字段，并添加自定义字号：

    ```json
    {
      "config": {
        "style": { // 在此添加并配置 style 字段。
          "text_size": { // 分别为移动端和桌面端添加自定义字号，同时添加兜底字号。用于在组件 JSON 中设置字号属性。支持添加多个自定义字号对象。
            "cus-0": {
              "default": "medium", // 在无法差异化配置字号的旧版Lark客户端上，生效的字号属性。选填。
              "pc": "medium", // 桌面端的字号。
              "mobile": "large" // 移动端的字号。
            },
            "cus-1": {
              "default": "medium", // 在无法差异化配置字号的旧版Lark客户端上，生效的字号属性。选填。
              "pc": "normal", // 桌面端的字号。
              "mobile": "x-large" // 移动端的字号。
            }
          }
        }
      }
    }
    ```
1. 在普通文本组件或富文本组件的 `text_size` 属性中，应用自定义字号。以下为在普通文本组件中应用自定义字号的示例：
   
    ```json
    {
      "i18n_elements": {
        "zh_cn": [
          {
            "tag": "column_set",
            "flex_mode": "none",
            "horizontal_spacing": "default",
            "background_style": "default",
            "columns": [
              {
                "tag": "column",
                "elements": [
                  {
                    "tag": "div",
                    "text": {
                      "tag": "plain_text",
                      "content": "这是一段普通文本示例。",
                      "text_size": "cus-0", // 在此处应用自定义字号。
                      "text_align": "center",
                      "text_color": "default"
                    },
                    "icon": {
                      "tag": "standard_icon",
                      "token": "app-default_filled",
                      "color": "blue"
                    }
                  }
                ],
                "width": "weighted",
                "weight": 1
              }
            ]
          }
        ]
      },
      "i18n_header": {}
    }
    ```

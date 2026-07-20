---
document_id: '6967331158356246534'
directory_id: '7394378222823014405'
title: 标题
full_path: /ukTMukTMukTM/ukTNwUjL5UDM14SO1ATN
breadcrumb:
- Developer Guides
- Message cards
- Component
- Content components
- Card Header
document_type: GuideDocumentType
updated_at: 2024-07-24T08:04:00Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukTNwUjL5UDM14SO1ATN
---

# 标题

本文介绍消息卡片搭建工具中的标题组件，以及该组件对应的 JSON 属性。

## 组件展示

在消息卡片搭建工具中，标题组件如下图所示。在工具内添加标题后，你可以编辑卡片的 **样式**、**内容**。

:::note
同一张卡片内仅支持添加一个标题。
:::

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5a03e7398088ef74ae5efa756d879092_6GgHMQnE6U.png?height=1398&lazyload=true&maxWidth=600&width=2882)

## 参数说明

标题组件的参数说明如下表。

:::html
<md-table>
    <md-thead>
    <md-tr>
        <md-th style="width: 15%">参数</md-th>
        <md-th style="width: 15%">是否必须</md-th>
        <md-th style="width: 15%">类型</md-th>
        <md-th style="width: 55%">描述</md-th>
    </md-tr>
    </md-thead>
    <md-tbody>
    <md-tr>
        <md-td>title</md-td>
        <md-td>是</md-td>
        <md-td>Object</md-td>
        <md-td>配置卡片的主标题信息。</md-td>
    </md-tr>
    <md-tr>
        <md-td>└ tag</md-td>
        <md-td>是</md-td>
        <md-td>String</md-td>
        <md-td>文本标识。固定取值：plain_text</md-td>
    </md-tr>
    <md-tr>
        <md-td>└ content</md-td>
        <md-td>否</md-td>
        <md-td>String</md-td>
        <md-td>卡片主标题内容。
<md-alert type="tip">
必须配置 `content` 或 `i18n` 两个属性的其中一个。如果同时配置仅生效 `i18n`。
</md-alert>
        </md-td>
    </md-tr>
    <md-tr>
        <md-td>└ i18n</md-td>
        <md-td>否
        </md-td>
        <md-td>\-</md-td>
        <md-td>
国际化文本内容，其中：
* zh_cn：简体中文
* en_us：英文
* ja_jp：日文
* zh_hk：繁体中文（中国香港）
* zh_tw：繁体中文（中国台湾）
          
示例配置：

```JSON
{
 "zh_cn": "这是主标题！",
 "en_us": "It is the title!"
}
```
<md-alert type="tip">
必须配置 `content` 或 `i18n` 两个属性的其中一个。如果同时配置仅生效 `i18n`。
</md-alert>
        </md-td>
    </md-tr>
    <md-tr>
        <md-td>subtitle</md-td>
        <md-td>否</md-td>
        <md-td>String</md-td>
        <md-td>配置卡片的副标题信息。
<md-alert type="tip">
- 不允许只配置副标题内容。如果只配置副标题，则实际展示为主标题效果。
- 副标题内容最多 1 行，超长文案末尾使用 `...` 进行省略。  
</md-alert>    
      </md-td>
    </md-tr>
    <md-tr>
        <md-td>└ tag</md-td>
        <md-td>是</md-td>
        <md-td>String</md-td>
        <md-td>文本标识。固定取值：plain_text</md-td>
    </md-tr>
    <md-tr>
        <md-td>└ content</md-td>
        <md-td>否</md-td>
        <md-td>String</md-td>
        <md-td>卡片副标题内容。
<md-alert type="tip">
必须配置 `content` 或 `i18n` 两个属性的其中一个。如果同时配置仅生效 `i18n`。
</md-alert>
        </md-td>
    </md-tr>
    <md-tr>
        <md-td>└ i18n</md-td>
        <md-td>否
        </md-td>
        <md-td>\-</md-td>
        <md-td>
国际化文本内容，其中：
* zh_cn：简体中文
* en_us：英文
* ja_jp：日文
* zh_hk：繁体中文（中国香港）
* zh_tw：繁体中文（中国台湾）
          
示例配置：

```JSON
{
 "zh_cn": "这是副标题！",
 "en_us": "It is the title!"
}
```
<md-alert type="tip">
必须配置 `content` 或 `i18n` 两个属性的其中一个。如果同时配置仅生效 `i18n`。
</md-alert>
        </md-td>
    </md-tr>
    <md-tr>
        <md-td>icon</md-td>
        <md-td>否</md-td>
        <md-td>Object</md-td>
        <md-td>该对象用于设置标题的前缀图标。一个卡片仅可配置一个标题图标。</md-td>
    </md-tr>
    <md-tr>
        <md-td>└ img_key</md-td>
        <md-td>否</md-td>
        <md-td>String</md-td>
        <md-td>前缀图标 key。
          
图标 key 的获取方式：调用[上传图片](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create)接口，上传用于发送消息的图片，并在返回值中获取图片的 `image_key`。</md-td>
    </md-tr>
    <md-tr>
        <md-td>template</md-td>
        <md-td>否</md-td>
        <md-td>String</md-td>
        <md-td>标题主题颜色。可选值与示例效果参见下文的 **标题样式表**。</md-td>
    </md-tr>
    <md-tr>
        <md-td>text_tag_list</md-td>
        <md-td>否</md-td>
        <md-td>TextTagList</md-td>
        <md-td>标题的标签属性。最多可配置 3 个标签内容，如果配置的标签数量超过 3 个，则取前 3 个标签进行展示。标签展示顺序与数组顺序一致。
<md-alert type="tip">
- 标题标签在Lark V6.11 及以上版本开始生效。在旧版本客户端内将不会展示标题标签内容。
- `text_tag_list` 和 `i18n_text_tag_list` 只能配置其中之一。如果同时配置两个字段，则优先生效国际化配置。
</md-alert> 
      </md-td>
    </md-tr>
    <md-tr>
        <md-td>└ tag</md-td>
        <md-td>否</md-td>
        <md-td>String</md-td>
        <md-td>标题标签的标识。固定取值：text_tag</md-td>
    </md-tr>
    <md-tr>
        <md-td>└ text</md-td>
        <md-td>否</md-td>
        <md-td>text</md-td>
        <md-td>标题标签的内容。基于[文本组件](/document/ukTMukTMukTM/uUzNwUjL1cDM14SN3ATN)的 `plain_text` 模式定义内容。

示例值：
```json
"text": {
          "tag": "plain_text",
          "content": "这里是标签"
        }          
```
      </md-td>
      </md-tr>
    <md-tr>
        <md-td>└ color</md-td>
        <md-td>否</md-td>
        <md-td>String</md-td>
        <md-td>标题标签的颜色，默认为蓝色（blue）。可选值与示例效果参见下文的 **标签样式表**。</md-td>
    </md-tr>
    <md-tr>
        <md-td>i18n_text_tag_list</md-td>
        <md-td>否</md-td>
        <md-td>\-</md-td>
        <md-td>标题标签的国际化属性。你可以在该字段内配置多语言环境的标签内容。每个语言环境下最多可配置 3 个标签内容，如果配置的标签数量超过 3 个，则取前 3 个标签进行展示。标签展示顺序与数组顺序一致。
<md-alert type="tip">
- 标题标签在Lark V6.11 及以上版本开始生效。在旧版本客户端内将不会展示标题标签内容。
- `text_tag_list` 和 `i18n_text_tag_list` 只能配置其中之一。如果同时配置两个字段，则优先生效国际化配置。
</md-alert>     
          
支持设置的多语言枚举值如下：
* zh_cn：简体中文
* en_us：英文
* ja_jp：日文
* zh_hk：繁体中文（中国香港）
* zh_tw：繁体中文（中国台湾）

该字段的取值格式如下：
          
```json
{
	"多语言枚举值": [
		text_tag_list 结构体   
	]
}     
```

示例值：
          
```json
// 如需使用该 JSON 示例，则注意需要手动清除 // 开头的注释
"i18n_text_tag_list": {
      "zh_cn": [ // 简体中文标签
        {
          "tag": "text_tag",
          "text": {
            "tag": "plain_text",
            "content": "标签内容"
          },
          "color": "carmine"
        }
      ],
      "en_us": [ // 英文标签
        {
          "tag": "text_tag",
          "text": {
            "tag": "plain_text",
            "content": "Tag content"
          },
          "color": "carmine"
        }
      ]
    }
```
      </md-td>
    </md-tr>
      
    </md-tbody>
</md-table>
:::

JSON 示例如下：

```json
// 如需使用该 JSON 示例，则注意需要手动清除 // 开头的注释
{
    "header": {	//在header中描述卡片的标题
        "title": {
            "tag": "plain_text",
            "content": "This is header" //卡片标题文案内容
        },
        "template":"red" //卡片标题的主题色
    }
}
```

## 标题样式表

消息卡片的标题样式由 `template` 参数的取值决定，详情参考下表。

:::note
Lark V5.0 及以后版本全局更新了 UI 风格，如果你在客户端查看的卡片样式与下表不一致，请升级至最新版Lark后重试。
:::

:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 20%">template 取值</md-th>
            <md-th>样式示例</md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-th>blue</md-th>
            <md-td>
                ![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/4d0f83f56efbca5707afc7be88d10a1c.png?height=158&lazyload=true&width=1228)
            </md-td>
        </md-tr>
        <md-tr>
            <md-th>wathet</md-th>
            <md-td>
                ![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/3109d23912af6698322428d61148520f.png?height=160&lazyload=true&width=1222)
            </md-td>
        </md-tr>
        <md-tr>
            <md-th>turquoise</md-th>
            <md-td>
                ![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/e0d699c79351025d325e29c20e913ad6.png?height=164&lazyload=true&width=1236)
            </md-td>
        </md-tr>
        <md-tr>
            <md-th>green</md-th>
            <md-td>
                ![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/d01737e9cb1cc87c41d29be9bd2d2eff.png?height=160&lazyload=true&width=1232)
            </md-td>
        </md-tr>
        <md-tr>
            <md-th>yellow</md-th>
            <md-td>
                ![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/76f98987b0eee9cdc03de1a293b12df9.png?height=160&lazyload=true&width=1222)
            </md-td>
        </md-tr>
        <md-tr>
            <md-th>orange</md-th>
            <md-td>
                ![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/a1406c23d26c8a72115de8b3914f6386.png?height=164&lazyload=true&width=1224)
            </md-td>
        </md-tr>
        <md-tr>
            <md-th>red</md-th>
            <md-td>
                ![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/672d05413780a0d47ba9817bbabf9a8d.png?height=160&lazyload=true&width=1222)
            </md-td>
        </md-tr>
        <md-tr>
            <md-th>carmine</md-th>
            <md-td>
                ![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/42da5c6ca8b01d3a2adaa9d8978b1736.png?height=166&lazyload=true&width=1218)
            </md-td>
        </md-tr>
        <md-tr>
            <md-th>violet</md-th>
            <md-td>
                ![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/8f518b18790d09e0d70bd377eac0b7e5.png?height=158&lazyload=true&width=1220)
            </md-td>
        </md-tr>
        <md-tr>
            <md-th>purple</md-th>
            <md-td>
                ![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/e6d4bae99bc68a0852cb7ede72d6c4de.png?height=152&lazyload=true&width=1224)
            </md-td>
        </md-tr>
        <md-tr>
            <md-th>indigo</md-th>
            <md-td>
                ![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/53d9b0ef62dbdffb830193a74f51cfb9.png?height=156&lazyload=true&width=1224)
            </md-td>
        </md-tr>
        <md-tr>
            <md-th>grey</md-th>
            <md-td>
                ![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/19be927a8a7f2ecc0dc05abdf64337ea.png?height=150&lazyload=true&width=1218)
            </md-td>
        </md-tr>
        <md-tr>
            <md-th>default</md-th>
            <md-td>
                ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/aa59f7c3b6ca9187dfdf61eb853b6968_UGHVXMriHW.png?height=258&lazyload=true&width=2422)
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 标签样式表

消息卡片标签（`text_tag_list` 或 `i18n_text_tag_list`）的颜色样式由 `color` 字段定义，该字段支持的枚举值与示例样式如下表。

| color 取值    | 颜色效果   |
| --------- | ------------|
| neutral   | ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/761076b831c55c5f94d2b56ebe8c04d9_boFNbhKcXF.png?height=68&lazyload=true&maxWidth=50&width=84) |
| blue      | ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/827883431ea1e65e5fcd73638cbb999a_MbMPxyyrOk.png?height=58&lazyload=true&maxWidth=50&width=92) |
| turquoise | ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c5f286dc2f7dc0b3a95b62f27bda62e9_jKFzxLthlQ.png?height=70&lazyload=true&maxWidth=50&width=96) |
| lime      | ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/27dc172e5609fdc99d2a7e96c4a06c6f_H3qhND1YnO.png?height=74&lazyload=true&maxWidth=50&width=86) |
| orange    | ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/de108549d5b5c764994b540ecbb1353c_bOvZTOz01a.png?height=78&lazyload=true&maxWidth=50&width=104) |
| violet    | ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6ff286b66137b37745bd2e04cb5fbd1a_OSSefuPkGe.png?height=80&lazyload=true&maxWidth=50&width=94) |
| indigo    | ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ec24fb4da8d29c525505ab9db678694c_AnZH9kvG05.png?height=74&lazyload=true&maxWidth=50&width=108) |
| wathet    | ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/92241eb6dc69a648dd13bbac2bfe81fb_90xOYjYIyr.png?height=62&lazyload=true&maxWidth=50&width=94) |
| green     | ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3c5c32b28dc75e5a6ca52f96d60e188a_j49N1b47Ba.png?height=78&lazyload=true&maxWidth=50&width=102) |
| yellow    | ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f576ac38660f0a85ad22d66bdcaab9a2_Fkq3gS9nrT.png?height=60&lazyload=true&maxWidth=50&width=100) |
| red       | ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/749a6988820ac78f7155f3f69bbc066b_tr649keDjG.png?height=66&lazyload=true&maxWidth=50&width=98) |
| purple    | ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e9aab9cb74258a506f66321fb085c144_v9vDFMTD65.png?height=88&lazyload=true&maxWidth=50&width=92) |
| carmine   | ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/99ff5fbe1d5d92fce54c99bd6fecd5c2_vQhT2fzLZ8.png?height=70&lazyload=true&maxWidth=50&width=100)

## 样式建议

彩色标题适合在群聊中使用，对于需高亮提醒的卡片信息，可将标题配置为应用的品牌色或表达状态的语义色，增强信息的视觉锚点。

在单聊中，建议根据卡片的状态不同，配置不同的语义颜色标题。不建议在单聊中，每张卡片无差别的使用同一种颜色的标题。这样既无法起到强调作用，也可能引起阅读者的视觉焦虑。

你可以参考以下示例，配置不同语义下的主题颜色。

* 绿色（green）代表完成/成功

	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/298b49c79be1483a8a912e9db616a6fc_iG926yH5Se.png?height=392&lazyload=true&maxWidth=600&width=3278)

* 橙色（orange）代表警告/警示

	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fe191beec3ab5642acf006b4ea084cef_Vo139Wq9Rj.png?height=396&lazyload=true&maxWidth=600&width=3278)

* 红色（red）代表错误/异常

	![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/c65920645f00a6c00edc8a16e8886e05.png?height=160&lazyload=true&maxWidth=600&width=1222)

* 灰色（grey）代表失效

	![](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/fcd937b264259f6b081f83f81018f93c.png?height=150&lazyload=true&maxWidth=600&width=1218)

## 常见问题

### 如何配置标题图标、副标题？

在搭建工具中，目前不支持对标题图标（`icon`）、副标题（`subtitle`）进行可视化编辑，你可以在 JSON 编辑器中设置好标题样式与内容，并在工具右上角点击 **向我发送预览**，在客户端内查看实际生效的卡片效果。以下提供了示例代码供你参考：

- 不包含多语言内容的卡片

  ```json
  // 如果你需要使用该 JSON 示例，则需要手动去除以 // 开头的注释内容
  {
    "header": {
      "icon": { //为卡片标题配置前缀的图标资源
        "img_key": "img_v2_718ccfc6-1180-41d7-a21d-1776dd053c8g"
      },
      "title": { //为卡片标题配置主标题文案内容
        "tag": "plain_text",
        "content": "这是title部分"
      },
      "subtitle": { //为卡片标题配置副标题文案内容
        "tag": "plain_text",
        "content": "这是subtitle部分"
      },
      "template": "red" //为卡片标题配置主题色
    },
    "elements": [
      {
        "tag": "markdown",
        "content": "普通文本\n*斜体*\n**粗体**\n~~删除线~~"
      }
    ]
  }
  ```

- 包含多语言内容的卡片

  ```json
  // 如果你需要使用该 JSON 示例，则需要手动去除以 // 开头的注释内容
  {
    "header": {
      "icon": { //为卡片标题配置前缀的图标资源
        "img_key": "img_v2_881d0c9c-8717-49a7-b075-1cca6562443g"
      },
      "title": { //为卡片标题配置多语言的主标题文案内容
        "tag": "plain_text",
        "i18n": {
          "en_us": "英语环境下的主标题",
          "ja_jp": "日语环境下的主标题",
          "zh_cn": "中文环境下的主标题"
        }
      },
      "subtitle": { //为卡片配置多语言的副标题文案内容
        "tag": "plain_text",
        "i18n": {
          "en_us": "英语环境下的副标题",
          "ja_jp": "日语环境下的副标题",
          "zh_cn": "中文环境下的副标题"
        }
      },
      "template": "red" //为卡片标题配置主题色
    },
    "elements": [
      {
        "tag": "markdown",
        "content": "普通文本\n*斜体*\n**粗体**\n~~删除线~~"
      }
    ]
  }
  ```

### 如何配置标题标签？

在搭建工具内，目前不支持对标题标签（`text_tag_list` 或 `i18n_text_tag_list`）进行可视化编辑，也不支持通过 **向我发送预览** 按钮查看实际效果，仅支持通过编写 JSON 的方式定义标题标签。因此，你可以手动编辑卡片 JSON，然后通过开放平台提供的 OpenAPI （例如[发送消息](/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create)接口）发送消息卡片进行测试。

以下提供了示例代码供你参考：

- 不包含多语言内容的卡片

  ```json
  {
    "header": {
      "text_tag_list": [
        {
          "tag": "text_tag",
          "text": {
            "tag": "plain_text",
            "content": "标题标签"
          },
          "color": "carmine"
        }
      ],
      "title": {
        "tag": "plain_text",
        "content": "主标题"
      },
      "subtitle": {
        "tag": "plain_text",
        "content": "副标题"
      },
      "template": "red"
    },
    "elements": [
      {
        "tag": "div",
        "text": {
          "content": "正文内容",
          "tag": "plain_text"
        }
      }
    ]
  }
  ```

  效果如下：

  ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/bca54c586bd39064e945163861ab6b25_3HDcbBBGSL.png?height=264&lazyload=true&maxWidth=600&width=1318)

- 包含多语言内容的卡片

  ```json
  {
    "header": {
      "i18n_text_tag_list": {
        "zh_cn": [
          {
            "tag": "text_tag",
            "text": {
              "tag": "plain_text",
              "content": "标题标签"
            },
            "color": "carmine"
          }
        ],
        "en_us": [
          {
            "tag": "text_tag",
            "text": {
              "tag": "plain_text",
              "content": "tagDemo"
            },
            "color": "carmine"
          }
        ]
      },
      "title": {
        "tag": "plain_text",
        "i18n": {
          "zh_cn": "主标题",
          "en_us": "title"
        }
      },
      "subtitle": {
        "tag": "plain_text",
        "i18n": {
          "zh_cn": "副标题",
          "en_us": "subtitle"
        }
      },
      "template": "red"
    },
    "i18n_elements": {
      "zh_cn": [
        {
          "tag": "div",
          "text": {
            "content": "正文内容",
            "tag": "plain_text"
          }
        }
      ],
      "en_us": [
        {
          "tag": "div",
          "text": {
            "content": "textDemo",
            "tag": "plain_text"
          }
        }
      ]
    }
  }
  ```

  效果如下：

  - 简体中文环境


  	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/cc9f4f29ccf88f7905f5fe63c11c09f8_mghGs9gULN.png?height=264&lazyload=true&maxWidth=600&width=1334)

  - 英文环境


  	![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fba969f8187f5f3c7f0dace4f6735d2a_2BiPb9RqGA.png?height=268&lazyload=true&maxWidth=600&width=1334)


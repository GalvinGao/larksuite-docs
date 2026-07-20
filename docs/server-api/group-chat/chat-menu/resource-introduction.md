---
document_id: '7197676707037642758'
directory_id: '7186934204960391174'
title: 资源介绍
full_path: /uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-menu_tree/overview
breadcrumb:
- Server API
- Group Chat
- Chat menu
- Resource introduction
document_type: GuideDocumentType
updated_at: 2024-06-05T08:09:18Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-menu_tree/overview
---

# 资源介绍
## 群菜单简介
菜单分为一级菜单(chat_menu_top_level)和二级菜单(chat_menu_second_level)。一个群内最多有3个一级菜单，每个一级菜单可以存在0到5个二级菜单。


| 名称         | 描述        |
| --------- | --------------- | -------   | ----------- | --------- |
|`menu_tree` | 菜单树，由若干一级菜单组成。 |
|`chat_menu_top_level` | 一级菜单，一个群内最多有3个一级菜单，每个一级菜单可以存在0到5个二级菜单。下图①是带二级菜单的一级菜单，下图②是不带二级菜单的一级菜单。 |
|`chat_menu_second_level` | 二级菜单，依附于一级菜单。下图③是依附于一级菜单的若干二级菜单。|
|`chat_menu_item` | 菜单元信息，一级菜单和二级菜单共用的用来描述元信息的结构体。 |

![20221208-111413.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b1dd1209d8cb73e4488d689920267364_0eF9zN4ZhW.png?lazyload=true&width=750&height=1624)

## 群菜单字段介绍
**chat_menu_item字段介绍**
| 名称         | 描述        |
| --------- | --------------- | -------   | ----------- | --------- |
|`action_type` | 菜单类型，有NONE、REDIRECT_LINK两种类型。一般情况都填充REDIRECT_LINK两种类型，仅一级菜单存在二级菜单时，该一级菜单设置NONE类型。 |
|`redirect_link` | 跳转链接。 |
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`∟ common_url` | 公用跳转链接，必须以http开头。 |
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`∟ ios_url` | IOS端跳转链接，当该字段不设置时，IOS端会使用common_url。必须以http开头。 |
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`∟ android_url` | Android端跳转链接，当该字段不设置时，Android端会使用common_url。必须以http开头。 |
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`∟ pc_url` | 	PC端跳转链接，当该字段不设置时，PC端会使用common_url。必须以http开头。 |
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`∟ web_url` | 	Web端跳转链接，当该字段不设置时，Web端会使用common_url。必须以http开头。 |
|`image_key` | image_key，群菜单小图标。 |
|`name` | 名称，一级菜单名称字符数要在1到8范围内，二级菜单名称字符数要在1到24范围内。一个中文字符占两个字符数。 |
|`i18n_names` | 国际化名称，一级菜单名称字符数要在1到8范围内，二级菜单名称字符数要在1到24范围内。一个中文字符占两个字符数。 |
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`∟ zh_cn` | 中文名|
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`∟ en_us` | 英文名 |
|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`∟ ja_jp` | 日文名 |

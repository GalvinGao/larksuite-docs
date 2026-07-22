---
document_id: '6967331158355116038'
directory_id: '7073460768595197957'
title: AppLink的结构
full_path: /uYjL24iN/ucjN1UjL3YTN14yN2UTN
breadcrumb:
- Developer Guides
- AppLink Protocol
- AppLink introduction
- AppLink structure
document_type: GuideDocumentType
updated_at: 2022-07-05T09:06:36Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucjN1UjL3YTN14yN2UTN
---

#  AppLink的结构


AppLink 协议 就是一个 URL 协议。AppLink 协议可以用于打开Lark或者其中的一个功能。


尝试一下：通过Applink打开Lark中的日历 [https://applink.larksuite.com/client/calendar/open](https://applink.larksuite.com/client/calendar/open)

目前可通过Applink可打开的Lark功能 请参考 [已支持的协议](/document/uAjLw4CM/uYjL24iN/applink-protocol/supported-protocol/open-lark) 

##  AppLink协议的结构

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/028c8f310ae89a947ef909294dce3052_gpFokhERsL.png?lazyload=true&width=1160&height=179)


| 字段         | 值                               | 说明                      | 
| --------- | --------------------- | -----------------   | 
|**scheme** |https| 固定值 | 
|**host** |applink.larksuite.com| 固定值 | 
|**path** |对应不同的协议| 例如：/client/op/open 打开Lark， /client/mini_program/open 打开小程序， /client/chat/open  打开聊天页面 | 
|**query** |协议参数，不同的协议有不同的定义| 例如：key1=value1&key2=value2 ，所有参数key 和 value 都要进行 encodeURIComponent | 


## AppLink协议的能力
- 在Lark内打开AppLink协议，则会直接跳转到Lark中对应的功能；
- 在Lark外部（如浏览器内）打开Applink协议，为了避免用户未安装Lark而无法唤起对应功能，会先打开**Applink网页**（见下图），提示用户下载Lark 或 点击打开Lark对应的功能。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5c9d55fc0469068c5e8674581ba44231_Hga7cYznrT.png?lazyload=true&width=2156&height=1398)


<BR>
AppLink还支持以下参数，实现打开方式的配置：

### 1. 支持对低版本Lark进行兼容
在Lark内使用 AppLink 时，如果Lark如果当前Lark版本低于设置的最小Lark版本时，将打开 **Applink网页**，提示版本过低，需要升级Lark版本后使用。

#### 参数
| 字段         | 必填           | 说明        | 
| --------- | --------------- | -------   | 
|min_lk_ver | 否         | 指定 AppLink 协议能够兼容的最小Lark版本，使用三位版本号 x.y.z。如果当前Lark版本号小于min_lk_ver，打开该 AppLink 会显示为兼容页面。 | 


::: note 
也可使用 min_lk_ver_android、min_lk_ver_ios、min_lk_ver_pc 参数对不同的客户端指定不同的版本。
::: 
#### Applink示例
[https://applink.larksuite.com/client/op/open?min_lk_ver=5.10.0](https://applink.larksuite.com/client/op/open?min_lk_ver=5.10.0)


###  2. 指定打开Lark客户端，而不打开品牌定制客户端、私有化客户端
在Lark外部 通过 **Applink 网页** 打开Lark时，可设置 `lk_unique = true` 参数，则指定打开Lark客户端，而不打开品牌定制客户端与私有化客户端。
#### 参数

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| lk_unique | boolean | 否 | false | **true**：指定打开Lark客户端，而不打开品牌定制客户端与私有化客户端。<br>**false**：可打开Lark / 品牌定制客户端 / 私有化客户端；如果用户安装了多个客户端，则随机打开其中一个。<br><md-alert type="tip" icon="none"><br>Lark V5.12.0 及以上版本支持。如果设为ture，低版本Lark客户端将无法唤起。<br></md-alert> |

#### Applink示例
[https://applink.larksuite.com/client/op/open?lk_unique=true](https://applink.larksuite.com/client/op/open?lk_unique=true)


###  3. 支持以降级协议打开
在Lark外部使用以 `lark` 代替 `https` 的协议，例如 [lark://applink.larksuite.com/client/op/open](lark://applink.larksuite.com/client/op/open)，会尝试直接打开Lark，但是当无法打开Lark时不会自动跳转对应的 **AppLink 网页**。

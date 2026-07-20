---
document_id: '7260082411118067717'
directory_id: '7258197168736534534'
title: 多维表格开放能力简介
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-extension-introduction
breadcrumb:
- Developer Guides
- Develop Base Extensions
- Base Extension introduction
document_type: GuideDocumentType
updated_at: 2023-07-27T02:36:41Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-extension-introduction
---

# 多维表格开放能力简介


**Lark多维表格** **是先进团队的业务管理工具，是一个表格，也是无数个应用，千人千面，万人万解。**

基于多维表格灵活易用的基础能力，大量的用户使用多维表格搭建了各种大大小小的系统，例如项目管理系统、客户管理系统、巡检系统，与此同时，用户也对多维表格提出了更多扩展功能的需求；多维表格团队在加快迭代尽力满足客户需求的同时，也将产品开放出来，欢迎广大的ISV开发者、企业IT开发人员加入我们一起拓展多维表格的能力。
在多维表格开放生态，开发者能够：
- 基于强大的拓展能力，尽情的发挥想象力，开发出创新应用。
- 和多维表格产品技术团队一起探讨，共同进步。
- 企业IT开发者能够通过开发企业自建应用，快速扩展多维表格的能力，拓展应用场景。
- ISV开发者有机会触及到海量的客户，拿到真实的业务需求；也能协同Lark商业化团队，更好更快的售卖产品。
  

## 多维表格开放模式

1. **多维表格提供什么样的开放能力？**
我们提供如下两种开放能力：OpenAPI 和 多维表格插件。

    - **OpenAPI**

    开发者可以通过 OpenAPI 控制一个多维表格的元数据定义、读写数据、控制视图、控制权限。

    详见：[概述 - 服务端文档 - 开发文档 - Lark开放平台](/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/bitable-overview)。

    - **多维表格插件**；更激进的开放，我们提供了：


      - 多个开放点位，让开发者直接在多维表格中嵌入自己开发的产品。

      -	插件市场和模板复制分发的能力，方便开发者插件的引流和复用。

2. **我该使用哪种开放能力?**
    - 如果您想实现自有系统和多维表格的数据连通，或者想对一个多维表格进行基础的数据操作，您可以尝试OpenAPI。
    - 如果您想在多维表格上开发一个新功能，尤其是希望您的功能能够被分发，被更多人使用，我们强烈建议您尝试多维表格插件。
    - 当然，您也可以自由的组合两种开放能力，开发出强大的多维表格应用。

下面我们对多维表格插件进行详细的说明和案例展示。
  

## 多维表格插件详解

多维表格将会有一个「插件市场」的入口（2023 下半年），用户在日常使用的时候，需要更多、更高阶的功能，即可在市场中寻找能满足 TA 需求的产品。
插件市场主要按分类和功能点位筛选展示。用户可直接安装使用插件，就像现在在Lark开放平台上安装应用一样，不同的点是它离用户使用场景更近。
同时，我们还将支持将插件嵌入到模板中，用户在使用模板的时候，可一键启用。

![截屏2023-07-11 10.25.22.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7ab6285b4e0492b2e9ec868bc214b72a_1SzuYASruw.png?height=1560&lazyload=true&width=2630)


![1.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a40bb672c047e8d4b1f5866227628988_0v894ny1is.png?height=1634&lazyload=true&width=2636)


![2.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/93e7c24ace27422d70cd4af03a90e6d6_QgD9tUvH9A.png?height=1634&lazyload=true&width=2632)
  
多维表格开放点位有 3 类，分为模型&数据、视图、逻辑，每个分类下细分不同的点位，下面将结合示意图和案例，给大家详细讲解一下，让大家快速了解不同点位的功能。





**点位分类**              | **插件点位**          | **点位示意图**                                                   | **案例**                                                        |
| --------------------- | ------------------------- | ------------------------------------------------------- | --------------------------------------------------------- |
| **视图**                | **记录视图插件** ***已开放***           | &nbsp;![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/55abcb915a382f936419b0457db29396_2Q0ZvZVxCV.png?height=1642&lazyload=true&width=2636)    | 排版打印插件![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/24f1799b405cdec8a5d433140c155eb2_kKsjNZQZXl.png?height=1588&lazyload=true&width=2914)![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d04bc571388bc8e7d6c1456c9b5329b7_uOVP6qkEV8.png?height=1596&lazyload=true&width=2924)  |
|     **视图**         | **数据表视图插件** ***已开放*** | &nbsp;![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c6e011b472a13469ad2d4c0e4553a203_Zw69YZzUMw.png?height=1596&lazyload=true&width=2516) | 地图视图![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0dfd9b7c9f2fcb24840831ecd36b5ca9_nRZhpLBvCg.png?height=1572&lazyload=true&width=2530)![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4760789e281672422bba447fd43f3f3c_kKZpdgJIEK.png?height=803&lazyload=true&width=1280)                      |
|   **视图**          | 仪表盘视图插件 ***开发中***     | ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2dde7a7ce278e4fe24abdd9533846228_897GKShxIZ.png?height=1640&lazyload=true&width=2640) | 虚位以待：如箱线图、热力图等                                              |
| **逻辑**                | **自动化** ***已开放*** | &nbsp;![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7cc0a10968bed4e36a28fe63af06a407_igRS0r3ngM.png?height=1644&lazyload=true&width=2636) | OCR 识别文件内容![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/66fbbbba4d5e0159fec819eca56787a5_UlMyRSnBaw.gif?height=540&lazyload=true&width=1180) |
|  **逻辑**            | 公式插件 *开发中*            |       一             |  一  |
| **模型&数据**             | 数据连接器 *开发中*       | &nbsp;![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/afe87a4e023d2c83d87b395c9c54757c_M23LSWtsym.png?height=1576&lazyload=true&width=2908)  | Jira 数据连接器![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/afe87a4e023d2c83d87b395c9c54757c_7oLfqnOoPS.png?height=1576&lazyload=true&width=2908)![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0354d8d1003c7ed2e6ef96d8b6697028_gccGmy44lt.png?height=1596&lazyload=true&width=2926)![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ab07e03d7bd96364d430bb85e634797b_b8nS5SUfsg.png?height=1590&lazyload=true&width=2902) |
|**模型&数据**             | 字段插件 *开发中*            | &nbsp;![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9e85c63a0ff893a1fc97c165220dbe30_gP5Ub43yUj.png?height=1640&lazyload=true&width=2640) | 高级地理位置字段![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a87ab41fb186a5dbce9be26f8953cae1_WVgIyT82gH.png?height=792&lazyload=true&width=1275)![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8e7e2609acfa6f313b557bbafde10405_lZgCsxhOz6.png?height=794&lazyload=true&width=1280)![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/41bf13eed906d0aa280736e8629dd464_TI5Ie3CS4S.png?height=796&lazyload=true&width=1280)|


总结一下，一个多维表格插件应用，可以使用多个插件点位，构建一个拥有完整功能的应用；也可以在单点功能上做得非常强大，让用户在已有的多维表格上使用。

## 体验多维表格插件

**排版打印**：该扩展可以快速将Base中的数据格式化为具有专业外观的文档。 在一次格式化后，表中的所有数据都可以使用模板导出为可打印文档。
[安装链接](http://app.larksuite.com/app/cli_a4d1b87a54f8d00a)
<br><br>
**Lark Card Extraction**：
理解和构建公司文档以创建业务自动化流程
[安装链接](https://app.larksuite.com/app/cli_a49a84bd1e78d00a)
<br><br>
**Email via Outlook** :
「通过 Outlook 发送电子邮件」扩展允许您触发 Outlook 服务，通过 Base 的自动化功能向各个用户发送电子邮件。
[安装链接](https://app.larksuite.com/app/cli_a403e6f10bb9d00a)

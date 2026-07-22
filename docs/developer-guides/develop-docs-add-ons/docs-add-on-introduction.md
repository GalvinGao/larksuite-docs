---
document_id: '7270779605446918150'
directory_id: '7270719284443545605'
title: 云文档小组件概述
full_path: /uAjLw4CM/uYjL24iN/docs-add-on/docs-add-on-introduction
breadcrumb:
- Developer Guides
- Develop Docs Add-ons
- Docs add on introduction
document_type: GuideDocumentType
updated_at: 2023-08-24T06:58:20Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/docs-add-on/docs-add-on-introduction
---

# 云文档小组件概述
## 文档小组件是什么？有什么价值？

「云文档小组件」是Lark文档新推出的一套开放能力，它支持企业 IT 和第三方服务商开发各类垂直场景的应用，实现文档协作与企业工作流的打通，进而提升效率。
- 对于企业 IT：可以通过自建应用的方式，快速地将业务工作流集成到文档，满足垂直的业务场景，降本增效。如打通项目管理系统、打通 OA 审批系统、打通任务督办系统等。
- 对于第三方服务商：可以通过在上架小组件的方式，发布特定场景的应用，供用户直接在文档中调用。如辅助排版的格式检查工具、浏览 3D 建模模型的应用、Mermaid 代码生成 UML 图形的应用等。

:::html
<video style="padding:10px;" width="100%" controls>
  <source src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/475bfb204b142b208b3590b7a7eb0ca1_yJeLzMc5V5.mp4"  type="video/mp4">
</video>
:::

普通用户可以在文档协作过程中，快速调用以上小组件，一个文档，搞定工作。具体使用案例和使用步骤详见下面。


## 案例分享
### 案例1：安克创新打通自研的“问题管理系统”，借助文档高效处理问题，实现快速响应

| 维度 | 描述 |
| --- | --- |
| **场景** | 业务流程管理（BPM），例如问题管理等，是制造业的核心管理流程之一。企业通常会使用专业的问题管理平台进行统一管理。 |
| **现状** | **问题管理旧流程**<br>在旧的流程中，由于任务分发涉及的判断条件比较多，系统的复杂度很高。无论问题大小，都需要由各个节点的负责人审批才能进入下一步。同时，由于系统中信息格式有限，问题资料并不能完整地存储在系统中，而是存在云文档里。每一个记录在问题管理系统里的问题，根据优先级、问题范围、大小等信息，都需要通过拉齐业务方讨论来产生解决方案，这样一来就会形成一个较为繁复的流程（如下图所示）。<br><br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/dfbedc798050d4e98970aa827893cfa2_os7xlo2keE.png?height=223&lazyload=true&width=1363)<br><br>**痛点**<br>1.  流程复杂导致问题管理周期冗长，这很容易造成问题处理不及时、未结案的问题难以被跟进等情况。<br>2. 文档与问题管理平台间信息不互通，重复操作增加了员工的工作量，导致问题解决效率低下。 |
| **解决方案** | **解决方案**<br>为了促进项目的流畅推进，安克创新在云文档中创建一个问题管理系统的小组件，把问题管理系统和文档信息联通，用文档来承载问题的所有上下文，在文档里直接更新问题进展，简化输入流程，高效处理问题，实现快速响应。<br><br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0783c3e244c00d386867c1c8d4ddac87_paOAcV4n8b.png?height=859&lazyload=true&width=1280)<br><br>**问题管理新流程**<br><br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/cfa84cf27866ce0826004d50a5f4c0ab_nz21LsLkQ8.png?height=203&lazyload=true&width=853)<br><br>**集成方式**<br>**步骤一 ：在云文档插入一个“问题管理小组件”**<br>1.  在小组件中输入 KEY 值，点击“同步到文档”即可绑定“问题管理系统”中 KEY 所对应的记录，并将它们展示在小组件中。<br>2. 小组件支持了丰富多样的标准组件，你可以通过调用这些组件，在文档中绘制一个表格来记录数据。<br><br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6977fb109377556f5e8574dd33f24d9c_phY3qiZrjg.png?height=835&lazyload=true&width=1280)<br><br>**步骤二 ：将“问题管理小组件”数据同步回“问题管理系统”**<br>1.  在小组件中记录问题信息后，点击“同步到系统”即可将数据更新到“问题管理系统”中<br>2. 你可以通过 Open API 获取填写人的用户信息，并自动添加到记录中。<br>3. 你还可以快速接入小组件的数据协同能力，来支持多个用户在同一个小组件中协同编辑。<br><br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/6a304051a93330a1cbcb99f725ab04d7_MQf0davnIJ.png?height=764&lazyload=true&width=1280) |
| **方案优势** | **轻量审批和同步，简化任务跟踪**<br>像其他 BPM 系统一样，问题管理系统很注重对真实世界的仿真。实际操作中，由于任务的判断和分发有很多的决定因素和协同部门，任务推进的每一步都需要各部门依次审批，流程繁琐、问题难以跟踪。<br><br>**通过集成问题管理系统的云文档 block** <br>可以实现在文档内管理任务。通过 block 直接更新信息，一键同步回问题管理平台，无需层层审批录入。原本要花几天才能走完的流程，现在几分钟就能完成，实现轻量管理问题，高效流转任务。<br><br>**以云文档为中心，高效判断和分发任务**<br>有了问题管理系统的云文档 block 后，来自不同部门的项目成员可以把问题相关的信息写在一个文档里，利用云文档的协同能力，@相关同事或划词评论，高效沟通、判断、并分发任务。讨论的过程和结论直接记录在云文档里，通过 block 同步回问管理平台。无需切换不同系统，让问题管理的重点回归到处理问题本身，而不是完成流程。<br><br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1cfb4c59990ef1bb235413d7c33016f9_R115uVFr0O.png?height=780&lazyload=true&width=1280)<br>**问题资料沉淀在文档里，信息完整易获取**<br>用问题管理系统来管理问题处理的流程，理清节点和每个节点在流程中的角色，让大家有全景图后更好地工作。用云文档来完成问题任务的判断和分发，不仅可以高效处理问题，还能把问题的上下文都记录在文档里，利用Lark的知识库、全局搜索、以及企业百科能力，随时沉淀、统一管理、快速查找。<br><br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4e995812dac5c963f6b169edab18f85d_GIrumdXBhT.png?height=780&lazyload=true&width=1280) |


### 案例 2：开发者基于文档小组件的开放能力，开发了几个实用的创作效率工具

| 应用名称 | 描述 |
| --- | --- |
| **文本绘图** | 使用 Mermaid 语言，用几行文本代码快速创建流程图、时序图等复杂图形<br>- 实时查看预览编辑效果，所见即所得<br>- 内置多种图形模板，助你快速上手<br><br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/c956b25ed9ad819c5ccdb4c2dd717d83_G7iLa6eSLa.png?height=1000&lazyload=true&width=1600)<br><br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/4ae78e7a6921ae561eca59fca983bf2b_HUc84FMI7O.png?height=1000&lazyload=true&width=1600) |
| **批量设置格式** | 支持快速设置 BIUS、字体颜色、对齐方式等多达 13 种文字格式，“刷一刷”即可快速应用设置好的格式，助你写作事半功倍<br><br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/442a1536bb2c74fd17eb1a8ecf11603a_2hMn9uJEX9.png?height=1000&lazyload=true&width=1600) |
| **文字排版助手** | 你的智能格式纠正助手，检测格式问题（中英空格、中数空格、全角半角符号等），一键优化，提升工作效率<br><br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5399a381edee981f4d21cad627e54ab0_VZ75nsHIkS.png?height=1000&lazyload=true&width=1600) |
| **时间轴** | 图形化地按时间、步骤、顺序表达项目和事件的发展进程、里程碑、关键点等内容。帮助文档编辑者高效生动表达，阅读者高效愉悦阅读<br><br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/f1d52bf57a11aa5b26d15562c53ae0b3_askGdoZ6k8.png?height=1000&lazyload=true&width=1600)<br><br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/be4c0d27182d71689e9245d03bccc655_9nL2vAgivc.png?height=1000&lazyload=true&width=1600) |


:::html
<style>
  .markdown-render-wrapper p > img {
  	margin: 0;
    display: block;
  }
  .markdown-render-wrapper table tbody tr td {
	vertical-align: top;
  }
</style>
:::
## 普通用户如何使用文档小组件？
### 如何发现？
|      **加号菜单或“/”快捷命令**     | **顶部菜单栏**      
| --------- | --------------- | ---------------
|![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e73deeff518d0f220f89169cb33568ad_qmT4oQVzUX.png?height=1078&lazyload=true&width=2320)<br><br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1dfea86ccf5260019659109d2a4fb5dd_8or9LAyYRO.png?height=800&lazyload=true&width=1280)| ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0cbb69da8bb1e1cc0b9c8e024871b920_cRTtlBvnhX.png?height=1280&lazyload=true&width=1085) 

### 有哪几种组件类型？
|      **内嵌自定义内容**     | **悬浮**      | **全屏**     
| --------- | --------------- | --------------- 
|![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/15128575d61133a56d9a783850727f09_1OQ3gAMSfS.png?height=800&lazyload=true&width=1280)<br><br>![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/15128575d61133a56d9a783850727f09_pDCDSl6xaU.png?height=800&lazyload=true&width=1280) | ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9b33e58cbd7e2da85588d1d5dee3e4c0_04mQ6pX5fE.png?height=789&lazyload=true&width=1280) | ![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/df136fe1a9c067145d875a0d4519dbe2_3HaKKvLxZm.png?height=800&lazyload=true&width=1280) 
## 文档开放解决方案简介

### 提供文档小组件的应用市场，帮助用户发现和管理自己的组件
支持用户自主探索发现文档小应用，安装后出现在个人列表中。
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3c3831a0ab5bbd847c91e4c962c704ba_6SXybuM5lJ.png?height=798&lazyload=true&width=1280)
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b8a958992cc851f10d364ddefa40f421_5ifEH39kRy.png?height=1068&lazyload=true&width=1280)
### 文档的哪些能力会被开放出去？

| 能力 | 详情 |
| --- | --- |
| **可获取文档信息** | 自身信息：文档标题、内容（块的类型及数据）、统计数据、权限信息、历史记录<br>环境信息：黑暗模式、文档模式、多语言 |
| **可调用文档功能** | 内容操作：工具栏、选人/会话组件、名片预览、图片预览、复制粘贴、撤销还原、展开/折叠块等<br>容器操作：窗口滚动、容器比例调整、提示消息配置、模态弹窗 |
| **可编辑文档内容** | 具体内容：标题、正文（文本、图片、表格、任务、公式、@、文件、云文档、iframe 等）<br><br>格式：标题格式、文本加粗/斜体/下划线/颜色、引用、分隔线、有序/无序列表、代码块<br><md-alert type="error">注：不支持三方内容的编辑，如在云文档中的 iframe 的操作。</md-alert> |
| **可感知文档变化** | 用户操作：文档的打开与关闭、文档内容变化、Block 的悬浮选中<br>用户权限：可阅读、可编辑、可评论 |
| **可与用户互动** | 识别用户身份、获取用户授权、阅读进度感知、权限感知、协同编辑 |
| **可完全掌控小组件本身，自定义渲染** | 改变自身形态（内嵌块、全屏、悬浮、正文扩展）<br>- 读取与存储数据<br>- 可引入外部能力 |



## FAQ：开发一个小组件需要哪些技术能力

1. 小组件使用的是 Web 网页开发技术，可以把小组件的外部运行环境想象成一个浏览器，只要能在浏览器运行的程序，就可以看做是一个小组件。
1. 对于一个前端工程师而言，我们不局限你使用什么框架，你可以选择自己熟悉的技术栈或者是组件库。我们也为你提供了开箱即用的模版，提供了服务构建、调试、发布工具集，这些是以 React 生态为主的。当然你也可以选择自己熟悉的工具集，将服务部署到我们的 CDN 服务器上即可。
1. 最后期望你具备：HTML、CSS 和 JavaScript 的网页基础知识，能阅读基础的技术API文档，小组件开发成本很低，快来尝鲜吧。

---
document_id: '6967331158355214342'
directory_id: '7122028361539010566'
title: 三方审批接入指南
full_path: /ukTMukTMukTM/uAzNyYjLwcjM24CM3IjN
breadcrumb:
- Server API
- Approval
- Access guide
- Third-party approval integration
- Three-party approval access guide
document_type: GuideDocumentType
updated_at: 2024-01-30T13:34:30Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uAzNyYjLwcjM24CM3IjN
---

# 三方审批接入指南


## 三方审批接入概述

三方审批接入，即Lark审批支持将外部系统（**企业自研**或采购的**第三方系统**）的审批任务推送到Lark审批应用。审批任务可以实时多端（PC、手机、Pad）触达用户，并通过**Lark审批的嵌入式单据**打开，用户在Lark审批中即可统一查看和处理所有审批、享受**一站式**的审批体验，同时结合对话框消息推送、一键转发聊天、每日待办提醒、效率看板等功能，从审批发起、处理到统计全面**提升审批效率**。

（如何使用Lark审批自建流程，请点击：[Lark审批用户手册](https://www.larksuite.com/hc/zh-CN/articles/360034502513)）

 ## 1.适用场景

中大型企业的审批往往分散在不同的业务系统中，审批人每天需要花费时间切换不同系统去处理审批，操作繁琐且浪费时间，而且审批人很容易忘记，导致业务不能及时得到处理，影响企业的管理效率，高层管理者面临的审批类型和数量更多，问题往往也更加严重。


![0818修改.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b7d8a821ed44fd99a44fcf625a9f3106_vyFuBVc7qe.png?height=1520&lazyload=true&width=3200)

Lark审批提供了轻量的对接方式，企业无需改造原有的业务系统，**只需在数据层将审批任务汇总推送到Lark审批** **，** **审批人即可“一站式”查看和处理所有审批任务**；并且通过每日待办提醒、快捷审批、列表自动聚类等审批提效功能赋能，提升审批人的处理效率。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/54afeaf394919c936a2a3ecb3b5fa569_LFSOvtdMKR.png?height=1520&lazyload=true&width=3200)


## 2. 三方审批接入Lark审批中心的价值

  1.  **审批中心将分散在各处的审批单据聚合，减少多入口的不便利，打造“一站式”的审批管理中心**

        1.  通过集成将原本分散在**企业** **OA** **、人力资源、财务**等各系统中的单据集合到审批中心列表中，并提供统一、便捷的操作、查询方式。
        1.  审批中心支持将三方系统的审批页面嵌入或侧边抽屉的形式打开，减少审批人页面跳转情况，带来连贯顺畅的审批体验。
        1.  页面结构及设计简洁，同时适配 PC、手机、Pad ，多端无缝切换，推广、上手门槛低。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b10957163d4f3269082b5fe70da69e1e_5kprDPsLlC.png?height=435&lazyload=true&width=1280)

2.  **接入审批中心为业务提效赋能，助力更高效的审批流转效率**

    1.  集成后的待办审批任务会通过 bot 消息聚合后推送每日待办，降低审批流转时间，减少审批任务长时间滞留情况。
    1.  集成后可快捷将审批转发到聊天中，打通审批与 IM 信息流，审批人在沟通中即可完成审批
    1.  审批流转情况将汇集到审批效率看板中，管理效率的提升直观可视化。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/59eea8d0b807df256175282c366d99e4_Tdyg11Pw0t.png?height=1716&lazyload=true&width=2500)

3.  **提供统一、简洁的审批操作，给审批人更轻松的审批体验**

    1.  审批中心建立在Lark生态环境下，各系统间的连接性更好，可拓展使用场景更广。
    1.  审批中心通过自动聚类、标记稍后处理等形式，以审批人的视角提升处理效率，带来更轻松的审批体验。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e2e81f5eb47db82a73afacc7c7f16572_At3XXIwK1P.png?height=1350&lazyload=true&width=2400)


## 3. 最佳实践

字节跳动的审批之前分散在 23 个系统中，现在审批人可以通过“审批中心”进行“一站式审批”：

-   从集成到审批中心前后对比来看，完成一单审批中全部节点审批的时间缩短 **48%** 。
-   将审批待办聚合并提醒，能一定程度上避免因审批环境分散而导致有单据遗漏不能得到及时处理的情况，对 24 小时内的消息阅读率有显著提升 **6.62%** 。
-   在字节跳动每天有约 **25000+** 的审批任务被完成的背景下，效率提升显著。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/3efd022366e3554df11f9da966244ea9_bTnLWrOLzj.png?height=1602&lazyload=true&width=2560)


  ## 4. 接入方式与成本

| **可选方案**          | **方案简介**                                                             | **对接方式**                                                                                                                                                                                            | **客户成本**                                                           |
| ----------------- | -------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| **使用Lark审批提供的集成工具** | Lark官方团队自研用于对接三方厂商 OA 与审批中心的连接器 Connector，以满足客户低成本集成三方 OA、在Lark使用审批功能的需求 | 目前仅支持部分头部的OA系统，                                              | 1.客户IT低成本对接<br>2.根据部署手册进行配置，最快当天完成测试上线、看到效果                       |
| **通过审批的开放能力自助集成** | 客户或客户合作伙伴利用Lark审批团队提供的审批开放能力，完成厂商系统对接以及Lark审批中心对接，实现在Lark使用审批功能            | 客户通过自己的研发团队或选择技术合作伙伴通过Lark官网提供的一系列 **[开放接口](/document/ukTMukTMukTM/uAzNyYjLwcjM24CM3IjN)**  进行自助对接 |   1. 由客户或客户选定的三方技术合作伙伴对厂商系统和Lark审批中心进行对接，通常需要一定的费用<br>  2.对接周期一般在 2~4 周 |


## 5.开发教程
:::html
<md-dt-table>
  <md-dt-thead>
      <md-dt-tr>
      <md-dt-th style="width: 50% ">教程名称</md-dt-th>
      <md-dt-th style="width: 50%">教程步骤拆解</md-dt-th>
      </md-dt-tr>
  </md-dt-thead>
  <md-dt-tbody>
    
<md-dt-tr level="0">
	<md-dt-td>
	[快速开发三方审批](/document/home/quickly-develop-three-party-approvals/introduction)
	![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/7e3dbfdaaeac897ce0949076621a69c0_QqSOUFSBky.jpeg?height=600&lazyload=true&width=1128)
	</md-dt-td>
	<md-dt-td>
	教程步骤拆解
1. [简介](/document/home/quickly-develop-three-party-approvals/introduction)
2. [准备工作](/document/home/quickly-develop-three-party-approvals/prep-work)
3. [创建应用和申请权限](/document/home/quickly-develop-three-party-approvals/creating-applications-and-requesting-permissions)
4. [获取访问凭证](/document/home/quickly-develop-three-party-approvals/get-access-token)
5. [创建和更新三方审批定义](/document/home/quickly-develop-three-party-approvals/create-and-update-three-party-approval-definitions)
6. [三方审批实例同步](/document/home/quickly-develop-three-party-approvals/three-party-approval-instance-synchronization)
7. [发送与更新审批bot消息](/document/home/quickly-develop-three-party-approvals/send-and-update-approval-bot-messages)
8. [三方快捷审批](/document/home/quickly-develop-three-party-approvals/three-party-expedited-approval)
9. [三方审批实例校验](/document/home/quickly-develop-three-party-approvals/three-party-approval-example-verification)
	</md-dt-td>
 
</md-dt-tr>
    
    
      </md-dt-tbody>
</md-dt-table>
:::

# FAQ

1.  **Lark审批和三方系统是怎么对接的，是不是要把流程搬过来？**

-   采用的是轻量对接方式，只在数据层面打通，在Lark审批中打开的页面还是三方系统的单据详情，这样子使得对接的成本极低，又能实现“**一站式**”查看的体验。

2.  **什么样的审批可以被集成到Lark审批** **？**

-   无论是企业自建审批系统还是采购的第三方审批系统，只要企业可以获取到审批任务数据，就可以对接Lark审批 **。** 企业自建系统理论都可以对接，三方系统中，目前我们官方Connector已经可实现对**泛微**OA、**致远**OA的快速集成，其他如 **蓝凌** **、北森**等系统，可通过开放能力实现集成。

3.  **相比企业自己开发 OA ，Lark审批集成方案有什么优势？**

-   与Lark套件的深度打通：企业自己的 OA 与Lark套件是割裂的，仅仅是在Lark内增加了一个应用，而Lark审批作为官方应用可以与Lark套件做更深度的集成，更强的联通意味着更优的体验和更高的效率，可以支持直接在 message、doc、email 进行审批，让审批的动作更贴近行为发生的地方。
-   审批效率和信噪比持续优化：大多数企业自己的 OA 往往是“一锤子买卖”，开发上线之后很少再投入大量资源进行持续优化，而Lark审批团队会持续地在效率和信噪比提升上进行投入，目标是打造“处理效率最高的审批系统之一”。
-   



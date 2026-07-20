---
document_id: '6967331173081563141'
directory_id: '7186908899420831749'
title: 审批接入指南
full_path: /ukTMukTMukTM/ukDNyUjL5QjM14SO0ITN
breadcrumb:
- Server API
- Approval
- Access guide
- Approval access guide
document_type: GuideDocumentType
updated_at: 2023-01-31T12:16:19Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/ukDNyUjL5QjM14SO0ITN
---

# 审批接入指南

## 什么是审批接口

审批是Lark套件为企业提供的官方应用，可以快速建立企业内部审批流程，如请假、出差等。  
审批开放接口可以对审批实例进行查询和创建，可用于企业原有平台与审批打通。

## 名词解释

### 1、审批定义（Approval）

单个审批流，由表单和审批流程（详情见下）组成，创建后可以让员工在发起审批时填写各个控件的值并形成审批实例。

### 1）Approval Code

审批定义对应的唯一编码，可在编辑审批时从浏览器 url 中找到。点击以下链接可以进入管理后台：

[https://www.larksuite.com/approval/admin/approvalList?devMode=on](https://www.larksuite.com/approval/admin/approvalList?devMode=on)


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d02eb479bb69d97f88a7e118bb79e0d5_vJ7QNmLumi.png?lazyload=true&width=1640&height=207)

点击上图中的编辑按钮，可以在URL中找到Approval Code，格式为：`definitionCode=E3254848-D172-4169-B03E-744E7CD11F06`


### 2）表单（Form）

表单包括一或多个控件，员工发起审批时需要填写的表单页面，每个审批定义都有自己的表单。

#### 审批控件（Widget）

组成审批的部分。一个表单由一或多个审批控件组成。每个控件都有一个基础属性：

```js 
struct{
    String Id; //标识在一个审批定义中某一个控件的唯一 ID
    String CustomId; //自定义控件 ID，标识在一个审批定义中某一个控件的唯一 ID
    String Name; //该控件的名称
    String Type; //该控件的类型
} 
```
#### 自定义ID

可以通过在审批管理后台编辑定义时指定一个 ID，之后可以用指定的 ID 唯一确定一个在该定义中的控件。

#### 控件说明

1. 单行文本（input）
- 用于填写一个单行文本。

2. 多行文本（textarea）
- 用于填写一个多行文本。

3. 数字（number）
- 用于填写一个数字。

4. 金额（amount）
- 用于填写审批金额数量及单位，默认单位为 CNY。

5. 日期（date）
- 用于填写详细时间。

6. 日期区间（dateInterval）
- 用于填写一个日期区间，包括有开始时间、结束时间以及持续时间。

7. 单选（radio, radioV2）
- 用于选择单个选择。

8. 多选框（checkbox, checkboxV2）
- 用于选择多个选择。

9. 地址（address）
- 用于填写一个地址。

10. 联系人（contact）
- 用于在审批中添加联系人。

11. 说明（text）
- 用于在审批定义中添加说明（如填写规范、注意事项），在发起审批时不可编辑。

12. 明细（fieldList）
- 用于填写明细信息，在明细中可以添加其他控件比如数字、金额等。在创建审批定义时设计一个明细控件表示明细的一个条目中包括哪些控件，发起人可以根据自身需求增加条目，每一个条目都和创建审批定义时所设明细控件一致。

13. 计算公式（formula）
- 在创建审批定义时设计计算方式，表示该控件的值依赖于其他控件（数字、金额）计算得出。

14. 图片（image, imageV2）
- 用于在审批中添加图片。

15. 附件（attachment, attachmentV2）
- 用于在审批中添加附件，如文件等。

16. 关联（connect）
- 用于在当前审批中关联其他审批，使审批人能够在审批时查看所关联审批的概况。

17. 请假（leaveGroup）
- 用于填写请假审批的相关内容，包括选择请假类型（如病假、产假等，请假类型需要管理员提前在假期管理中设置并在创建审批定义时选择当前审批定义发起实例时可选种类有哪些），填写请假开始时间，填写请假结束时间以及请假时长。

18. 加班（workGroup）
- 用于填写加班申请的相关内容，包括选择加班类型（比如调休、带薪、不带薪等，由管理员在创建审批定义时设定），填写加班开始/结束时间和时长以及填写加班原因。

19. 换班（shiftGroup）
- 用于填写换班审批的相关内容，包括填写换班时间以及填写换班原因。

20. 出差（tripGroup）
- 用于填写出差审批的相关内容，包括填写行程（一段行程中包括行程开始时间、行程结束时间、行程时长、出发地、目的地、交通方式、单程/往返以及备注，其中行程时长以自然日为单位计算，并且倘若出差有多段行程，可以由发起人手动增加新的一段行程），出差总时长，出差原因以及同行人。

21. 补卡（remedyGroup）
- 用于填写补卡审批的相关内容，包括填写补卡时间和填写补卡原因。

### 3）审批流程（Process）

管理员在创建审批定义同时会设计一个审批流转方式，这个方式包括了一个由该定义产生的实例每一步如何进行。流程由审批节点构成，一个审批定义对应一个审批流程。一个审批流程对应一或多个审批节点。员工在发起审批实例后，按照设计好的审批流程进行流转。设计审批流程时，需要指定在流程的每一步（审批节点，详情见下）的审批操作。包括有每一步的审批人或者到当前步骤时系统自动通过或系统自动拒绝。如果当前步骤不是由系统自动进行操作，则需要设置审批人或者设置由发起人自选审批人。

#### 发起人自选审批人

即创建审批定义时在某一步不指定审批人，设置了由发起人自选，那么是由操作人在发起审批实例时自己选择审批人，审批定义创建时只需指定当前步骤是由所有操作人所选审批人或签或者会签即可。**如果在定义设置了发起人自选审批人，在通过 API 发起审批实例时必须要指定该步骤发起人所选审批人**。具体如何通过 API 发起一个自选审批人的实例可以查看 API 方法说明。

#### 审批节点（Node）

每一个审批流程由一或多个审批节点构成，每一个节点表示审批流转过程中的一个步骤，在当前节点完成后进入下一个节点。每当实例创建时生成该实例对应的各个审批节点，并由审批节点构成审批流程。

#### 自定义 ID

和控件类似的，节点 ID 也可以指定，指定后可以用指定的 ID 唯一确定一个在该流程中的节点。

#### 审批节点种类

1.会签（AND）
- 表示当前节点需要所有选择的审批人都同意后才被认作同意，进入下一节点

2.或签（OR）
- 表示当前节点只需要有一个审批人同意后就可被认作同意，进入下一节点

3.自动通过（AUTO_PASS）
- 表示当前节点系统会自动同意该审批，进入下一节点

4.自动拒绝（AUTO_REJECT）
- 表示当前节点系统会自动拒绝该审批

### 2、审批实例（Instance）

员工发起审批时产生的对象。包括多个审批任务（详情见下）。

#### 审批实例状态
1.进行中（PENDING）
- 表示当前审批实例还在流转中，没有最终结果

2.已同意（APPROVED）
- 表示当前审批实例已经被通过

3.已拒绝（REJECTED）
- 表示当前审批实例已经被拒绝

4.已撤回（CANCELED）
- 表示当前审批实例已经被发起人撤回

5.已删除（DELETED）
- 表示当前审批实例正在流程中，但是由于管理员停用或删除了当前审批定义，导致该审批实例变为已删除状态

### 审批任务（Task）

审批任务依赖于审批节点存在，每一个审批节点可能包含有一或多个审批任务，每一个任务表明当前审批节点的审批人是谁。如果当前节点包含有不止一个审批人，则有多个审批任务，每一个审批任务对应不同的审批人。当某一节点有多个审批任务时，某些任务可能会因为其他任务状态的改变而改变状态，并不单独依赖于审批人的操作（例如一个或签节点对应多个审批任务，当其中一个任务通过之后，其他任务会自动变为已完成状态而不需要对应审批人进行操作）。每当审批实例流转到新的节点时创建该节点的任务。

#### 审批任务状态

1.进行中（PENDING）
- 表示当前审批任务正在进行中，没有最终结果

2.已同意（APPROVED）
- 表示当前审批任务审批人已经同意

3.已拒绝（REJECTED）
- 表示当前审批任务审批人已经拒绝

4.已转交（TRANSFERRED）
- 表示当前审批任务审批人已经转交，交给其他人审批

5.已完成（DONE）
- 表示当前审批任务已经完成

::: note
可能出现审批任务状态为DONE的情形包括但不限于：
- 或签节点中其中一人同意（该人审批任务状态为 APPROVED），其余人审批任务状态为 DONE
- 或签/会签节点中其中一人拒绝（该人审批任务状态为 REJECTED），其余人审批任务状态为 DONE
- 在某一任务进行中时，发起审批人将该审批撤回，则审批人任务状态为 DONE
- 在某一任务进行中时，审批流程管理员将该审批定义删除，则审批人任务状态为 DONE
- 一般来说，可以认为在审批人没有进行操作而他的审批任务状态被改变的情况下，当前审批任务状态一般都为 DONE。
:::

### 3、审批动态（Timeline）

记录了某一审批实例从创建开始到当前时间所发生的所有操作及状态改变，用于追踪实例详情的改变。

## 如何使用审批接口

### 1、创建审批

启用审批功能后，进入审批后台创建审批，根据需要填写审批信息、表单和流程。


![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/0f82470fd78078fc2bb10ee0cdfe37ff_uZyYBGKrp9.png?lazyload=true&width=1640&height=769)

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/66b8a297dc56bfe1b74c94f0b18df6a0_942kVPhgcL.png?lazyload=true&width=1640&height=731)

### 2、调用接口

审批创建后，员工可以在应用中心-审批小程序中发起审批实例。开发者通过调用接口，发起或获取审批实例数据。

审批应用支持的接口如下：

* [查看审批定义](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/get)
* [批量获取审批实例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/list)
* [获取单个审批实例详情](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/get)
* [创建审批实例](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/create)
* [审批任务同意](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/approve)
*  [审批任务拒绝](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/reject)
*  [审批任务转交](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/transfer)
*  [审批实例撤销](/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/cancel)

审批接口权限认证依赖 tenant_access_token，获取方式如下：

* [获取 tenant_access_token（企业自建应用）](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token_internal)
* [获取 tenant_access_token（应用商店应用）](/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token)

### 3、事件监听

* [审批事件监听开发指南](/document/ukTMukTMukTM/ugDNyUjL4QjM14CO0ITN)

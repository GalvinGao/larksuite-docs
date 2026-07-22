---
document_id: '6963854764275220485'
directory_id: '6907567269107777538'
title: 申请 API 权限
full_path: /ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN
breadcrumb:
- Server API
- API Call Guide
- Calling Process
- Apply for API permission
document_type: GuideDocumentType
updated_at: 2023-11-10T09:36:09Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN
---

# 申请 API 权限

通过本文你可以了解什么是应用的 API 权限，以及如何为应用申请 API 权限。

## 什么是 API 权限

在开发应用过程中，你可能遇到调用开放平台服务端 API、监听已订阅的事件等操作，该类操作中可能需要访问租户、用户的隐私信息，也可能需要操作租户、用户的应用数据。出于安全考虑，你需要为应用申请相应的权限。当应用审核通过后权限方可生效，后续应用才可以调用[服务端 API](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/server-api-list)或监听已订阅的[事件列表](/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-list)。简单来说，应用的 API 权限（Scope）决定了应用能使用哪些Lark服务端的开放能力。

### 权限等级划分

在商店应用、自建应用两类不同的应用中，权限存在不同的等级划分。你可以通过下表了解不同应用类型中的权限等级划分情况。

:::note
开放平台具备的所有权限详情，可参阅[API 权限列表](/document/ukTMukTMukTM/uYTM5UjL2ETO14iNxkTN/scope-list)。
:::

#### 自建应用

| 权限等级 | 权限说明 | 权限审核规则 |
| --- | --- | --- |
| 免审权限 | 租户管理员可以根据本租户实际数据管控诉求，配置免审权限来减轻审核负担。具体配置方法，可参见[管理员如何设置自建应用审核规则](https://www.larksuite.com/hc/zh-CN/articles/360048488346)。 | 无需审核，申请后立即生效。 |
| 需审核权限 | 对于涉及敏感数据的权限，请纳入需审核权限列表。 | 申请后，需要创建版本并提交审核，由应用管理员审核通过后才可生效。 |


#### 商店应用

| 权限等级 | 权限说明 | 权限审核规则 |
| --- | --- | --- |
| 普通权限 | 访问的数据敏感程度不高。 | 对于商店应用，所有的权限操作都需要经过以下审核流程：<br>- 应用上架流程审核：由Lark开放平台审核。<br>- 租户安装应用流程审核：由租户管理员在版本更新时审核。 |
| 高级权限 | 访问的数据敏感程度较高。如无必要原因，申请高级权限时不会通过审核。 | 对于商店应用，所有的权限操作都需要经过以下审核流程：<br>- 应用上架流程审核：由Lark开放平台审核。<br>- 租户安装应用流程审核：由租户管理员在版本更新时审核。 |



### 权限申请原则

权限的申请应该遵循最小可用原则，权限范围过大可能会威胁企业数据安全管控。在无充分理由的情况下，请注意避免直接申请大量接口权限。因此，在Lark开放体系中，权限的管控有严格的审核流程。

- **企业自建应用**：应用开发者申请的权限需要通过 **租户管理员** 的审核。租户管理员可以按需配置审核规则和审核方式，参考[自建应用发版审核指南](https://www.larksuite.com/hc/zh-CN/articles/360048488346)。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/9d85c5a218aade1a90742badce398770_9o1byXFiAa.png?height=1758&lazyload=true&maxWidth=600&width=3070)

- **ISV 开发的商店应用**：如需申请权限，需要通过Lark开放平台[发布应用](/document/uMzNwEjLzcDMx4yM3ATM/uYjMyUjL2IjM14iNyITN)时和[租户安装应用](https://www.larksuite.com/hc/zh-CN/articles/777448915154)时的两道审核流程。
    
    商店应用在初次安装和版本更新时会接收到授权提醒。租户管理员可以按需设置管理规则，参考[审核应用的获取与使用申请](https://www.larksuite.com/hc/zh-CN/articles/360023575474)。


### 确定所需申请的权限

当你在实际开发应用时，可根据 API 或事件的开发文档获取相应的权限信息。
以[获取单个用户信息](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/get)接口为例，在相应的接口文档中，你可以获取接口的权限要求、字段权限要求等信息。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/668e96dd40c2144670a3d3287e299003_YJlVnMyMz1.png?height=1088&lazyload=true&maxWidth=600&width=1666)

其中：

- **权限要求**：代表整个接口的权限要求，该类权限之间是或关系（即应用申请任一权限即可调用该接口）。

- **字段权限要求**：展示了获取响应体字段数据所需的权限要求。
    
    例如下图，自建应用想要获取响应体中的`user_id`字段值，则必须开通 **获取用户 user ID** 权限。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/1593fb323084c155e2a496eacb581d87_hMvpuj70yT.png?height=340&lazyload=true&maxWidth=600&width=1646)


## 为企业自建应用申请 API 权限

### 步骤一：申请权限

1. 登录[开发者后台](https://open.larksuite.com/app)，进入指定自建应用。

2. 在左侧导航栏进入 **开发配置** > **权限管理** 功能页。

3. 在 **API 权限** 页签选择调用 API 所需的权限。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/60d928bc3806193cfce706d0f2d5bfcb_1gxnscSiev.png?height=820&lazyload=true&maxWidth=600&width=2344)

4. 选择完成后，在 **权限配置** 区域右上角点击 **批量开通**。

### 步骤二：免审核 API 权限进行测试联调

在应用的测试联调阶段，你可以通过以下路径，无需审核通过即可生效 **需审核权限**，进行 API 调试。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d10526b691f33d2407060de68496ee03_BFHpJYsEFc.png?height=540&lazyload=true&maxWidth=600&width=3200)


#### 方式一：使用 user_access_token 调试 API

批量申请 API 权限时，如果权限支持通过应用开发者的 user_access_token 免审调用 API（如下图所示） ，则点击 **确认** 后，无需发布应用即可使用 user_access_token 调试 API。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/566477b94ebdda3f120ccca65c9e6c6e_FvRS6SQTNs.png?height=1434&lazyload=true&maxWidth=600&width=1166)

以调用[列出自定义角色](/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role/list)接口为例，在调用前需要在应用的 **API 权限** 功能页中，申请 **查看、评论、编辑和管理多维表格** 或者 **查看、评论和导出多维表格** 权限。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/abd14986efc8f7ca0ae4ad5a00a37401_Mv1a0NB12H.png?height=310&lazyload=true&maxWidth=600&width=1156)

申请后，无需发布应用并等待审核通过，直接使用应用开发者的用户访问凭证（user_access_token）即可调试 API。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/374dd3d190c28bd2d1ec4aff2b2cbc11_GMDCSYl9DE.png?height=446&lazyload=true&maxWidth=600&width=1780)


#### 方式二：使用测试企业调试 API

在 API 权限中，存在不支持 user_access_token 鉴权的 API，且支持 user_access_token 鉴权的 API 中也存在少量的敏感权限基于 user_access_token 无法免审调试 API。该类权限在申请时可以通过提示查看（如下图所示）。

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ed9b0b9f0a002a46de3cae122ab8aed5_pKBtfTTqUs.png?height=1430&lazyload=true&maxWidth=600&width=1158)

此时，你可以为应用配置测试企业和人员，并切换为测试版本（具体操作参见[测试企业与人员](/document/home/introduction-to-custom-app-development/testing-enterprise-and-personnel-functions)）。切换后，前往 **开发配置** > **权限管理** 页面，在 **API 权限** 页签申请指定权限，申请的权限均为 **免审权限**，点击 **确认** 后立即生效。

:::note
如果你申请了 **通讯录** 或者 **Lark人事（企业版）** API 权限，且需要通过应用身份（tenant_access_token）调用相关 API，则需要为应用配置相应的数据权限。详情参见[配置应用数据权限](/document/home/introduction-to-scope-and-authorization/configure-app-data-permissions)。
:::

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/bdac0ed8fa769de10252ac00baf6f672_rahcVH24TG.png?height=882&lazyload=true&maxWidth=600&width=2324)


### 步骤三：正式发布应用

1. 确保测试联调阶段申请的权限符合业务预期后，前往 **应用发布** > **版本管理与发布** 页面，点击 **创建版本**。
    
:::note
使用测试企业调试 API 的场景中，开通的权限不会生效于应用的正式版本。因此在结束测试联调后，需要返回正式版本，重新开通相同的权限，再创建版本。
:::
    
2. 在 **版本详情** 页面，配置以下字段，并点击 **保存**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e6a06b065dafb2e1d3353bff136f7bdb_K2L2JqCmrd.png?height=1106&lazyload=true&maxWidth=600&width=1588)

    - **应用版本号**：自定义应用版本号，格式示例：1.0.0。
    
    - **更新说明**：自定义当前版本的更新详情。
    
    - **应用能力**、**权限变更**：查看并确认添加的能力、权限是否符合预期。
    
    - **可用范围**：应用的可用范围。可点击 编辑，调整可用范围。关于可用范围的说明可参见[配置应用可用范围](/document/home/introduction-to-scope-and-authorization/availability)。
    
    - **申请理由**：用于帮助审核人员了解更多应用版本信息。

3. 在弹出的对话框中，点击 **申请线上发布**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/00b0bf496c7efacb3489bcd6ed9fbde0_J1kqDXBWlJ.png?height=826&lazyload=true&maxWidth=600&width=2324)

4. 等待企业管理员审核应用。
    
    当应用通过审核后，所有的 API 权限才会正式生效。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e74676dbe3ca25d8636205473f021bcd_sRRr2gmfOK.png?height=660&lazyload=true&maxWidth=600&width=2348)


## 为商店应用申请 API 权限

### 步骤一：申请权限

1. 在[开发者后台](https://open.larksuite.com/app)进入指定商店应用。

2. 在左侧导航栏进入 **开发配置** > **权限管理** 功能页。

3. 在 **权限配置** 区域申请权限。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/28fa06a5baf41de9d2e6f6f25390130a_IoKUBPrFBw.png?height=1216&lazyload=true&maxWidth=600&width=2882)

4. 选择完成后，在 **权限配置** 区域右上角点击 **批量开通**。

5. 在 **提示** 对话框，确认申请的权限列表无误后，点击 **确认**。

### 步骤二：免审核 API 权限进行测试联调

在应用的测试联调阶段，你可以通过商店应用的测试企业和人员功能，生成与测试版本。基于测试版本开通的权限无需等待审核通过即可生效，从而进行 API 调试。

1. 在左侧导航栏，选择 **开发配置** > **测试企业和人员**，然后创建一个测试企业。
    
:::note
关于测试商店应用的详细配置，参见[六、测试商店应用](/document/uMzNwEjLzcDMx4yM3ATM/uUjMyUjL1IjM14SNyITN)。
:::

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/5cd34af7e4f3ba070c0163cbe9ce080e_bKP6D2nTt3.png?height=408&lazyload=true&maxWidth=600&width=2208)

2. 在左侧导航栏，选择 **应用发布** > **版本管理与发布**，然后点击 **创建版本**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/437798671d54f9ac9ef3ce264dd30565_FoBjwm60ut.png?height=678&lazyload=true&maxWidth=600&width=2342)

3. 依次配置版本号、版本说明，并确认待开通权限的完整性，最后在页面底部点击 **保存**。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d34c4d4f8ac284359d8381756e1644db_k58likMeVE.png?height=1318&lazyload=true&maxWidth=600&width=2882)

4. 在 **待申请** 版本页面，点击 **设置为测试版本**，并在弹出的对话框完成测试版本的设置。


    完成配置后，版本状态会变更为 **测试中**，此时该应用已安装到对应测试企业，并可以在该测试企业中免审调用 API。
    
    ![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ff9d0637fef0aeaeeaefce0fdccc86f9_6lE3TXTciM.png?height=396&lazyload=true&maxWidth=600&width=2224)


### 步骤三：正式发布应用

确保测试联调阶段申请的权限符合业务预期后，前往 **应用发布** > **版本管理与发布** 页面，发布应用。
商店应用的正式发布分为定向上架、全量上架、非公开上架。你需要根据业务实际情况选择发布，详细介绍参见[七、发布商店应用](/document/uMzNwEjLzcDMx4yM3ATM/uYjMyUjL2IjM14iNyITN)。

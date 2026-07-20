---
document_id: '7074618159678504966'
directory_id: '6907567266541092866'
title: 三、填写应用介绍
full_path: /uMzNwEjLzcDMx4yM3ATM/ucjN3QjL3YzN04yN2cDN
breadcrumb:
- Developer Guides
- Develop Process
- Store App Development Process
- 3. Fill in app introduction
document_type: GuideDocumentType
updated_at: 2024-07-19T02:17:40Z
source_url: https://open.larksuite.com/document/uMzNwEjLzcDMx4yM3ATM/ucjN3QjL3YzN04yN2cDN
---

# 三、填写应用介绍

商店应用创建成功后，需要填写应用的基础信息及应用中心的展示信息。
:::note
相关内容修改需提交应用发版申请并审核后通过后才可生效。
:::

## （推荐）配置应用基础信息

1. 登录[开发者后台](https://open.larksuite.com/app?)，在左侧导航栏点击进入 **凭证与基础信息** 页。

2. 点击 **综合信息** 区域的编辑图标，上传 **应用图标**，填写 **管理后台主页。**
   * **应用图标**：你可以继续使用创建应用时选择的系统图标，也可以点击图标上传自定义图标。参考 [设计规范](https://bytedance.larkoffice.com/docs/doccnLPTg2Vpif0yujdAz47jsoN)。
   * **管理后台主页**：若应用有管理后台，需要提供。管理员可以在 [Lark企业管理后台](https://larksuite.com/admin) 通过该网址打开应用管理后台，进行应用内管理与设置。
   
    ![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/7307a29f954b84bd68c7f058c1445c63.png?height=806&lazyload=true&maxWidth=600&width=1992)
    
3. 点击 **国际化配置** 区域的编辑图标，点击 **添加语言**，选择需要的语言，并进行对应的国际化配置。
   * **应用名称：** 名称需具备品牌化。
   * **应用描述：** 内容清晰简短，长度均控制在 **120 个字符以内。** 若中文环境下存在英文或数字，则中文与英文或数字之间需要有半角空格；句尾不加标点符号。
   
    ![图片](https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/9d7c8d20c8e53ab0ba01fe30504fda8c.png?height=734&lazyload=true&maxWidth=600&width=1564)
    
## 填写应用中心信息

商品介绍会在应用中心进行展示，编写清晰直观的产品介绍可以更好地向用户体现应用的价值。
你可以在 [开发者后台](https://open.larksuite.com/app?) > **应用发布** > **应用中心信息** 进行填写。

:::note
* 图文物料中避免出现除Lark和该应用之外的其他企业信息。真名、电话号码等真实信息需经对方授权。
* 如果在配置应用基础信息时添加了其他语言的国际化配置，则需要在此处配置对应语言的应用展示信息。
:::

![图片](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/e658417432ca7e86c8ae6aa165d65c65_mYLGN5Y4rL.png?height=812&lazyload=true&maxWidth=650&width=1243)


:::html
<table>
  <colgroup>
    <col width="20%">
    <col width="60%">
    <col width="20%">
  </colgroup>
  <thead>
    <tr>
      <th>属性</th>
      <th>描述</th>
      <th>是否必填</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>应用分类</td>
      <td>为应用设定合适的应用分类，管理员根据应用分类查找他们需要的应用，最多可选择 <strong>3个分类</strong>。</td>
      <td>是</td>
    </tr>
    <tr>
      <td>商品说明</td>
      <td>对应用的描述，用以详细说明特性和功能。 <br><li> <strong>应用简介:</strong> 1 句话总起，简述应用的核心功能 或 能解决的业务问题</li>，句尾需要添加句号。 <br> <li><strong>功能亮点：</strong>以“-”开头的 3-5 个应用的核心亮点，并提取核心关键词放在句首，且句尾无需标点。</li>  <li><strong>使用条件：</strong>若应用不能使用Lark账号直接开通服务，需要提供使用条件，且句尾无需标点。 <br><md-alert>除应用名称外，其余介绍部分若中文描述中出现英文或数字，中文与英文或数字之间需要有半角空格。文本</md-alert> </li> </td>
      <td>是</td>
    </tr>
    <tr>
      <td>演示视频</td>
      <td>以视频形式传达应用的价值，解决目标客户的主要挑战和目标。 <br> 建议长度：15-90 秒。 <br> 建议添加操作方法帮助用户快速上手使用。 <br> 支持 MOV/M4V/MP4 格式的视频，16:9 比例，500 MB 以内，最多上传 4 个。 <br> 更多注意事项与参考案例，参考 <a href="https://bytedance.larkoffice.com/docs/doccnLPTg2Vpif0yujdAz47jsoN">设计规范</a>。</td>
      <td>否</td>
    </tr>
    <tr>
      <td>演示图片</td>
      <td>使用演示图片展示应用的主要和特色功能： <br> 以产品界面图片与描述结合的方式表达出产品的核心卖点。 <br> 体现出产品的关键功能以帮助用户快速上手使用。 <br> 支持 PNG、JPG 格式的图片，16:10 比例，1600*1000px，3MB 以内，建议 2 张以上，最多上传 8 张。 <br> 更多注意事项与参考案例，参考 <a href="https://bytedance.larkoffice.com/docs/doccnLPTg2Vpif0yujdAz47jsoN">设计规范</a>。</td>
      <td>是</td>
    </tr>
    <tr>
      <td>关键词</td>
      <td>关键词帮助用户更好地搜索应用，需提供描述应用的<strong>至少 3 个关键词，最多 10 个</strong>关键词。 <br> 前三个关键词会展示在应用中心，每个关键词需控制在 <strong>5 个中文汉字或 10 个英文字符</strong>。</td>
      <td>是</td>
    </tr>
    <tr>
      <td>操作指南</td>
      <td>使用帮助 URL。文章需保持信息层级清晰，可分为应用简介、在Lark<strong>安装应用流程、使用方法、常见问题</strong>等模块。更多注意事项与参考案例，参考 <a href="https://bytedance.larkoffice.com/docs/doccnynLrG60hKrE9A63WcHlRrf">撰写指南</a>。</td>
      <td>是</td>
    </tr>
  </tbody>
</table>
:::

## 联系方式与协议

联系方式与协议包含 **开发者信息**、**应用官网网址**、**客服联系方式** 及应用相关的 **隐私政策** 与 **服务条款**，会展示在应用中心和应用的关于页面。你可以在 [开发者后台](https://open.larksuite.com/app?) > **应用发布** > **应用中心信息** 进行编辑。

![图片](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8203488d04338c0077fa41a3fa27bc6a_N6uherlhSF.png?height=974&lazyload=true&maxWidth=650&width=1482)

:::html
<table>
  <colgroup>
    <col width="20%">
    <col width="60%">
    <col width="20%">
  </colgroup>
  <thead>
    <tr>
      <th>属性</th>
      <th>描述</th>
      <th>是否必填</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>开发者信息</td>
      <td>系统自动填充当前测试企业名称。</td>
      <td>是</td>
    </tr>
    <tr>
      <td>应用官网网址</td>
      <td>应用官网网址会在应用中心进行展示， 提供应用官网网址以便用户了解应用。</td>
      <td>是</td>
    </tr>
    <tr>
      <td>客服联系方式</td>
      <td>客服联系方式包括 <strong>客服电话、客服邮箱、Lark账官方号</strong>，三项配置需<strong>至少配置一项</strong>。 客服联系方式在应用中心和应用关于页面展示， 用户可以通过此联系方式反馈应用购买或使用的问题。</td>
      <td>是</td>
    </tr>
    <tr>
      <td>服务条款网址</td>
      <td>服务条款会在应用中心和应用关于页面进行展示。</td>
      <td>是</td>
    </tr>
    <tr>
      <td>隐私政策网址</td>
      <td>隐私政策会在应用中心和应用关于页面进行展示。</td>
      <td>是</td>
    </tr>
  </tbody>
</table>
:::

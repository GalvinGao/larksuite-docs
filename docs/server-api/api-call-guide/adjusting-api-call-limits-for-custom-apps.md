---
document_id: '7454090400672923654'
directory_id: '7262910572679708677'
title: 自建应用 API 调用量规则调整说明
full_path: /uAjLw4CM/ukTMukTMukTM/api-call-guide/api-billing
breadcrumb:
- Server API
- API Call Guide
- Adjusting API call limits for custom apps
document_type: GuideDocumentType
updated_at: 2025-03-04T05:01:26Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/api-call-guide/api-billing
---

# 自建应用 API 调用量规则调整说明

开放平台为提升服务稳定性与用户体验，自 2025 年 1 月 8 日起，将服务端 API 分为 **不计费 API**、**基础 API**、**高级 API** 三类，并根据不同的类别进行计量计费。

## 调整内容

服务端 API 分为 **不计费 API**、**基础 API**、**高级 API** 三种类型，不同类型的 API 计费规则不同。

### 不计费 API

企业自建应用调用以下业务的 API 时，不计量不计费，可免费试用。
如果不计费 API 存在历史版本，调用历史版本 API 也不计量、不计费。
- [认证及授权](/document/common-capabilities/sso/api/obtain-oauth-code)
- [事件订阅](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)
- [通讯录概述](/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/resources)
- [AI 能力](/document/uAjLw4CM/ukTMukTMukTM/document_ai-v1/jp_driving_license/recognize)

### 基础 API

基础 API 是指除不计费 API 和高级 API 之外的 API，企业自建应用调用这类 API 时会计算调用量，不同 [Lark 版本](https://www.larksuite.com/zh_cn/plans?from=footer) 的 API 调用总量不同，具体说明如下表所示。

| Lark 版本 | API 调用额度说明 |
| --- | --- |
| 标准版 | 单租户下所有企业自建应用的基础 API 调用总量上限为 **10,000 次/月**，该上限在每个自然月的 1 日刷新。<br><md-alert type="tip" icon="none"><br>标准版内，未认证企业和认证企业的自建应用基础 API 调用总量上限均为 10,000 次/月。<br></md-alert> |
| -   基础版<br>- 专业版<br>- 旗舰版 | 不限制 API 调用额度。 |


例如，某一企业所用的 Lark 版本为标准版（基础 API 调用量上限为 10,000 次/月），其中自建应用 A 本月调用基础 API 300 次、自建应用 B 本月调用基础 API 1,000 次，则该企业的自建应用调用基础 API 总量为 1,300 次，当月剩余的基础 API 调用量为 8,700 次。

### 高级 API

高级 API 是指功能逻辑较为复杂、存在一定服务成本的 API。如果企业自建应用需要调用高级 API，则需先[联系我们](https://www.larksuite.com/global/salessupport?tracking_code=701TL00000HcrRPYAZ&lang=en-US)购买高级 API 计量包，计量包内提供权益点位，调用高级 API 将消耗权益点位。不同 API 计量单位所消耗的权益点位不同，具体说明如下表所示。如有疑问可[联系客服](https://applink.larksuite.com/client/web_app/open?appId=cli_a4517c8461f8100a&mode=sidebar&channel=Boss+Openapi+Call+Limit+Paid)。

:::warning
- 计量包仅 Lark 基础版、专业版、旗舰版可以购买。如需使用请先升级至匹配版本再购买计量包，版本介绍参见 [Lark 版本](https://www.larksuite.com/zh_cn/plans?from=footer)。
- 计量包支持叠加购买，每个计量包自购买时计算有效期为 1 年。
- 随着业务升级迭代，高级 API 将会持续增加。
:::

| API 名称 | 计量单位 | 权益点位消耗 |
| --- | --- | --- |
| 识别文件中的简历信息 | 页 | 6 |
| [流式语音识别](/document/uAjLw4CM/ukTMukTMukTM/reference/ai/speech_to_text-v1/speech/stream_recognize) | 秒 | 1 |
| [基础图片识别](/document/uAjLw4CM/ukTMukTMukTM/reference/ai/optical_char_recognition-v1/image/basic_recognize) | 1,000 次 | 1,000 |


例如，调用[基础图片识别](/document/uAjLw4CM/ukTMukTMukTM/reference/ai/optical_char_recognition-v1/image/basic_recognize)接口 1,000 次，将扣除 1,000 权益点位。

## 生效范围

API 计量计费仅生效于企业自建应用。商店应用调用 API 暂无限制。

## 生效时间

- **2025 年 1 月 8 日 起可以查看 API 用量，但不管控用量**

	- Lark 标准版内，单租户下所有企业自建应用的 **基础 API** 调用总量上限调整为 **10,000 次/月**，但允许超量调用 API。
	- 所有 Lark 版本开始支持通过[管理后台](https://www.larksuite.com/admin) > **费用中心** > **权益数据** 页面查看 **API 调用次数**（仅统计基础 API 调用次数）。

- **2025 年 3 月 3 日起，基础 API 生效调用量上限**

	Lark 标准版的企业自建应用，在当月超过 **基础 API** 调用量上限后，将无法继续调用 **基础 API**，继续调用将会失败，并返回 `99991403` 错误码。

- **2025 年 4 月 1 日起，高级 API 开始消耗权益点位**

	**高级 API** 开始消耗计量包中的权益点位。当权益点位不足时，企业自建应用开发者将无法继续调用 **高级 API**，继续调用将会失败，并返回 `99991406` 错误码。

    :::warning
    **高级 API** 会根据业务实际情况逐步开放管控。
    :::
  
## 查询 API 调用明细

### 通过管理后台查看 API 调用次数

企业管理员可登录[管理后台](https://www.larksuite.com/admin)，在 **费用中心** > **权益数据** 页面查看 **API 调用次数**。

:::note
**API 调用次数** 用于统计 **基础 API** 调用量，不统计其他 API 类型。所有 Lark 版本均可查看 API 调用次数的已用量，其中，Lark 标准版的 API 调用次数总量显示为 **10,000**，其他 Lark 版本显示为 **不限**。
:::

![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/8cdf9ad8c183afc56a53d4d3a9e2d2f2_PMKULCAeGA.png?height=1408&lazyload=true&maxWidth=600&width=2882)

### 接收超量预警通知

- **基础 API**：当企业内所有自建应用的 **基础 API** 调用总量达到上限的 90%、100% 时，开放平台会向企业管理员以及自建应用的开发者推送预警通知。

| 通知方式 | 说明 |
| --- | --- |
| 管理后台 | 企业管理员登录[管理后台](https://www.larksuite.com/admin)时，在首页的 **权益预警** 区域可查看 API 调用量信息。 |
| Lark 客户端 Bot 推送 | 开放平台会通过 Lark 客户端发送卡片消息通知：<br>- **基础 API** 达到调用量上限的 90% 时，卡片消息通知示例如下。<br>![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b498fabc88ab25f2a01fce04b798f95e_pUYoEkINwE.png?height=1018&lazyload=true&maxWidth=350&width=2354)<br>- **基础 API** 达到调用量上限的 100% 时，卡片消息通知示例如下。<br>![](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ce92bb2c20bd8855ec1e64c28406d785_iqdTA2fHNc.png?height=952&lazyload=true&maxWidth=350&width=2360) |


- **高级 API**：当高级 API 计量包消耗至 90%、100% 时，开放平台会向自建应用的开发者推送预警通知，提醒开发者及时购买高级 API 计量包。此外，在每个自然月的 1 日，开发者小助手会向自建应用的开发者推送高级 API 计量包的消耗情况。

  :::warning
  暂不支持通过平台查看高级 API 计量包消耗明细，请关注 Lark 客户端 Bot 推送的预警通知。
  :::

## 提高 API 调用次数

基础 API 的调用总量限制可以通过升级 Lark 版本来解除；高级 API 可以通过购买计量包补充权益点位。

### 基础 API 升级调用总量

当企业的 **基础 API** 调用量达到上限时，企业内的自建应用将无法调用 **基础 API**，继续调用将失败，并返回以下报错信息：

- HTTP 状态码：`429`
- 错误码：`99991403`
- 错误信息：`This month's API call quota has been exceeded`

你可以联系企业管理员升级 Lark 版本，升级版本后将不限制 **基础 API** 调用量。Lark 版本介绍与升级入口，参见[Lark 版本](https://www.larksuite.com/en_us/plans?from=footer)，如有疑问可[联系客服](https://applink.larksuite.com/client/web_app/open?appId=cli_a4517c8461f8100a&mode=sidebar&channel=Boss+Openapi+Call+Limit+Paid)。

### 高级 API 购买计量包

当高级 API 计量包消耗完毕，企业内的自建应用将无法调用 **高级 API**，继续调用将失败，并返回以下报错信息：

- HTTP 状态码：`429`
- 错误码：`99991406`
- 错误信息：`Advanced API package has been depleted.`

此外，高级 API 计量包自购买日起有效期为 1 年，超时将无法使用。如需购买高级 API 计量包请[联系我们](https://www.larksuite.com/global/salessupport?tracking_code=701TL00000HcrRPYAZ&lang=en-US)，如有疑问可[联系客服](https://applink.larksuite.com/client/web_app/open?appId=cli_a4517c8461f8100a&mode=sidebar&channel=Boss+Openapi+Call+Limit+Paid)。

## 常见问题

### 调用 API 失败，返回 4xx/5xx 错误码，是否占用企业的 API 计量计费？

不占用。

### 当开放平台调整 OpenAPI 计量计费规则后，是否立即生效？

否。当月调整 API 计量计费规则在次月生效。

### 群组中添加的自定义机器人以 webhook 方式发送消息是否会占用 API 计量计费？

不会。群组内添加的[自定义机器人](/document/ukTMukTMukTM/ucTM5YjL3ETO24yNxkjN)以 webhook 方式向群组内推送消息不会占用 API 计量计费。

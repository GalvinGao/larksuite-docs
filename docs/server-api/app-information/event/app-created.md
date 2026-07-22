---
document_id: '7072608739626876933'
directory_id: '7021712771402711046'
title: 应用创建
full_path: /uAjLw4CM/ukTMukTMukTM/application-v6/application/events/created
breadcrumb:
- Server API
- App Information
- Event
- App Created
document_type: ReferenceDocumentType
updated_at: 2022-03-08T06:13:49Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/application/events/created
---

# 应用创建

当企业内有新的应用被创建时推送此事件{使用示例}(url=/api/tools/api_explore/api_explore_config?project=application&version=v6&resource=application&event=created)

:::html
<md-alert type="error">

</md-alert>
:::

:::html
<md-alert type="warn">

</md-alert>
:::

:::html
<md-alert type="tip">

</md-alert>
:::



## 事件

| 基本 |  |
| --- | --- |
| 事件类型 | application.application.created_v6 |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">订阅该事件所需的权限，开启其中任意一项权限即可订阅</md-tooltip> | <md-perm name="admin:app.info:readonly" desc="获取应用信息" support_app_types="custom" tags="">获取应用信息</md-perm> |
| 字段权限要求 | <md-alert type="tip" icon="none"><br>该接口返回体中存在下列敏感字段，仅当开启对应的权限后才会返回；如果无需获取这些字段，则不建议申请<br></md-alert><br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm> |
| 推送方式 | <md-tag mode="inline" type="push-webhook" href="/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM" >Webhook</md-tag> |




### 事件体

| 名称 | 类型 | 描述 |
| --- | --- | --- |
| <md-text type="field-name" >schema</md-text> | <md-text type="field-type" >string</md-text> | 事件模式 |
| <md-text type="field-name" >header</md-text> | <md-text type="field-type" >event_header</md-text> | 事件头 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >event_id</md-text> | <md-text type="field-type" >string</md-text> | 事件 ID |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >event_type</md-text> | <md-text type="field-type" >string</md-text> | 事件类型 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >create_time</md-text> | <md-text type="field-type" >string</md-text> | 事件创建时间戳（单位：毫秒） |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >token</md-text> | <md-text type="field-type" >string</md-text> | 事件 Token |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >app_id</md-text> | <md-text type="field-type" >string</md-text> | 应用 ID |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >tenant_key</md-text> | <md-text type="field-type" >string</md-text> | 租户 Key |
| <md-text type="field-name" >event</md-text> | <md-text type="field-type" >\-</md-text> | \- |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >operator_id</md-text> | <md-text type="field-type" >user_id</md-text> | 用户 ID |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >union_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 union id |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >user_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 user id<br>**字段权限要求**：<br><md-perm name="contact:user.employee_id:readonly" desc="获取用户 user ID" support_app_types="custom" tags="">获取用户 user ID</md-perm> |
| &emsp;&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >open_id</md-text> | <md-text type="field-type" >string</md-text> | 用户的 open id |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >app_id</md-text> | <md-text type="field-type" >string</md-text> | 应用 ID |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >name</md-text> | <md-text type="field-type" >string</md-text> | 应用名称 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >description</md-text> | <md-text type="field-type" >string</md-text> | 应用描述 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >avatar</md-text> | <md-text type="field-type" >string</md-text> | 应用图标链接 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >app_scene_type</md-text> | <md-text type="field-type" >int</md-text> | 应用类型，0: 自建应用，1: 应用商店应用 |
| &emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >primary_language</md-text> | <md-text type="field-type" >string</md-text> | 应用主语言 |




### 事件体示例

```json
{
    "schema": "2.0",
    "header": {
        "event_id": "5e3702a84e847582be8db7fb73283c02",
        "event_type": "application.application.created_v6",
        "create_time": "1608725989000",
        "token": "rvaYgkND1GOiu5MM0E1rncYC6PLtF7JV",
        "app_id": "cli_9f5343c580712544",
        "tenant_key": "2ca1d211f64f6438"
    },
    "event": {
        "operator_id": {
            "union_id": "on_8ed6aa67826108097d9ee143816345",
            "user_id": "e33ggbyz",
            "open_id": "ou_84aad35d084aa403a838cf73ee18467"
        },
        "app_id": "cli_9f5343c580712544",
        "name": "应用名称",
        "description": "应用描述",
        "avatar": "https://s1-imfile.feishucdn.com/static-resource/v1/c1f2f629-65e5-462c-be6a-bf327f971edg",
        "app_scene_type": 0,
        "primary_language": "zh_cn"
    }
}
```






### 事件订阅示例代码

事件订阅流程可参考：[事件订阅概述](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)，新手入门可参考：[教程](/document/uAjLw4CM/uMzNwEjLzcDMx4yM3ATM/develop-an-echo-bot/introduction)

:::html
<div style="margin-bottom: 4px;display: flex;column-gap: 4px;align-items: center;">
  <md-text type='field-name'>订阅方式</md-text>
  <md-tooltip>
    <ul class="md_render-table_solid md_render-table">
      <li><b>长连接方式（推荐）：</b>无需发布到公网地址，在本地开发环境中即可接收事件回调，且无需处理加解密逻辑。</li>
      <li><b>发送至开发者服务器：</b>需要提供服务器公网地址。</li>
    </ul>
  </md-tooltip>
</div>
:::

:::html
<md-code-tabs>
  <md-code-tab-group title="使用长连接接收事件">
	
    <md-code-tab-panel sdkType="golang-sdk">
package main

import (
	"context"
	"fmt"

	larkcore "github.com/larksuite/oapi-sdk-go/v3/core"
	larkevent "github.com/larksuite/oapi-sdk-go/v3/event"
	"github.com/larksuite/oapi-sdk-go/v3/event/dispatcher"
	"github.com/larksuite/oapi-sdk-go/v3/service/application/v6"
	larkws "github.com/larksuite/oapi-sdk-go/v3/ws"
)

// SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/golang-sdk-guide/preparations
func main() {
	// 注册事件 Register event
	eventHandler := dispatcher.NewEventDispatcher("", "").
		OnP2ApplicationCreatedV6(func(ctx context.Context, event *larkapplication.P2ApplicationCreatedV6) error {
			fmt.Printf("[ OnP2ApplicationCreatedV6 access ], data: %s\n", larkcore.Prettify(event))
			return nil
		})

	// 构建 client Build client
	cli := larkws.NewClient("YOUR_APP_ID", "YOUR_APP_SECRET",
		larkws.WithEventHandler(eventHandler),
		larkws.WithLogLevel(larkcore.LogLevelDebug),
	)

	// 建立长连接 Establish persistent connection
	err := cli.Start(context.Background())

	if err != nil {
		panic(err)
	}
}

    </md-code-tab-panel>

    <md-code-tab-panel sdkType="python-sdk">
# SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/python--sdk/preparations-before-development
import lark_oapi as lark


def do_p2_application_application_created_v6(data: lark.application.v6.P2ApplicationApplicationCreatedV6) -> None:
    print(f'[ do_p2_application_application_created_v6 access ], data: {lark.JSON.marshal(data, indent=4)}')

# 注册事件 Register event
event_handler = lark.EventDispatcherHandler.builder("", "") \
    .register_p2_application_application_created_v6(do_p2_application_application_created_v6) \
    .build()


def main():
    # 构建 client Build client
    cli = lark.ws.Client("APP_ID", "APP_SECRET",
                        event_handler=event_handler, log_level=lark.LogLevel.DEBUG)
    # 建立长连接 Establish persistent connection
    cli.start()

if __name__ == "__main__":
    main()

    </md-code-tab-panel>

    <md-code-tab-panel sdkType="java-sdk">

package com.example.sample;

import com.lark.oapi.core.utils.Jsons;
import com.lark.oapi.service.application.ApplicationService;
import com.lark.oapi.service.application.v6.model.P2ApplicationCreatedV6;
import com.lark.oapi.event.EventDispatcher;
import com.lark.oapi.ws.Client;

// SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/java-sdk-guide/preparations
public class Sample {
    // 注册事件 Register event
    private static final EventDispatcher EVENT_HANDLER = EventDispatcher.newBuilder("", "")
            .onP2ApplicationCreatedV6(new ApplicationService.P2ApplicationCreatedV6Handler() {
                @Override
                public void handle(P2ApplicationCreatedV6 event) throws Exception {
                    System.out.printf("[ onP2ApplicationCreatedV6 access ], data: %s\n", Jsons.DEFAULT.toJson(event.getEvent()));
                }
            })
            .build();

    public static void main(String[] args) {
        // 构建 client Build client
        Client client = new Client.Builder("APP_ID", "APP_SECRET")
                .eventHandler(EVENT_HANDLER)
                .build();
        // 建立长连接 Establish persistent connection
        client.start();
    }
}
    </md-code-tab-panel>

    <md-code-tab-panel sdkType="nodejs-sdk">
// SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/nodejs-sdk/preparation-before-development
import * as Lark from '@larksuiteoapi/node-sdk';
const baseConfig = {
    appId: 'APP_ID',
    appSecret: 'APP_SECRET'
}
// 构建 client Build client
const wsClient = new Lark.WSClient(baseConfig);
// 建立长连接 Establish persistent connection
wsClient.start({
    // 注册事件 Register event
    eventDispatcher: new Lark.EventDispatcher({}).register({
        'application.application.created_v6': async (data) => {
            console.log(data);
        }
    })
});
    </md-code-tab-panel>

  </md-code-tab-group>
  <md-code-tab-group title="将事件推送至开发者服务器">
	
    <md-code-tab-panel sdkType="golang-sdk">
package main

import (
	"context"
	"fmt"
	"net/http"

	larkcore "github.com/larksuite/oapi-sdk-go/v3/core"
	"github.com/larksuite/oapi-sdk-go/v3/core/httpserverext"
	larkevent "github.com/larksuite/oapi-sdk-go/v3/event"
	"github.com/larksuite/oapi-sdk-go/v3/event/dispatcher"
	"github.com/larksuite/oapi-sdk-go/v3/service/application/v6"
)

// SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/golang-sdk-guide/preparations
func main() {
	// 注册事件 Register event
	eventHandler := dispatcher.NewEventDispatcher("", "").
		OnP2ApplicationCreatedV6(func(ctx context.Context, event *larkapplication.P2ApplicationCreatedV6) error {
			fmt.Printf("[ OnP2ApplicationCreatedV6 access ], data: %s\n", larkcore.Prettify(event))
			return nil
		})

	// 创建路由处理器 Create route handler
	http.HandleFunc("/webhook/event", httpserverext.NewEventHandlerFunc(handler, larkevent.WithLogLevel(larkcore.LogLevelDebug)))

	err := http.ListenAndServe(":7777", nil)

	if err != nil {
		panic(err)
	}
}

    </md-code-tab-panel>

    <md-code-tab-panel sdkType="python-sdk">
# SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/python--sdk/preparations-before-development
from flask import Flask
from lark_oapi.adapter.flask import *
import lark_oapi as lark

app = Flask(__name__)


def do_p2_application_application_created_v6(data: lark.application.v6.P2ApplicationApplicationCreatedV6) -> None:
    print(f'[ do_p2_application_application_created_v6 access ], data: {lark.JSON.marshal(data, indent=4)}')

# 注册事件 Register event
event_handler = lark.EventDispatcherHandler.builder("", "") \
    .register_p2_application_application_created_v6(do_p2_application_application_created_v6) \
    .build()


# 创建路由处理器 Create route handler
@app.route("/webhook/event", methods=["POST"])
def event():
    resp = event_handler.do(parse_req())
    return parse_resp(resp)

if __name__ == "__main__":
    app.run(port=7777)

    </md-code-tab-panel>

    <md-code-tab-panel sdkType="java-sdk">

package com.lark.oapi.sample.event;

import com.lark.oapi.core.utils.Jsons;
import com.lark.oapi.service.application.ApplicationService;
import com.lark.oapi.service.application.v6.model.P2ApplicationCreatedV6;
import com.lark.oapi.sdk.servlet.ext.ServletAdapter;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

// SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/java-sdk-guide/preparations
@RestController
public class EventController {

    // 注册事件 Register event
    private static final EventDispatcher EVENT_HANDLER = EventDispatcher.newBuilder("verificationToken", "encryptKey")
            .onP2ApplicationCreatedV6(new ApplicationService.P2ApplicationCreatedV6Handler() {
                @Override
                public void handle(P2ApplicationCreatedV6 event) throws Exception {
                    System.out.printf("[ onP2ApplicationCreatedV6 access ], data: %s\n", Jsons.DEFAULT.toJson(event.getEvent()));
                }
            })
            .build();

    // 注入 ServletAdapter 实例 Inject ServletAdapter instance
    @Autowired
    private ServletAdapter servletAdapter;

    // 创建路由处理器 Create route handler
    @RequestMapping("/webhook/event")
    public void event(HttpServletRequest request, HttpServletResponse response)
            throws Throwable {
        // 回调扩展包提供的事件回调处理器 Callback handler provided by the extension package
        servletAdapter.handleEvent(request, response, EVENT_DISPATCHER);
    }
}
    </md-code-tab-panel>

    <md-code-tab-panel sdkType="nodejs-sdk">
// SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/nodejs-sdk/preparation-before-development
import http from 'http';
import * as lark from '@larksuiteoapi/node-sdk';

// 注册事件 Register event
const eventDispatcher = new lark.EventDispatcher({
    encryptKey: '',
    verificationToken: '',
}).register({
    'application.application.created_v6': async (data) => {
        console.log(data);
        return 'success';
    },
});

const server = http.createServer();
// 创建路由处理器 Create route handler
server.on('request', lark.adaptDefault('/webhook/event', eventDispatcher));
server.listen(3000);
    </md-code-tab-panel>

  </md-code-tab-group>
</md-code-tabs>
:::

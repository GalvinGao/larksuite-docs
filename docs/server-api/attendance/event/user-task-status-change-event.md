---
document_id: '7026247042947284998'
directory_id: '7021712771402694662'
title: 用户任务状态变更事件
full_path: /uAjLw4CM/ukTMukTMukTM/Attendance/event/user-task-status-change-event
breadcrumb:
- Server API
- Attendance
- Event
- User task status change event
document_type: GuideDocumentType
updated_at: 2022-03-03T15:54:33Z
source_url: https://open.larksuite.com/document/uAjLw4CM/ukTMukTMukTM/Attendance/event/user-task-status-change-event
---

# 用户任务状态变更事件
:::html
<md-alert type="tip">
了解事件订阅的使用场景和配置流程，请点击查看 [事件订阅概述](/document/ukTMukTMukTM/uUTNz4SN1MjL1UzM)
</md-alert>
:::
**事件**

当用户任务变更后，推送该用户的任务状态变更消息。


**事件体**
|名称|类型|描述|
|---|---|---|
|schema|string|事件模式|
|header|event_header|事件头|
|&emsp;∟event_id|string|事件 ID|
|&emsp;∟event_type|string|事件类型|
|&emsp;∟create_time|string|事件创建时间戳（单位：毫秒）|
|&emsp;∟token|string|事件 Token|
|&emsp;∟app_id|string|应用 ID|
|&emsp;∟tenant_key|string|租户 Key|
|event|-|事件体|
|&emsp;∟employee_id|string|员工 ID|
|&emsp;∟employee_no|string|员工工号|
|&emsp;∟group_id|string|考勤组 ID|
|&emsp;∟shift_id|string|班次 ID|
|&emsp;∟date|int|日期|
|&emsp;∟status_changes|list|状态变更数组|
|&emsp;&emsp;∟before_status|string|变更前打卡结果，值为：【NoNeedCheck（无需打卡），SystemCheck（系统打卡），Normal（正常），Early（早退），Late（迟到），Lack（缺卡）】|
|&emsp;&emsp;∟current_status|string|变更后打卡结果，值为：【NoNeedCheck（无需打卡），SystemCheck（系统打卡），Normal（正常），Early（早退），Late（迟到），Lack（缺卡）】|
|&emsp;&emsp;∟before_supplement|string|变更前结果补充，值为：【None（无），ManagerModification（管理员修改），CardReplacement（补卡通过），ShiftChange（换班），Travel（出差），Leave（请假），GoOut（外出），CardReplacementApplication（补卡申请中），FieldPunch（外勤打卡）】|
|&emsp;&emsp;∟current_supplement|string|变更后打卡结果补充，值为：【None（无），ManagerModification（管理员修改），CardReplacement（补卡通过），ShiftChange（换班），Travel（出差），Leave（请假），GoOut（外出），CardReplacementApplication（补卡申请中），FieldPunch（外勤打卡）】|
|&emsp;&emsp;∟work_type|string|上下班状态变更，值为：【on（上班），off（下班）】
|&emsp;&emsp;∟index|string|任务中的第几次上下班
**事件示例**
```json
{
  "schema": "2.0",
  "header": {
    "event_id": "2a0b662d75f87508a016a8da8b225a46",
    "token": "drIibiox5ZEZl5UvuRL3Uf3LNwD0fB6e",
    "create_time": "1615381917559",
    "event_type": "attendance.user_task.updated_v1",
    "tenant_key": "2fce678eb60d1651",
    "app_id": "cli_a0c27934a1f8100b"
  },
  "event": {
    "date": 20210310,
    "employee_id": "2b68933a",
    "employee_no": "",
    "group_id": "6935645428507557915",
    "shift_id": "6938006252085739521",
    "status_changes": [
      {
        "before_status": "Normal",
        "before_supplement": "None",
        "current_status": "Late",
        "current_supplement": "None",
        "index": 1,
        "work_type": "on"
      }
    ],
    "task_id": "6936733176245026817",
    "time_zone": "Asia/Shanghai"
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
	"github.com/larksuite/oapi-sdk-go/v3/service/attendance/v1"
	larkws "github.com/larksuite/oapi-sdk-go/v3/ws"
)

// SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/golang-sdk-guide/preparations
func main() {
	// 注册事件 Register event
	eventHandler := dispatcher.NewEventDispatcher("", "").
		OnP2UserTaskUpdatedV1(func(ctx context.Context, event *larkattendance.P2UserTaskUpdatedV1) error {
			fmt.Printf("[ OnP2UserTaskUpdatedV1 access ], data: %s\n", larkcore.Prettify(event))
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


def do_p2_attendance_user_task_updated_v1(data: lark.attendance.v1.P2AttendanceUserTaskUpdatedV1) -> None:
    print(f'[ do_p2_attendance_user_task_updated_v1 access ], data: {lark.JSON.marshal(data, indent=4)}')

# 注册事件 Register event
event_handler = lark.EventDispatcherHandler.builder("", "") \
    .register_p2_attendance_user_task_updated_v1(do_p2_attendance_user_task_updated_v1) \
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
import com.lark.oapi.service.attendance.AttendanceService;
import com.lark.oapi.service.attendance.v1.model.P2UserTaskUpdatedV1;
import com.lark.oapi.event.EventDispatcher;
import com.lark.oapi.ws.Client;

// SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/java-sdk-guide/preparations
public class Sample {
    // 注册事件 Register event
    private static final EventDispatcher EVENT_HANDLER = EventDispatcher.newBuilder("", "")
            .onP2UserTaskUpdatedV1(new AttendanceService.P2UserTaskUpdatedV1Handler() {
                @Override
                public void handle(P2UserTaskUpdatedV1 event) throws Exception {
                    System.out.printf("[ onP2UserTaskUpdatedV1 access ], data: %s\n", Jsons.DEFAULT.toJson(event.getEvent()));
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
        'attendance.user_task.updated_v1': async (data) => {
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
	"github.com/larksuite/oapi-sdk-go/v3/service/attendance/v1"
)

// SDK 使用说明 SDK user guide：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/server-side-sdk/golang-sdk-guide/preparations
func main() {
	// 注册事件 Register event
	eventHandler := dispatcher.NewEventDispatcher("", "").
		OnP2UserTaskUpdatedV1(func(ctx context.Context, event *larkattendance.P2UserTaskUpdatedV1) error {
			fmt.Printf("[ OnP2UserTaskUpdatedV1 access ], data: %s\n", larkcore.Prettify(event))
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


def do_p2_attendance_user_task_updated_v1(data: lark.attendance.v1.P2AttendanceUserTaskUpdatedV1) -> None:
    print(f'[ do_p2_attendance_user_task_updated_v1 access ], data: {lark.JSON.marshal(data, indent=4)}')

# 注册事件 Register event
event_handler = lark.EventDispatcherHandler.builder("", "") \
    .register_p2_attendance_user_task_updated_v1(do_p2_attendance_user_task_updated_v1) \
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
import com.lark.oapi.service.attendance.AttendanceService;
import com.lark.oapi.service.attendance.v1.model.P2UserTaskUpdatedV1;
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
            .onP2UserTaskUpdatedV1(new AttendanceService.P2UserTaskUpdatedV1Handler() {
                @Override
                public void handle(P2UserTaskUpdatedV1 event) throws Exception {
                    System.out.printf("[ onP2UserTaskUpdatedV1 access ], data: %s\n", Jsons.DEFAULT.toJson(event.getEvent()));
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
    'attendance.user_task.updated_v1': async (data) => {
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

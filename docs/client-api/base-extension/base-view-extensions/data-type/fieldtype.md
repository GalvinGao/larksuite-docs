---
document_id: '7260082411118755845'
directory_id: '7258197168736714758'
title: FieldType
full_path: /uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/fieldtype
breadcrumb:
- Client API
- Base Extension
- Base View Extensions
- Data Type
- FieldType
document_type: GuideDocumentType
updated_at: 2024-01-05T03:44:27Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/base-extensions/base-view-extensions/data-type/fieldtype
---

# FieldType
字段类型。
```js

 enum FieldType {
    Text = 1, // 多行文本
    Number = 2, // 数字
    SingleSelect = 3, // 单选
    MultiSelect = 4, // 多选
    DateTime = 5, // 日期
    Checkbox = 7, // 复选框
    User = 11, // 人员
    Phone = 13, // 电话号码
    Url = 15, // 超链接
    Attachment = 17, // 附件
    SingleLink = 18, // 单向关联
    Lookup = 19, // 查找引用
    Formula = 20, // 公式
    DuplexLink = 21, // 双向关联
    Location = 22, // 地理位置
    GroupChat = 23, // 群组

    CreatedTime = 1001, // 创建时间
    ModifiedTime = 1002, // 最后更新时间
    CreatedUser = 1003, // 创建人
    ModifiedUser = 1004, // 修改人
    AutoNumber = 1005, // 自动编号
    Barcode = 99001, // 条码
    Progress = 99002, // 进度
    Currency = 99003, // 货币
    Rating = 99004 // 评分
}

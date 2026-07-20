---
document_id: '6967331158355492870'
directory_id: '6907567269107515394'
title: 云文档请求错误码列表
full_path: /ukTMukTMukTM/uIzM0UjLyMDN14iMzQTN
breadcrumb:
- Server API
- Docs
- Appendix
- Error Codes
document_type: GuideDocumentType
updated_at: 2021-05-28T13:23:18Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uIzM0UjLyMDN14iMzQTN
---

# 云文档错误码

## sheet open api 错误码 
|name|	 err code         | msg           | 
|  --------- | --------- | --------------- | 
WrongRequestJson|	90201|	请求体不是一个 json
WrongRange|	90202|	请求中range格式有误
Fail|	90203|	不是预期内的 fail
WrongRequestBody|	90204|	请求体有误
InvalidUsers|	90205|	非法的 user
EmptySheetId|	90206|	sheeId 为空
EmptySheetTitle|	90207|	sheet 名称为空
SameSheetIdOrTitle|	90208|	请求中有相同的 sheetId 或 title
ExistSheetId|	90209|	sheetId 已经存在
ExistSheetTitle|	90210|	sheet title 已经存在
WrongSheetId|	90211|	错误的 sheetId
WrongRowOrCol|	90212|	非法的行列
PermissionFail|	90213|	没有文件的权限 forbidden
SpreadSheetNotFound|	90214|	SpreadSheet没有找到
SheetIdNotFound|	90215|	sheetId 没有找到
EmptyValue|	90216|	请求中有空值
TooManyRequest|	90217|	请求太频繁
LockedCell|90218|单元格被保护
CellExcess|90219|单元格数量超过100万限制
TooLargeResponse|90221|需要返回的数据过大
TooLargeCell|90222|单元格内容超过5万字符
ColIdNotFound|90223|ColId没有设置
RowIdNotFound|90224|RowId没有设置
NotLinkSpreadSheet|90225|未关联ISV
ExcessLimit|90226|超出限制
TooLargeRequest|90227|请求过大
ImportProcessing|90228|导入中
WrongURLParam|90229|URL参数错误
|InternalError|	95201—95209|	内部错误，详情请咨询客服	
|DefaultCode|95299|其他错误，详情请咨询客服


## apigateway open api 错误码
|name|	 err code         | msg           | 
|  --------- | --------- | --------------- | 
FAILED|	96401|	失败
TIMEOUT|	96402|	超时
PROCESSING|	96403|	请求正在处理中
PARAMERR|	91401|	参数出现错误
NOTEXIST|	91402|	未找到
FORBIDDEN|	91403|	没有权限
LOGIN_REQUIRED|	91404|	需要登录

## explorer 错误码 
|name|	 err code         | msg           | 
|  --------- | --------- | --------------- | 
FAILED|	91201|	失败
PARAMERR|91202|参数错误
NOTEXIST|91203|未找到
FORBIDDEN|91204|没有权限
DELETED|91205|数据已删除
OUT_OF_LIMIT|91206|超过限制
DUPLICATE|91207|重复记录
REVIEW|91208|内容审查不通过
LOCK|96201|内部错误
RECOVER|96202|内部错误

## permission open api 错误码 
|name|	 err code         | msg           | 
|  --------- | --------- | --------------- | 
OPEN_CODE_PARAM_ERROR|	91001|	参数错误
OPEN_CODE_FORBIDDEN|91002|没有权限
OPEN_CODE_INVALID_OPERATION|91003|操作异常
OPEN_CODE_USER_NO_SHARE_PERM|91004|用户没有共享权限
OPEN_CODE_INTERNAL_ERROR|96001|内部错误

## comment open api 错误码 
|name|	 err code         | msg           | 
|  --------- | --------- | --------------- | 
FAILED|	1069301|操作失败
PARAM_ERROR|1069302|参数错误
FORBIDDEN|1069303|没有权限
DOCS_HAD_BEEN_DELETED|1069304|云文档已删除
DOCS_NOT_EXIST|1069305|云文档不存在
CONTENT_REVIEW_NOT_PASS|1069306|评论内容审核不通过
NOT_EXIST|1069307|不存在
EXCEEDED_LIMIT|1069308|超过数量上限限制
INTERNAL_ERROR|1069399|内部错误

## comment open api 错误码 （历史版本）
|name|	 err code         | msg           | 
|  --------- | --------- | --------------- | 
FAILED|	90301|操作失败
PARAM_ERROR|90302|参数错误
FORBIDDEN|90303|没有权限
META_DELETED|90304|文件已删除
META_NOT_EXIST|90305|文件不存在
REVIEW_NOT_PASS|90306|评论内容审核不通过
INTERNAL_ERROR|90399|内部错误

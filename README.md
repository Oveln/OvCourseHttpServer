# OvCourseHttpServer

基于Rust-Actix的教师、课程管理系统
使用Postgresql进行数据存储
通过HttpAPI进行访问控制

# 对象类型

## 课程
- id：课程ID，自动生成，全局唯一
- teacher_id：教师ID
- name：课程名，非空
- time：创建时间
- description：描述
- format：格式
- structure：课程结构
- duration：周期
- price：学分
- language：语言
- level：等级

## 教师
- id：教师ID，自动生成，全局唯一
- name：教师名，非空
- picture_url：教师图片URL地址，非空
- profile：教师介绍，非空

## 注意

上述非空字段在创建该对象时必须填写

# 开放接口

- http://address/health
  - GET 健康检查
- http://address/courses/
  - POST 传入Json格式的课程信息
- http://address/courses/{teacher_id}
  - GET 查看该教师的所有课程
- http://address/courses/{teacher_id}/{course_id}
  - GET 查看该教师所教的对应ID的课程信息
  - PUT 更新该教师所教的对应ID的课程信息
  - DELETE 删除对应课程
- http://address/teacher/
  - GET 获取所有教师信息
  - POST 传入Json格式的教师信息
- http://address/teacher/{teacher_id}
  - GET 获取该教师信息
  - PUT 更新该教师信息
  - DELETE 删除该教师信息
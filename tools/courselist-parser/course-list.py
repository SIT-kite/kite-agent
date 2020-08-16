'''
    SecondCourse Crawler for testing and importing.
    As part of [kite-agent](https://github.com/sunnysab/kite-crawler) package.

    sunnysab (C) 2020 All right reserved.
    July 21, 2020

'''

import requests
import re
import json
from bs4 import BeautifulSoup
from collections import namedtuple

Course = namedtuple('Course', 
    [
        'id',            # 课程序号
        'name',          # 课程名称
        'code',          # 课程代码
        'type',          # 课程类型
        'credit',        # 学分
        'teacher',       # 授课老师
        'time_array',    # 上课时间
        'place',         # 上课地点
        'campus',        # 校区
        'plan_count',    # 计划人数
        'selected_count',# 已选人数
        'arranged_class',# 配课班
        'note',          # 备注
    ])

CourseTime = namedtuple('CourseTime',
    [
        'week_range',    # tuple, 如 (1, 18) 表示 第 1..18 周
        'type',          # 0 每周, 1 单周, 2 双周
        'day_index',     # tuple, 周 1..7
        'day_range',     # tuple, 第 1..12 节
    ])


'''
    Parse range string, for example, '1-20' -> (1, 20), '3' -> (3, 3)
'''
def process_range_string(range_string: str) -> (int, int):
    l = range_string.split('-')
    if len(l) == 2:
        return int(l[0]), int(l[1])
    else:
        return int(l[0]), int(l[0])

def process_place_string(place_string: str) -> list:
    # return place_string.split(',')
    return '{' + place_string + '}'

def process_teacher_string(teacher_string: str) -> list:
    # return teacher_string.split(',')
    return '{' + teacher_string + '}'

def process_class_string(class_string: str) -> list:
    # return class_string.split(', ')
    return '{' + class_string + '}'

'''
    Parse time string, return a list of CourseTime objects. For example:
    第4-11周,周1,第1-2节;  -> (4, 11), '', 1, (1, 2)
'''
def process_time_string(time_string: str) -> list:
    time_array = time_string.split(';')
    result: list = []

    for each_time_arrangement in time_array:
        for item in re.findall(r'第(\d+(?:-\d+)?)周([*]{0,2}),周(\d),第(\d+(?:-\d+)?)节', each_time_arrangement):
            week_range = process_range_string(item[0])
            class_range = process_range_string(item[3])

            result.append({
                'week_range' : week_range,
                'type': len(item[1]),
                'day_index': int(item[2]),
                'day_range': class_range
            })
    return json.dumps(result).replace(' ', '')


if __name__ == '__main__':

    # Load course list page.
    html = open("2020A.html", encoding="GBK")
    page = BeautifulSoup(html, "html5lib")

    # Select each course line.
    each_course_html = page.select("table tr[bgcolor=\"#efefef\"]")
    course_list: list = []

    for cols in each_course_html:
        # Convert each column to string and trim whites.
        # Profermance tip: Redundant strip() and string copy may cost CPU calculation.
        cols = [x.text.strip() for x in cols.select('td')]
        # Convert to Course obejct.
        course = Course(
            id = cols[0],
            name = cols[1],
            code = cols[2],
            type = cols[3],
            credit = float(cols[4]),
            teacher = process_teacher_string(cols[5]),
            time_array = process_time_string(cols[6]),
            place = process_place_string(cols[7]),
            campus = cols[8],
            plan_count = int(cols[9]),
            selected_count = int(cols[10]),
            arranged_class = process_class_string(cols[12].replace('\n', '').replace(' ', '')),
            note = cols[13],
        )
        course_list.append(course)

    with open('Course.csv', 'a+', encoding='utf-8') as f:
        for c in course_list:
            f.write('INSERT INTO course.course_list VALUES(\'2020A\',\'{}\',\'{}\',\'{}\',\'{}\',{},\'{}\',\'{}\',\'{}\',{},{},\'{}\',\'{}\',\'{}\'::jsonb);\n'.format( \
                c.code, c.name, c.id, c.type, c.credit, c.teacher, \
                c.place, c.campus, c.plan_count, c.selected_count, c.arranged_class, c.note, c.time_array))


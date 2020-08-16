'''
    SecondCourse Crawler for testing and importing.
    As part of [kite-agent](https://github.com/sunnysab/kite-crawler) package.

    sunnysab (C) 2020 All right reserved.
    July 28, 2020

'''

import re
from bs4 import BeautifulSoup
from collections import namedtuple

PlannedCourse = namedtuple('PlannedCourse',
    [
        'code',          # 课程代码
        'name',          # 课程名称
        'has_test',      # 是否考试, bool
        'credit',        # 学分, float
        'theory_hour',   # 理论课 课时数
        'practice_hour', # 实践课 课时数
        'department',    # 开课院系代码
        'term',          # 学期 
                         # 学期编号
    ])

def getCourseList(text: str) -> []:
    html_page = BeautifulSoup(text, 'html5lib')
    result = []

    courses = html_page.find_all(name='tr', id=re.compile('\d{5}'))
    for course in courses:
        cols = [str(x.get_text().strip()) for x in course.find_all('td')]
        code = cols[1]
        name = cols[3]
        has_test = cols[4] != ''
        credit = cols[5]
        theory_hour = int(cols[6])
        practice_hour = int(cols[7])
        department = cols[18]
        term = ''
        for i in range(8, 18):
            if cols[i] != '':
                term = i - 7
                break
        result.append(PlannedCourse(code=code, name=name, has_test=has_test, credit=credit, \
            theory_hour=theory_hour, practice_hour=practice_hour, \
            department=department, term=term)) 
    return result

def getMajorList(text: str) -> []:
    html_page = BeautifulSoup(text, 'html5lib')
    result = []

    '''
        Add all major sort codes and description.
    '''
    html_major_sort = html_page.find_all(name='option', attrs={'value': re.compile(r'[A-Z][\d+]{4}$')})
    # ('B1508', 铁道工程')
    major_sort = [tuple(x.get_text().split('\xa0')) for x in html_major_sort]
    # ['B150801', 'B150802' ... ]
    sort_codes = [x[0] for x in major_sort]
    # Add to result
    result.extend([(a, '', c) for a, c in major_sort])

    '''
        Add all majors and find their sorts.
    '''
    html_majors = html_page.find_all(name='option', attrs={'value': re.compile(r'[A-Z][A-Z\d+]{6}$')})
    for major in html_majors:
        major_id, detail = tuple(major.get_text().split('\xa0'))
        if major_id[:5] in sort_codes:
            sort = major_id[:5]
            result.append((sort, major_id, detail))
        else:
            sort = major_id
            result.append((sort, '', detail))
            result.append((sort, major_id, detail))

    return result

if __name__ == '__main__':
    with open('教学计划查询页面.html', encoding='utf-8') as f:
        text = f.read()

    with open('majors.csv', 'a+', encoding='utf-8') as f:
        l = getMajorList(text)

        for i in l:
            f.write(','.join(i) + '\n')

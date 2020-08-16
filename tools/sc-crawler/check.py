'''
    SecondCourse Crawler for testing and importing.
    As part of [kite-agent](https://github.com/sunnysab/kite-crawler) package.

    sunnysab (C) 2020 All right reserved.
    July 14, 2020

'''

'''
    Some lines have more than 11 commas which means the publisher had written comma in description.
    Treat them by converting ',' (in ASCII) to '，'(in Unicode) in description.
'''
source = open('activities.txt', 'r', encoding='utf-8').readlines()
normal = open('activities_processed.txt', 'w+', encoding='utf-8')

for line in source:
    columns = [column.strip() for column in line.split(',')]

    if len(columns) == 12:
        normal.write(','.join(columns) + '\n')
    else:
        normal.write(','.join(columns[:11]))
        normal.write(',{}\n'.format('，'.join(columns[:11])))

normal.close()

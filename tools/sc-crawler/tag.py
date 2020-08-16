'''
    SecondCourse Crawler for testing and importing.
    As part of [kite-agent](https://github.com/sunnysab/kite-crawler) package.

    sunnysab (C) 2020 All right reserved.
    July 15, 2020

'''

import psycopg2

conn = psycopg2.connect(host="host", port=1000, database="db", user="pg", password="pd")
cur = conn.cursor()

cur.execute('SELECT * FROM events.tags;')
tags = cur.fetchall()
print('共获取到了 ', len(tags), ' 条标签')

activities = open('output.csv', 'r', encoding='utf-8').readlines()
for item in activities:
    columns = item.split(',')
    id = columns[0]
    title = columns[1]
    applied_tag = []
    hide = False

    for tag in tags:
        if tag[1] in title:
            applied_tag.append(tag[1])
            hide = hide or tag[2]

    sql = 'UPDATE events.sc_events SET hide = %s, tag = %s WHERE activity_id = %s'
    cur.execute(sql, (hide, applied_tag, id))
    conn.commit()

cur.close()
conn.close()

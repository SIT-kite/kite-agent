"""
    更新电费数据
    本文件作为 kite-agent (https://github.com/sunnysab/kite-agent) 的一部分发布，仅供学习和交流使用。

    sunnysab (C) 2020
    i@sunnysab.cn
"""

import requests
import json
import psycopg2
import datetime

# Configuration file
import agent_config as config

# SQL statement for inserting data
INSERT_SQL = \
    '''INSERT INTO dormitory.balance (room, base_balance, elec_balance, total_balance) VALUES (%s, %s, %s, %s);'''
# SQL statement for querying stored rooms
QUERY_ROOM_SQL = \
    '''SELECT name FROM dormitory.rooms;'''

def get_all_room_balance():
    """
    Fetch all rooms' balance from electricity recharge page.
    :return: A json object, with the format consistent with the original return.
            The format is like:
            '{"RoomName":"1010101","BaseBalance":"-0.0610","ElecBalance":"0.0000","Balance":"-0.0610"}'
    """
    post_headers = {
        'User-Agent': config.USER_AGENT_STRING,
        'Cookie': 'FK_Dept=B1101',
        'Content-Type': 'application/x-www-form-urlencoded'
    }

    try:
        r = requests.post(config.ELECTRICITY_BALANCE_PAGE, headers=post_headers,
                          data='SQL=select * from sys_room_balance', timeout=20)
        # Return a empty json if the r.text is empty.
        # However, if SQL error occurs, the r.text will be a error message, and json.loads() will throw an exception.
        return json.loads(r.text)

    except requests.RequestException as e:
        print('Network error for ' + str(e))
        return json.loads("")

    except json.JSONDecodeError:
        # Ignore this warning, the requests error occurs before json decoder error forever.
        print('Error occurs: ', r.text)
        return json.loads("")



def convert_all_room_balance(original_list: list):
    """
    Convert the balance string to float
    """
    return [(i['RoomName'], i['BaseBalance'], i['ElecBalance'], i['Balance'])
            for i in original_list]


def generate_logfile_name():
    current_time = datetime.datetime.now()

    return current_time.strftime('%Y%m%d %H%M%S.csv')


def save_room_log(balance_list: list, logfile: str):
    with open(logfile, 'w+', encoding='GBK') as f:
        f.write('room,base_balance,elec_balance,balance\n')
        for line in balance_list:
            f.write(','.join(line) + '\n')
    # End of with


def pull_valid_rooms(cursor):
    """
    Pull valid rooms from postgresql database.
    :return: A set of string for rooms.
    """
    cursor.execute(QUERY_ROOM_SQL)
    rows = cursor.fetchall()
    return set([row[0] for row in rows])


if __name__ == '__main__':
    # Get balance list and save to file for backing up.
    g_balance_list = convert_all_room_balance(get_all_room_balance())
    save_room_log(g_balance_list, generate_logfile_name())

    # Connect to database
    conn = psycopg2.connect(database=config.DATABASE_NAME, user=config.DATABASE_USER,
                            password=config.DATABASE_PASSWD, host=config.DATABASE_HOST, port=config.DATABASE_PORT)
    # Get cursor
    cur = conn.cursor()
    # Get rooms from database and filter the room balance data. Remove the virtual and dead rooms in them.
    rooms = pull_valid_rooms(cur)
    g_balance_list = list(filter(lambda x: x[0] in rooms, g_balance_list))

    cur.executemany(INSERT_SQL, g_balance_list)

    # Commit to db and close the connection
    conn.commit()
    conn.close()

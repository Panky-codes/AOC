from collections import defaultdict
from collections import deque


def init_map(ip : list):
    map = defaultdict(lambda: deque(maxlen=2))
    for (index, val) in enumerate(ip):
        map[val].append(index + 1)
    return map


def calc_ans(ip : list, len1):

    map : dict = init_map(ip)
    prev_val = ip[-1]
    for i in range(len(map), len1):
        curr_val = prev_val
        if curr_val in map:
            len_list = len(map[curr_val])
            if len_list == 1 or len_list == 0:
                prev_val = 0
                map[0].append(i + 1)
            else:
                diff = map[curr_val][-1] - map[curr_val][-2]
                prev_val = diff
                map[diff].append(i + 1)
        else:
            prev_val = 0
            map[0].append(i + 1)

    print ("{}".format(prev_val))


def main():
    # input_list = [0,3,6]
    input_list = [14, 8, 16, 0, 1, 17]
    calc_ans(input_list, 30000000)
    # calc_ans(input_list, 2020)


if __name__ == "__main__":
    main()
